import { describe, expect, it, vi } from "vitest";

import { handleRequest } from "../src/index";
import type { Env, ExecutionContextLike, SkillSearchResult } from "../src/types";

describe("Worker request flow", () => {
  it("returns 403 without kt headers", async () => {
    const response = await handleRequest(
      new Request("https://api.ktesio.dev/search-skills?q=react"),
      fakeEnv(),
      fakeCtx(),
      vi.fn() as unknown as typeof fetch
    );

    expect(response.status).toBe(403);
  });

  it("serves a KV hit before providers", async () => {
    const skill = result("react");
    const env = fakeEnv({
      kvValue: JSON.stringify({
        hash: "hash",
        provider: "skillsmp",
        cached_at: "2026-06-03T00:00:00Z",
        response: {
          data: [skill],
          meta: {
            provider: "skillsmp",
            cache: "miss",
            query: "react",
            limit: 20,
            page: 1,
            count: 1
          }
        }
      })
    });
    const fetchImpl = vi.fn() as unknown as typeof fetch;

    const ctx = fakeCtx();
    const response = await handleRequest(request("react"), env, ctx, fetchImpl);
    await Promise.all(ctx.promises);
    const body = await response.json() as { data: SkillSearchResult[]; meta: { cache: string } };

    expect(response.status).toBe(200);
    expect(body.meta.cache).toBe("kv-hit");
    expect(body.data[0]?.skill).toBe("react");
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it("uses SkillsMP on D1 miss and writes KV", async () => {
    const env = fakeEnv();
    const fetchImpl = vi.fn(async () => Response.json({
      data: [
        {
          id: "react",
          name: "React",
          source: "example/skills",
          slug: "react",
          installs: 4
        }
      ]
    })) as unknown as typeof fetch;

    const ctx = fakeCtx();
    const response = await handleRequest(request("react"), env, ctx, fetchImpl);
    await Promise.all(ctx.promises);
    const body = await response.json() as { data: SkillSearchResult[]; meta: { provider: string } };

    expect(response.status).toBe(200);
    expect(body.meta.provider).toBe("skillsmp");
    expect(body.data[0]?.install_target).toBe("example/skills/react");
    expect(env.SEARCH_CACHE.put).toHaveBeenCalled();
    expect(env.DB.batch).toHaveBeenCalled();
  });

  it("falls back to Skills.sh on cold SkillsMP retryable failure", async () => {
    const env = fakeEnv();
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = input.toString();
      if (url.includes("skillsmp.com")) {
        return new Response("unavailable", { status: 503 });
      }
      return Response.json({
        skills: [
          {
            id: "example/skills/react",
            skillId: "react",
            name: "React",
            source: "example/skills",
            installs: 1
          }
        ]
      });
    }) as unknown as typeof fetch;

    const response = await handleRequest(request("react"), env, fakeCtx(), fetchImpl);
    const body = await response.json() as { meta: { provider: string; cache: string } };

    expect(response.status).toBe(200);
    expect(body.meta.provider).toBe("skills_sh");
    expect(body.meta.cache).toBe("fallback");
    expect(env.DB.batch).not.toHaveBeenCalled();
  });
});

function request(query: string): Request {
  return new Request(`https://api.ktesio.dev/search-skills?q=${query}`, {
    headers: {
      "user-agent": "ktesio/0.4.0",
      "x-ktesio-client": "kt-cli"
    }
  });
}

function fakeCtx(): ExecutionContextLike & { promises: Promise<unknown>[] } {
  const promises: Promise<unknown>[] = [];
  return {
    promises,
    waitUntil(promise: Promise<unknown>) {
      promises.push(promise.catch(() => undefined));
    }
  };
}

function fakeEnv(options: { kvValue?: string | null; d1Results?: unknown[] } = {}): Env {
  const prepare = vi.fn((sql: string) => ({
    bind: vi.fn((..._values: unknown[]) => ({
      all: vi.fn(async () => ({ results: options.d1Results ?? [] })),
      run: vi.fn(async () => ({ success: true })),
      sql
    }))
  }));
  return {
    SEARCH_CACHE: {
      get: vi.fn(async () => options.kvValue ?? null),
      put: vi.fn(async () => undefined)
    } as unknown as KVNamespace,
    DB: {
      prepare,
      batch: vi.fn(async () => [])
    } as unknown as D1Database,
    UPSTREAM_BUDGET: {
      getByName: vi.fn(() => ({
        reserveSkillsMp: vi.fn(async () => ({ allowed: true })),
        updateSkillsMpHeaders: vi.fn(async () => undefined)
      }))
    } as unknown as DurableObjectNamespace,
    SKILLS_MP_API_TOKEN: "test",
    SKILLSMP_SEARCH_URL: "https://skillsmp.com/api/v1/skills/search",
    SKILLS_SH_SEARCH_URL: "https://www.skills.sh/api/search"
  };
}

function result(skill: string): SkillSearchResult {
  return {
    id: skill,
    name: skill,
    source: "example/skills",
    skill,
    installs: 0,
    installable: true,
    install_target: `example/skills/${skill}`
  };
}
