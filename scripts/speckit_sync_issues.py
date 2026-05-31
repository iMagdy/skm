#!/usr/bin/env python3
"""Synchronize Speckit story/task files with GitHub issues and a Project."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TASK_RE = re.compile(
    r"^- \[(?P<done>[ xX])\] (?P<id>T\d{3})(?: \[P\])?(?: \[(?P<story>US\d+)\])? (?P<text>.+)$"
)
CHECKBOX_RE = re.compile(r"^- \[(?P<done>[ xX])\] (?P<id>T\d{3})\b")
STORY_RE = re.compile(r"^## Phase \d+: User Story (?P<num>\d+) - (?P<rest>.+)$")
PRIORITY_RE = re.compile(r"\(Priority: (?P<priority>P\d+)\)")


@dataclass
class Task:
    task_id: str
    story: str
    text: str
    done: bool


@dataclass
class Story:
    key: str
    title: str
    priority: str
    tasks: list[Task]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--feature-dir", type=Path, default=None)
    parser.add_argument("--repo", default="iMagdy/ktesio")
    parser.add_argument("--project-owner", default="iMagdy")
    parser.add_argument("--project-title", default="Ktesio")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    feature_dir = resolve_feature_dir(args.feature_dir)
    stories = parse_feature(feature_dir)

    if args.dry_run:
        print(
            json.dumps(
                {
                    "feature": feature_dir.name,
                    "repo": args.repo,
                    "project": {
                        "owner": args.project_owner,
                        "title": args.project_title,
                    },
                    "groups": {
                        key: {
                            "title": story.title,
                            "priority": story.priority,
                            "tasks": [task.task_id for task in story.tasks],
                        }
                        for key, story in stories.items()
                    },
                },
                indent=2,
            )
        )
        return 0

    issue_map_path = feature_dir / "issue-map.json"
    issue_map = load_issue_map(issue_map_path)

    ensure_gh_ready()
    ensure_repo_matches(args.repo)

    pulled = pull_checked_tasks_from_github(feature_dir, stories, issue_map)
    if pulled:
        print(f"Pulled {pulled} checked task update(s) from GitHub issues")
        stories = parse_feature(feature_dir)

    ensure_labels(feature_dir.name)
    project_number = find_project_number(args.project_owner, args.project_title)
    issue_map.update(
        {
            "feature": feature_dir.name,
            "repo": args.repo,
            "project": {
                "owner": args.project_owner,
                "title": args.project_title,
                "number": project_number,
            },
            "updated_at": datetime.now(timezone.utc).isoformat(),
            "issues": issue_map.get("issues", {}),
            "task_to_issue": {},
        }
    )
    issue_map.setdefault("created_at", issue_map["updated_at"])

    for key, story in stories.items():
        issue_info = issue_map["issues"].get(key, {})
        title = issue_title(feature_dir.name, key, story)
        body = issue_body(feature_dir, key, story, args.project_title)
        labels = ["speckit", f"feature:{feature_dir.name}"]
        labels.append("infrastructure" if key == "shared" else "story")

        if issue_info.get("number"):
            number = int(issue_info["number"])
            gh("issue", "edit", str(number), "--title", title, "--body", body)
            for label in labels:
                gh("issue", "edit", str(number), "--add-label", label, check=False)
            url = gh("issue", "view", str(number), "--json", "url", "-q", ".url").strip()
        else:
            url = gh(
                "issue",
                "create",
                "--title",
                title,
                "--body",
                body,
                "--label",
                ",".join(labels),
            ).strip()
            number = int(url.rstrip("/").rsplit("/", 1)[-1])

        project_item_id = issue_info.get("project_item_id")
        if not project_item_id:
            added = gh(
                "project",
                "item-add",
                str(project_number),
                "--owner",
                args.project_owner,
                "--url",
                url,
                "--format",
                "json",
                check=False,
            )
            project_item_id = parse_project_item_id(added)

        issue_map["issues"][key] = {
            "number": number,
            "title": title,
            "url": url,
            "project_item_id": project_item_id,
        }
        for task in story.tasks:
            issue_map["task_to_issue"][task.task_id] = number

    issue_map_path.write_text(json.dumps(issue_map, indent=2) + "\n", encoding="utf-8")
    print(f"Synced {len(stories)} issue groups to {args.repo} and project {args.project_title}")
    return 0


def resolve_feature_dir(feature_dir: Path | None) -> Path:
    if feature_dir is None:
        feature_json = ROOT / ".specify" / "feature.json"
        if not feature_json.exists():
            raise SystemExit("No --feature-dir supplied and .specify/feature.json is missing")
        feature_dir = ROOT / json.loads(feature_json.read_text())["feature_directory"]
    elif not feature_dir.is_absolute():
        feature_dir = ROOT / feature_dir

    if not (feature_dir / "tasks.md").exists():
        raise SystemExit(f"{feature_dir}: tasks.md not found")
    return feature_dir


def parse_feature(feature_dir: Path) -> dict[str, Story]:
    story_meta: dict[str, tuple[str, str]] = {}
    for line in (feature_dir / "tasks.md").read_text(encoding="utf-8").splitlines():
        match = STORY_RE.match(line)
        if match:
            key = f"US{match.group('num')}"
            rest = match.group("rest").strip()
            priority_match = PRIORITY_RE.search(rest)
            if priority_match:
                title = rest[: priority_match.start()].strip()
                priority = priority_match.group("priority")
            else:
                title = rest
                priority = ""
            story_meta[key] = (title, priority)

    groups: dict[str, list[Task]] = {}
    for line in (feature_dir / "tasks.md").read_text(encoding="utf-8").splitlines():
        match = TASK_RE.match(line)
        if not match:
            continue
        story = match.group("story") or "shared"
        groups.setdefault(story, []).append(
            Task(
                task_id=match.group("id"),
                story=story,
                text=match.group("text").strip(),
                done=match.group("done").lower() == "x",
            )
        )

    stories: dict[str, Story] = {}
    for key, tasks in groups.items():
        title, priority = story_meta.get(key, ("Setup & Infrastructure", ""))
        stories[key] = Story(key=key, title=title, priority=priority, tasks=tasks)
    return stories


def issue_title(feature: str, key: str, story: Story) -> str:
    if key == "shared":
        return f"[{feature}] Setup & Infrastructure"
    return f"[{feature}] {key}: {story.title}"


def issue_body(feature_dir: Path, key: str, story: Story, project_title: str) -> str:
    lines = [
        f"## {story.title}",
        "",
        f"**Feature**: `{feature_dir.name}`",
        f"**Spec**: `{feature_dir.relative_to(ROOT) / 'spec.md'}`",
        f"**Tasks**: `{feature_dir.relative_to(ROOT) / 'tasks.md'}`",
        f"**GitHub Project**: `{project_title}`",
        "",
    ]
    if story.priority:
        lines += [f"**Priority**: {story.priority}", ""]

    lines += ["### Tasks", ""]
    for task in story.tasks:
        checkbox = "x" if task.done else " "
        lines.append(f"- [{checkbox}] {task.task_id} {task.text}")

    if key != "shared":
        lines += ["", "### Acceptance Criteria", "", "See the linked Speckit specification."]

    return "\n".join(lines) + "\n"


def load_issue_map(path: Path) -> dict:
    if path.exists():
        return json.loads(path.read_text(encoding="utf-8"))
    return {}


def ensure_gh_ready() -> None:
    result = subprocess.run(
        ["gh", "auth", "status", "-h", "github.com"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        raise SystemExit(
            "GitHub CLI is not authenticated for github.com. Run `gh auth login` and "
            "`gh auth refresh -s project` before live sync."
        )


def ensure_repo_matches(expected: str) -> None:
    actual = gh("repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner").strip()
    if actual != expected:
        raise SystemExit(f"GitHub remote mismatch: expected {expected}, got {actual}")


def ensure_labels(feature: str) -> None:
    labels = {
        "speckit": ("5319e7", "Synced from Speckit artifacts"),
        "story": ("0969da", "Speckit user story"),
        "infrastructure": ("6a737d", "Shared setup or infrastructure tasks"),
        f"feature:{feature}": ("0e8a16", f"Speckit feature {feature}"),
    }
    for name, (color, description) in labels.items():
        gh(
            "label",
            "create",
            name,
            "--color",
            color,
            "--description",
            description,
            check=False,
        )


def find_project_number(owner: str, title: str) -> int:
    raw = gh("project", "list", "--owner", owner, "--format", "json", "--limit", "100")
    data = json.loads(raw)
    projects = data.get("projects", data if isinstance(data, list) else [])
    matches = [project for project in projects if project.get("title") == title]
    if len(matches) != 1:
        raise SystemExit(f"Expected exactly one GitHub Project titled {title!r}; found {len(matches)}")
    return int(matches[0]["number"])


def parse_project_item_id(raw: str) -> str | None:
    try:
        data = json.loads(raw)
    except json.JSONDecodeError:
        return None
    return data.get("id") or data.get("item", {}).get("id")


def pull_checked_tasks_from_github(
    feature_dir: Path, stories: dict[str, Story], issue_map: dict
) -> int:
    if not issue_map:
        return 0

    known_tasks = {task.task_id: task.done for story in stories.values() for task in story.tasks}
    if not known_tasks:
        return 0

    checked_tasks: set[str] = set()
    for issue_info in issue_map.get("issues", {}).values():
        number = issue_info.get("number")
        if not number:
            continue
        body = gh("issue", "view", str(number), "--json", "body", "-q", ".body")
        checked_tasks.update(parse_checked_task_ids(body))

    safe_updates = {
        task_id for task_id in checked_tasks if task_id in known_tasks and not known_tasks[task_id]
    }
    if not safe_updates:
        return 0

    tasks_path = feature_dir / "tasks.md"
    original = tasks_path.read_text(encoding="utf-8").splitlines()
    updated: list[str] = []
    changed = 0
    for line in original:
        match = TASK_RE.match(line)
        if match and match.group("id") in safe_updates and match.group("done") == " ":
            updated.append(line.replace("- [ ]", "- [x]", 1))
            changed += 1
        else:
            updated.append(line)

    if changed:
        trailing_newline = "\n" if tasks_path.read_text(encoding="utf-8").endswith("\n") else ""
        tasks_path.write_text("\n".join(updated) + trailing_newline, encoding="utf-8")
    return changed


def parse_checked_task_ids(body: str) -> set[str]:
    checked: set[str] = set()
    for line in body.splitlines():
        match = CHECKBOX_RE.match(line.strip())
        if match and match.group("done").lower() == "x":
            checked.add(match.group("id"))
    return checked


def gh(*args: str, check: bool = True) -> str:
    result = subprocess.run(
        ["gh", *args],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if check and result.returncode != 0:
        hint = ""
        if args and args[0] == "project":
            hint = "\nHint: run `gh auth refresh -s project` and confirm the project title is unique."
        raise SystemExit(
            f"gh {' '.join(args)} failed with {result.returncode}\n{result.stderr.strip()}{hint}"
        )
    return result.stdout


if __name__ == "__main__":
    raise SystemExit(main())
