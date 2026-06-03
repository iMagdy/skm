import { DurableObject } from "cloudflare:workers";

import {
  budgetToQuota,
  cacheEntry,
  cacheRequestFor,
  canonicalCacheKey,
  d1ResultsAreSufficient,
  fetchProvider,
  ftsQuery,
  jsonError,
  jsonResponse,
  makeSearchResponse,
  parseDailyRemaining,
  parseSearchParams,
  skillsMpUrl,
  skillsShUrl,
  validateClientHeaders
} from "./core";
import type {
  BudgetDecision,
  CacheEntry,
  Env,
  ExecutionContextLike,
  SearchParams,
  SearchResponse,
  SkillSearchResult,
  UpstreamBudgetStub
} from "./types";

const DEFAULT_SKILLSMP_SEARCH_URL = "https://skillsmp.com/api/v1/skills/search";
const DEFAULT_SKILLS_SH_SEARCH_URL = "https://www.skills.sh/api/search";

export class UpstreamBudget extends DurableObject<Env> {
  async reserveSkillsMp(now = Date.now()): Promise<BudgetDecision> {
    const dailyCap = parseCap(this.env.UPSTREAM_DAILY_CAP, 450);
    const minuteCap = parseCap(this.env.UPSTREAM_MINUTE_CAP, 25);
    const dayKey = `skillsmp:day:${utcDay(now)}`;
    const minuteKey = `skillsmp:minute:${Math.floor(now / 60_000)}`;
    const providerBlockedUntil = await this.ctx.storage.get<number>("skillsmp:provider-blocked-until");

    if (providerBlockedUntil && providerBlockedUntil > now) {
      return {
        allowed: false,
        reason: "provider_daily",
        retryAfterSeconds: Math.ceil((providerBlockedUntil - now) / 1000)
      };
    }

    const [dailyUsed, minuteUsed] = await Promise.all([
      this.ctx.storage.get<number>(dayKey),
      this.ctx.storage.get<number>(minuteKey)
    ]);
    if ((dailyUsed ?? 0) >= dailyCap) {
      return {
        allowed: false,
        reason: "daily",
        retryAfterSeconds: Math.ceil((nextUtcDay(now) - now) / 1000)
      };
    }
    if ((minuteUsed ?? 0) >= minuteCap) {
      return {
        allowed: false,
        reason: "minute",
        retryAfterSeconds: 60 - Math.floor((now % 60_000) / 1000)
      };
    }

    await Promise.all([
      this.ctx.storage.put(dayKey, (dailyUsed ?? 0) + 1),
      this.ctx.storage.put(minuteKey, (minuteUsed ?? 0) + 1)
    ]);
    return { allowed: true };
  }

  async updateSkillsMpHeaders(headers: Record<string, string>, now = Date.now()): Promise<void> {
    const remaining = parseDailyRemaining(headers);
    if (remaining !== null && remaining <= 0) {
      await this.ctx.storage.put("skillsmp:provider-blocked-until", nextUtcDay(now));
    }
  }
}

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    return handleRequest(request, env, ctx, fetch);
  }
};

export async function handleRequest(
  request: Request,
  env: Env,
  ctx: ExecutionContextLike,
  fetchImpl: typeof fetch
): Promise<Response> {
  const url = new URL(request.url);
  if (request.method !== "GET") {
    return jsonError(405, "METHOD_NOT_ALLOWED", "Only GET is supported.");
  }
  if (url.pathname !== "/search-skills") {
    return jsonError(404, "NOT_FOUND", "Endpoint not found.");
  }
  if (!validateClientHeaders(request.headers)) {
    return jsonError(403, "FORBIDDEN", "Use the kt CLI to search skills.");
  }

  const paramsOrResponse = parseSearchParams(url);
  if (paramsOrResponse instanceof Response) return paramsOrResponse;
  const params = paramsOrResponse;
  const cacheKey = canonicalCacheKey(params);
  const cacheRequest = cacheRequestFor(request, params);

  const edgeHit = await matchEdgeCache(cacheRequest);
  if (edgeHit) return edgeHit;

  const kvEntry = await getCacheEntry(env, cacheKey);
  if (kvEntry) {
    const response = withCacheState(kvEntry.response, "cache", "kv-hit");
    const httpResponse = jsonResponse(response);
    ctx.waitUntil(putEdgeCache(cacheRequest, httpResponse.clone()));
    return httpResponse;
  }

  const d1Results = await searchD1(env, params);
  if (d1ResultsAreSufficient(d1Results, params.limit)) {
    const response = makeSearchResponse(d1Results, "d1", "d1-hit", params);
    ctx.waitUntil(writeCache(env, cacheKey, cacheRequest, response, "d1"));
    return jsonResponse(response);
  }

  const skillsMpResponse = await fetchSkillsMp(env, params, fetchImpl);
  if (skillsMpResponse.ok) {
    const response = makeSearchResponse(skillsMpResponse.data, "skillsmp", "miss", params, {
      skillsmp_remaining_daily: parseDailyRemaining(skillsMpResponse.headers)
    });
    ctx.waitUntil(updateD1(env, skillsMpResponse.data, "skillsmp"));
    ctx.waitUntil(writeCache(env, cacheKey, cacheRequest, response, "skillsmp"));
    ctx.waitUntil(updateBudgetHeaders(env, skillsMpResponse.headers));
    return jsonResponse(response);
  }

  if (skillsMpResponse.retryable) {
    const fallbackResponse = await fetchSkillsSh(env, params, fetchImpl);
    if (fallbackResponse.ok) {
      const response = makeSearchResponse(fallbackResponse.data, "skills_sh", "fallback", params, {
        skillsmp_blocked_reason: skillsMpResponse.error ?? `HTTP ${skillsMpResponse.status}`
      });
      return jsonResponse(response);
    }
  }

  return jsonError(
    skillsMpResponse.retryable ? 503 : 502,
    "SEARCH_UNAVAILABLE",
    "Skill search is temporarily unavailable."
  );
}

async function fetchSkillsMp(env: Env, params: SearchParams, fetchImpl: typeof fetch) {
  const decision = await reserveBudget(env);
  if (!decision.allowed) {
    return {
      ok: false as const,
      retryable: true,
      status: 429,
      provider: "skillsmp" as const,
      data: [],
      headers: {},
      error: budgetToQuota(decision)?.skillsmp_blocked_reason ?? "quota"
    };
  }

  const headers = new Headers({
    Accept: "application/json",
    "User-Agent": "ktesio-search-api/0.1.0"
  });
  if (env.SKILLS_MP_API_TOKEN) {
    headers.set("Authorization", `Bearer ${env.SKILLS_MP_API_TOKEN}`);
  }

  return fetchProvider(
    "skillsmp",
    skillsMpUrl(env.SKILLSMP_SEARCH_URL ?? DEFAULT_SKILLSMP_SEARCH_URL, params),
    { headers },
    fetchImpl
  );
}

async function fetchSkillsSh(env: Env, params: SearchParams, fetchImpl: typeof fetch) {
  return fetchProvider(
    "skills_sh",
    skillsShUrl(env.SKILLS_SH_SEARCH_URL ?? DEFAULT_SKILLS_SH_SEARCH_URL, params),
    {
      headers: {
        Accept: "application/json",
        "User-Agent": "ktesio-search-api/0.1.0"
      }
    },
    fetchImpl
  );
}

async function reserveBudget(env: Env): Promise<BudgetDecision> {
  const stub = env.UPSTREAM_BUDGET.getByName("skillsmp") as unknown as UpstreamBudgetStub;
  return stub.reserveSkillsMp();
}

async function updateBudgetHeaders(env: Env, headers: Record<string, string>): Promise<void> {
  const stub = env.UPSTREAM_BUDGET.getByName("skillsmp") as unknown as UpstreamBudgetStub;
  await stub.updateSkillsMpHeaders(headers);
}

async function getCacheEntry(env: Env, key: string): Promise<CacheEntry | null> {
  const raw = await env.SEARCH_CACHE.get(key);
  if (!raw) return null;
  try {
    return JSON.parse(raw) as CacheEntry;
  } catch {
    return null;
  }
}

async function writeCache(
  env: Env,
  key: string,
  cacheRequest: Request,
  response: SearchResponse,
  provider: CacheEntry["provider"]
): Promise<void> {
  const entry = await cacheEntry(response, provider);
  await env.SEARCH_CACHE.put(key, JSON.stringify(entry));
  await putEdgeCache(cacheRequest, jsonResponse(response));
}

async function searchD1(env: Env, params: SearchParams): Promise<SkillSearchResult[]> {
  const query = ftsQuery(params.query);
  if (!query) return [];
  const offset = (params.page - 1) * params.limit;
  const statement = env.DB.prepare(`
    SELECT
      s.id,
      s.name,
      s.source,
      s.skill,
      s.repo,
      s.installs,
      s.stars,
      s.url,
      s.install_target,
      s.installable,
      s.description,
      s.category,
      s.occupation,
      s.tags_json,
      s.updated_at
    FROM skills s
    JOIN skills_fts ON skills_fts.id = s.id
    WHERE skills_fts MATCH ?
    ORDER BY bm25(skills_fts), s.installable DESC, s.installs DESC, s.provider_rank ASC
    LIMIT ? OFFSET ?
  `).bind(query, params.limit, offset);

  const result = await statement.all<D1SkillRow>();
  return (result.results ?? []).map(rowToSkill);
}

async function updateD1(env: Env, results: SkillSearchResult[], provider: "skillsmp"): Promise<void> {
  if (results.length === 0) return;
  const now = new Date().toISOString();
  const statements: D1PreparedStatement[] = [];

  for (const result of results) {
    const hash = await hashSkill(result);
    const tagsJson = JSON.stringify(result.tags ?? []);
    statements.push(
      env.DB.prepare(`
        INSERT INTO skills (
          id, provider, name, source, skill, repo, installs, stars, url, install_target,
          installable, description, category, occupation, tags_json, updated_at,
          last_seen_at, payload_hash, provider_rank
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
          provider = excluded.provider,
          name = excluded.name,
          source = excluded.source,
          skill = excluded.skill,
          repo = excluded.repo,
          installs = excluded.installs,
          stars = excluded.stars,
          url = excluded.url,
          install_target = excluded.install_target,
          installable = excluded.installable,
          description = excluded.description,
          category = excluded.category,
          occupation = excluded.occupation,
          tags_json = excluded.tags_json,
          updated_at = excluded.updated_at,
          last_seen_at = excluded.last_seen_at,
          payload_hash = excluded.payload_hash,
          provider_rank = excluded.provider_rank
      `).bind(
        result.id,
        provider,
        result.name,
        result.source,
        result.skill,
        result.repo ?? null,
        result.installs,
        result.stars ?? null,
        result.url ?? null,
        result.install_target ?? null,
        result.installable ? 1 : 0,
        result.description ?? null,
        result.category ?? null,
        result.occupation ?? null,
        tagsJson,
        result.updated_at ?? null,
        now,
        hash,
        10
      ),
      env.DB.prepare("DELETE FROM skills_fts WHERE id = ?").bind(result.id),
      env.DB.prepare(`
        INSERT INTO skills_fts (id, name, description, source, skill, tags, category, occupation)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
      `).bind(
        result.id,
        result.name,
        result.description ?? "",
        result.source,
        result.skill,
        (result.tags ?? []).join(" "),
        result.category ?? "",
        result.occupation ?? ""
      )
    );
  }

  await env.DB.batch(statements);
}

async function hashSkill(result: SkillSearchResult): Promise<string> {
  const encoded = new TextEncoder().encode(JSON.stringify(result));
  const digest = await crypto.subtle.digest("SHA-256", encoded);
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

function rowToSkill(row: D1SkillRow): SkillSearchResult {
  return {
    id: row.id,
    name: row.name,
    source: row.source,
    skill: row.skill,
    repo: row.repo,
    installs: row.installs,
    stars: row.stars,
    url: row.url,
    install_target: row.install_target,
    installable: row.installable === 1,
    description: row.description,
    category: row.category,
    occupation: row.occupation,
    tags: parseTags(row.tags_json),
    updated_at: row.updated_at
  };
}

function parseTags(value: string): string[] {
  try {
    const parsed = JSON.parse(value) as unknown;
    return Array.isArray(parsed) ? parsed.filter((entry): entry is string => typeof entry === "string") : [];
  } catch {
    return [];
  }
}

async function matchEdgeCache(cacheRequest: Request): Promise<Response | null> {
  if (typeof caches === "undefined") return null;
  return (await cloudflareDefaultCache().match(cacheRequest)) ?? null;
}

async function putEdgeCache(cacheRequest: Request, response: Response): Promise<void> {
  if (typeof caches === "undefined") return;
  await cloudflareDefaultCache().put(cacheRequest, response);
}

function cloudflareDefaultCache(): Cache {
  return (caches as unknown as { default: Cache }).default;
}

function withCacheState(
  response: SearchResponse,
  provider: SearchResponse["meta"]["provider"],
  cache: SearchResponse["meta"]["cache"]
): SearchResponse {
  return {
    ...response,
    meta: {
      ...response.meta,
      provider,
      cache
    }
  };
}

function parseCap(value: string | undefined, fallback: number): number {
  const parsed = value ? Number.parseInt(value, 10) : fallback;
  return Number.isFinite(parsed) ? parsed : fallback;
}

function utcDay(now: number): string {
  return new Date(now).toISOString().slice(0, 10);
}

function nextUtcDay(now: number): number {
  const date = new Date(now);
  return Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), date.getUTCDate() + 1);
}

interface D1SkillRow {
  id: string;
  name: string;
  source: string;
  skill: string;
  repo: string | null;
  installs: number;
  stars: number | null;
  url: string | null;
  install_target: string | null;
  installable: number;
  description: string | null;
  category: string | null;
  occupation: string | null;
  tags_json: string;
  updated_at: string | null;
}
