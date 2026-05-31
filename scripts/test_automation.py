#!/usr/bin/env python3
"""Unit tests for repository automation helpers."""

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

import generate_release_docs as release_docs
import generate_homebrew_formula as homebrew_formula
import speckit_sync_issues as sync_issues


class SpeckitSyncTests(unittest.TestCase):
    def test_parse_feature_groups_shared_and_stories(self) -> None:
        stories = sync_issues.parse_feature(
            sync_issues.ROOT / "tests" / "fixtures" / "speckit-sync-feature"
        )

        self.assertEqual(["T001", "T002"], [task.task_id for task in stories["shared"].tasks])
        self.assertEqual("Sync Stories to GitHub", stories["US1"].title)
        self.assertEqual("P1", stories["US1"].priority)
        self.assertEqual(["T003", "T004"], [task.task_id for task in stories["US1"].tasks])

    def test_parse_checked_task_ids(self) -> None:
        body = "\n".join(
            [
                "- [x] T001 completed",
                "- [ ] T002 incomplete",
                "- [X] T003 completed",
            ]
        )

        self.assertEqual({"T001", "T003"}, sync_issues.parse_checked_task_ids(body))

    def test_pull_checked_tasks_from_github_only_marks_completed_tasks(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            feature_dir = Path(tmp) / "feature"
            feature_dir.mkdir()
            (feature_dir / "spec.md").write_text("# Fixture\n", encoding="utf-8")
            (feature_dir / "tasks.md").write_text(
                "\n".join(
                    [
                        "# Tasks",
                        "",
                        "## Phase 1: User Story 1 - Example (Priority: P1)",
                        "",
                        "- [ ] T001 [US1] Local incomplete",
                        "- [x] T002 [US1] Already complete",
                    ]
                )
                + "\n",
                encoding="utf-8",
            )
            stories = sync_issues.parse_feature(feature_dir)
            issue_map = {"issues": {"US1": {"number": 7}}}

            original_gh = sync_issues.gh
            sync_issues.gh = lambda *args, **kwargs: "- [x] T001 Local incomplete\n- [ ] T002 Already complete\n"
            try:
                changed = sync_issues.pull_checked_tasks_from_github(
                    feature_dir, stories, issue_map
                )
            finally:
                sync_issues.gh = original_gh

            self.assertEqual(1, changed)
            self.assertIn("- [x] T001 [US1]", (feature_dir / "tasks.md").read_text())
            self.assertIn("- [x] T002 [US1]", (feature_dir / "tasks.md").read_text())

    def test_find_project_number_requires_unique_title(self) -> None:
        original_gh = sync_issues.gh
        try:
            sync_issues.gh = lambda *args, **kwargs: json.dumps(
                {"projects": [{"title": "skm", "number": 3}]}
            )
            self.assertEqual(3, sync_issues.find_project_number("iMagdy", "skm"))

            sync_issues.gh = lambda *args, **kwargs: json.dumps(
                {
                    "projects": [
                        {"title": "skm", "number": 3},
                        {"title": "skm", "number": 4},
                    ]
                }
            )
            with self.assertRaises(SystemExit):
                sync_issues.find_project_number("iMagdy", "skm")
        finally:
            sync_issues.gh = original_gh

    def test_project_item_id_parsing(self) -> None:
        self.assertEqual("PVTI_1", sync_issues.parse_project_item_id('{"id":"PVTI_1"}'))
        self.assertEqual(
            "PVTI_2", sync_issues.parse_project_item_id('{"item":{"id":"PVTI_2"}}')
        )


class ReleaseDocsTests(unittest.TestCase):
    def test_release_body_has_single_download_table(self) -> None:
        body = release_docs.render_release_body("v1.2.3", None, [])

        self.assertIn("Initial release history", body)
        self.assertEqual(1, body.count("| Platform | Target | Archive | Checksum |"))

    def test_asset_table_has_all_tier_one_targets_and_checksums(self) -> None:
        table = "\n".join(release_docs.render_asset_table("v1.2.3"))

        for _platform, target, extension in release_docs.TARGETS:
            self.assertIn(f"skm-v1.2.3-{target}.{extension}", table)
            self.assertIn(f"skm-v1.2.3-{target}.{extension}.sha256", table)
        self.assertIn("skm-v1.2.3-checksums.txt", table)

    def test_changelog_groups_conventional_commits(self) -> None:
        grouped = release_docs.group_commits(
            [
                release_docs.Commit("abc1234", "feat: add export"),
                release_docs.Commit("def5678", "fix: repair docs"),
                release_docs.Commit("fff0000", "plain commit"),
            ]
        )

        self.assertEqual(["abc1234"], [commit.sha for commit in grouped["feat"]])
        self.assertEqual(["def5678"], [commit.sha for commit in grouped["fix"]])
        self.assertEqual(["fff0000"], [commit.sha for commit in grouped["other"]])

    def test_semver_key_accepts_only_v_tags(self) -> None:
        self.assertEqual((1, 2, 3), release_docs.semver_key("v1.2.3"))
        self.assertIsNone(release_docs.semver_key("1.2.3"))
        self.assertIsNone(release_docs.semver_key("v1.2.3-beta"))

    def test_release_workflow_contains_expected_asset_and_release_steps(self) -> None:
        workflow = (release_docs.ROOT / ".github" / "workflows" / "release.yml").read_text(
            encoding="utf-8"
        )

        for _platform, target, _extension in release_docs.TARGETS:
            self.assertIn(target, workflow)
        self.assertIn(".sha256", workflow)
        self.assertIn("checksums.txt", workflow)
        self.assertIn("gh release create", workflow)
        self.assertIn("gh release upload", workflow)
        self.assertIn("gh pr create", workflow)
        self.assertIn("packages: write", workflow)
        self.assertIn("oras push", workflow)
        self.assertIn("generate_homebrew_formula.py", workflow)
        self.assertIn("HOMEBREW_TAP_TOKEN", workflow)
        self.assertIn("CARGO_REGISTRY_TOKEN", workflow)
        self.assertIn("cargo publish --locked", workflow)

    def test_homebrew_formula_uses_release_assets_and_checksums(self) -> None:
        checksums = {
            "skm-v1.2.3-x86_64-apple-darwin.tar.gz": "a" * 64,
            "skm-v1.2.3-aarch64-apple-darwin.tar.gz": "b" * 64,
            "skm-v1.2.3-x86_64-unknown-linux-gnu.tar.gz": "c" * 64,
            "skm-v1.2.3-x86_64-pc-windows-msvc.zip": "d" * 64,
        }

        formula = homebrew_formula.render_formula("v1.2.3", checksums)

        self.assertIn('class Skm < Formula', formula)
        self.assertIn('version "1.2.3"', formula)
        self.assertIn('depends_on "git"', formula)
        self.assertIn("on_macos do", formula)
        self.assertIn("on_arm do", formula)
        self.assertIn("on_intel do", formula)
        self.assertIn("x86_64-apple-darwin", formula)
        self.assertIn("aarch64-apple-darwin", formula)
        self.assertIn("on_linux do", formula)
        self.assertIn("x86_64-unknown-linux-gnu", formula)
        self.assertNotIn("x86_64-pc-windows-msvc", formula)
        self.assertIn('bin.install "skm"', formula)

    def test_homebrew_checksum_parser_accepts_sha256sum_lines(self) -> None:
        checksums = homebrew_formula.parse_checksums(
            "\n".join(
                [
                    f"{'A' * 64}  skm-v1.2.3-x86_64-apple-darwin.tar.gz",
                    f"{'b' * 64} *skm-v1.2.3-aarch64-apple-darwin.tar.gz",
                ]
            )
        )

        self.assertEqual("a" * 64, checksums["skm-v1.2.3-x86_64-apple-darwin.tar.gz"])
        self.assertEqual("b" * 64, checksums["skm-v1.2.3-aarch64-apple-darwin.tar.gz"])

    def test_ci_runs_coverage_after_primary_gates(self) -> None:
        ci = (release_docs.ROOT / ".github" / "workflows" / "ci.yml").read_text(
            encoding="utf-8"
        )

        self.assertIn("needs: [fmt, clippy, test, build, docs]", ci)
        self.assertIn("cargo tarpaulin --fail-under 95", ci)


if __name__ == "__main__":
    unittest.main(verbosity=2)
