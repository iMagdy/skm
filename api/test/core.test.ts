import { describe, expect, it } from "vitest";

import {
  canonicalCacheKey,
  d1ResultsAreSufficient,
  ftsQuery,
  normalizeSkillsMpResponse,
  normalizeSkillsShResponse,
  parseSearchParams,
  skillsMpUrl,
  validateClientHeaders
} from "../src/core";

describe("client header gate", () => {
  it("accepts kt CLI headers", () => {
    const headers = new Headers({
      "user-agent": "ktesio/0.4.0",
      "x-ktesio-client": "kt-cli"
    });

    expect(validateClientHeaders(headers)).toBe(true);
  });

  it("rejects missing or spoof-poor headers", () => {
    expect(validateClientHeaders(new Headers())).toBe(false);
    expect(
      validateClientHeaders(
        new Headers({
          "user-agent": "curl/8.0",
          "x-ktesio-client": "kt-cli"
        })
      )
    ).toBe(false);
  });
});

describe("query canonicalization", () => {
  it("normalizes cache keys", () => {
    const first = parseSearchParams(new URL("https://api.ktesio.dev/search-skills?q=React&limit=10"));
    const second = parseSearchParams(new URL("https://api.ktesio.dev/search-skills?q=react&limit=10"));
    if (first instanceof Response || second instanceof Response) throw new Error("unexpected response");

    expect(canonicalCacheKey(first)).toEqual(canonicalCacheKey(second));
  });

  it("builds SkillsMP URLs with supported filters", () => {
    const params = parseSearchParams(
      new URL("https://api.ktesio.dev/search-skills?q=react&limit=10&page=2&sortBy=recent&category=frontend")
    );
    if (params instanceof Response) throw new Error("unexpected response");

    const url = skillsMpUrl("https://skillsmp.com/api/v1/skills/search", params);

    expect(url).toContain("q=react");
    expect(url).toContain("limit=10");
    expect(url).toContain("page=2");
    expect(url).toContain("sortBy=recent");
    expect(url).toContain("category=frontend");
  });
});

describe("provider normalization", () => {
  it("normalizes SkillsMP-style results", () => {
    const results = normalizeSkillsMpResponse({
      data: [
        {
          id: "react-agent",
          slug: "react",
          name: "React",
          source: "example/skills",
          installs: "12",
          stars: 5,
          description: "React help",
          tags: ["frontend"]
        }
      ]
    });

    expect(results[0]).toMatchObject({
      id: "react-agent",
      name: "React",
      source: "example/skills",
      skill: "react",
      repo: "https://github.com/example/skills.git",
      install_target: "example/skills/react",
      installable: true,
      installs: 12,
      stars: 5
    });
  });

  it("normalizes Skills.sh fallback results", () => {
    const results = normalizeSkillsShResponse({
      skills: [
        {
          id: "hashicorp/agent-skills/run-acceptance-tests",
          skillId: "run-acceptance-tests",
          name: "run-acceptance-tests",
          source: "hashicorp/agent-skills",
          installs: 1468
        }
      ]
    });

    expect(results[0]?.install_target).toBe("hashicorp/agent-skills/run-acceptance-tests");
  });
});

describe("D1 search helpers", () => {
  it("creates a safe FTS prefix query", () => {
    expect(ftsQuery("React Native!!")).toBe("react* native*");
  });

  it("treats installable D1 hits as sufficient", () => {
    expect(
      d1ResultsAreSufficient(
        [
          {
            id: "one",
            name: "one",
            source: "example/skills",
            skill: "one",
            installs: 0,
            installable: true,
            install_target: "example/skills/one"
          }
        ],
        20
      )
    ).toBe(true);
  });
});
