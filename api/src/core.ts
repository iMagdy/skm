import type {
  BudgetDecision,
  CacheEntry,
  ProviderResult,
  SearchParams,
  SearchResponse,
  SkillSearchResult
} from "./types";

const DEFAULT_LIMIT = 20;
const MAX_LIMIT = 100;

export function validateClientHeaders(headers: Headers): boolean {
  const userAgent = headers.get("user-agent") ?? "";
  const client = headers.get("x-ktesio-client") ?? "";
  return /^ktesio\/[0-9]+(?:\.[0-9]+){0,2}(?:[-+][A-Za-z0-9._-]+)?$/.test(userAgent.trim())
    && client.trim() === "kt-cli";
}

export function parseSearchParams(url: URL): SearchParams | Response {
  const query = (url.searchParams.get("q") ?? "").trim();
  if (query.length < 2) {
    return jsonError(400, "BAD_REQUEST", "Search query must be at least 2 characters.");
  }

  const limit = clampNumber(url.searchParams.get("limit"), DEFAULT_LIMIT, 1, MAX_LIMIT);
  const page = clampNumber(url.searchParams.get("page"), 1, 1, 100);
  const sortBy = optionalParam(url.searchParams.get("sortBy"));
  const category = optionalParam(url.searchParams.get("category"));
  const occupation = optionalParam(url.searchParams.get("occupation"));

  return {
    query,
    limit,
    page,
    ...(sortBy ? { sortBy } : {}),
    ...(category ? { category } : {}),
    ...(occupation ? { occupation } : {})
  };
}

export function canonicalCacheKey(params: SearchParams): string {
  const parts = new URLSearchParams();
  parts.set("q", params.query.toLowerCase());
  parts.set("limit", params.limit.toString());
  parts.set("page", params.page.toString());
  if (params.sortBy) parts.set("sortBy", params.sortBy.toLowerCase());
  if (params.category) parts.set("category", params.category.toLowerCase());
  if (params.occupation) parts.set("occupation", params.occupation.toLowerCase());
  return `search:v1:${parts.toString()}`;
}

export function cacheRequestFor(request: Request, params: SearchParams): Request {
  const url = new URL(request.url);
  url.search = "";
  url.searchParams.set("q", params.query.toLowerCase());
  url.searchParams.set("limit", params.limit.toString());
  url.searchParams.set("page", params.page.toString());
  if (params.sortBy) url.searchParams.set("sortBy", params.sortBy.toLowerCase());
  if (params.category) url.searchParams.set("category", params.category.toLowerCase());
  if (params.occupation) url.searchParams.set("occupation", params.occupation.toLowerCase());
  return new Request(url.toString(), { method: "GET" });
}

export function makeSearchResponse(
  data: SkillSearchResult[],
  provider: SearchResponse["meta"]["provider"],
  cache: SearchResponse["meta"]["cache"],
  params: SearchParams,
  quota?: SearchResponse["meta"]["quota"]
): SearchResponse {
  return {
    data,
    meta: {
      provider,
      cache,
      query: params.query,
      limit: params.limit,
      page: params.page,
      count: data.length,
      ...(quota ? { quota } : {})
    }
  };
}

export async function hashResults(results: SkillSearchResult[]): Promise<string> {
  const encoded = new TextEncoder().encode(stableJson(results));
  const digest = await crypto.subtle.digest("SHA-256", encoded);
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

export async function cacheEntry(
  response: SearchResponse,
  provider: CacheEntry["provider"]
): Promise<CacheEntry> {
  return {
    hash: await hashResults(response.data),
    provider,
    response,
    cached_at: new Date().toISOString()
  };
}

export function normalizeSkillsMpResponse(body: unknown): SkillSearchResult[] {
  const items = arrayFromProviderBody(body);
  return items.map((item, index) => normalizeProviderSkill(item, "skillsmp", index));
}

export function normalizeSkillsShResponse(body: unknown): SkillSearchResult[] {
  const items = arrayFromProviderBody(body);
  return items.map((item, index) => normalizeProviderSkill(item, "skills_sh", index));
}

export function skillsMpUrl(baseUrl: string, params: SearchParams): string {
  const url = new URL(baseUrl);
  url.searchParams.set("q", params.query);
  url.searchParams.set("limit", params.limit.toString());
  url.searchParams.set("page", params.page.toString());
  if (params.sortBy) url.searchParams.set("sortBy", params.sortBy);
  if (params.category) url.searchParams.set("category", params.category);
  if (params.occupation) url.searchParams.set("occupation", params.occupation);
  return url.toString();
}

export function skillsShUrl(baseUrl: string, params: SearchParams): string {
  const url = new URL(baseUrl);
  url.searchParams.set("q", params.query);
  url.searchParams.set("limit", params.limit.toString());
  return url.toString();
}

export async function fetchProvider(
  provider: "skillsmp" | "skills_sh",
  url: string,
  init: RequestInit,
  fetchImpl: typeof fetch
): Promise<ProviderResult> {
  try {
    const response = await fetchImpl(url, init);
    const headers = interestingHeaders(response.headers);
    if (!response.ok) {
      return {
        ok: false,
        retryable: response.status === 429 || response.status >= 500,
        status: response.status,
        provider,
        data: [],
        headers,
        error: `HTTP ${response.status}`
      };
    }

    const body = await response.json();
    return {
      ok: true,
      retryable: false,
      status: response.status,
      provider,
      data: provider === "skillsmp" ? normalizeSkillsMpResponse(body) : normalizeSkillsShResponse(body),
      headers
    };
  } catch (error) {
    return {
      ok: false,
      retryable: true,
      status: 0,
      provider,
      data: [],
      headers: {},
      error: error instanceof Error ? error.message : "network error"
    };
  }
}

export function d1ResultsAreSufficient(results: SkillSearchResult[], limit: number): boolean {
  return results.some((result) => result.installable) || results.length >= limit;
}

export function ftsQuery(query: string): string | null {
  const terms = query.toLowerCase().match(/[a-z0-9]+/g);
  if (!terms || terms.length === 0) return null;
  return terms.slice(0, 8).map((term) => `${term}*`).join(" ");
}

export function parseDailyRemaining(headers: Record<string, string>): number | null {
  const value = headers["x-ratelimit-daily-remaining"];
  if (!value) return null;
  const parsed = Number.parseInt(value, 10);
  return Number.isFinite(parsed) ? parsed : null;
}

export function budgetToQuota(decision: BudgetDecision): SearchResponse["meta"]["quota"] {
  return decision.allowed
    ? undefined
    : {
        skillsmp_remaining_daily: null,
        skillsmp_blocked_reason: decision.reason ?? "unknown"
      };
}

export function jsonResponse(body: unknown, init: ResponseInit = {}): Response {
  const headers = new Headers(init.headers);
  headers.set("content-type", "application/json; charset=utf-8");
  headers.set("cache-control", headers.get("cache-control") ?? "public, max-age=60");
  return new Response(JSON.stringify(body), { ...init, headers });
}

export function jsonError(status: number, code: string, message: string): Response {
  return jsonResponse(
    { error: { code, message } },
    {
      status,
      headers: {
        "cache-control": "no-store"
      }
    }
  );
}

function clampNumber(value: string | null, fallback: number, min: number, max: number): number {
  const parsed = value ? Number.parseInt(value, 10) : fallback;
  if (!Number.isFinite(parsed)) return fallback;
  return Math.min(max, Math.max(min, parsed));
}

function optionalParam(value: string | null): string | undefined {
  const trimmed = value?.trim();
  return trimmed ? trimmed : undefined;
}

function arrayFromProviderBody(body: unknown): Record<string, unknown>[] {
  if (Array.isArray(body)) return body.filter(isRecord);
  if (!isRecord(body)) return [];
  const candidates = [body.data, body.skills, body.results, body.items];
  for (const candidate of candidates) {
    if (Array.isArray(candidate)) return candidate.filter(isRecord);
  }
  return [];
}

function normalizeProviderSkill(
  item: Record<string, unknown>,
  provider: "skillsmp" | "skills_sh",
  index: number
): SkillSearchResult {
  const name = stringField(item, ["name", "title", "skillName"]) ?? `skill-${index + 1}`;
  const source = normalizeSource(item);
  const skill = normalizeSkillSlug(item, name);
  const repo = normalizeRepo(item, source);
  const installTarget = repo && isGithubSource(source) ? `${source}/${skill}` : null;
  const tags = arrayField(item, ["tags", "keywords"]);

  return {
    id: stringField(item, ["id", "skillId", "slug"]) ?? `${provider}:${source}:${skill}`,
    name,
    source,
    skill,
    repo,
    installs: numberField(item, ["installs", "installCount", "downloads"]) ?? 0,
    stars: numberField(item, ["stars", "starCount", "githubStars"]),
    url: stringField(item, ["url", "htmlUrl", "pageUrl"]),
    install_target: installTarget,
    installable: installTarget !== null,
    description: stringField(item, ["description", "summary", "readme"]),
    category: stringField(item, ["category"]),
    occupation: stringField(item, ["occupation"]),
    tags,
    updated_at: stringField(item, ["updated_at", "updatedAt", "lastUpdated"])
  };
}

function normalizeSource(item: Record<string, unknown>): string {
  const source = stringField(item, ["source", "sourceRepo", "repository", "repo"]);
  if (source) {
    const github = githubOwnerRepo(source);
    return github ?? source.replace(/^github:/, "").replace(/\/$/, "");
  }
  const repoUrl = stringField(item, ["repoUrl", "repositoryUrl", "githubUrl", "installUrl"]);
  return repoUrl ? githubOwnerRepo(repoUrl) ?? repoUrl : "unknown";
}

function normalizeSkillSlug(item: Record<string, unknown>, name: string): string {
  const slug = stringField(item, ["skill", "slug", "skillId", "packageName"]);
  if (slug) return slug.split("/").filter(Boolean).pop() ?? slug;
  return name.toLowerCase().replace(/[^a-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "") || "skill";
}

function normalizeRepo(item: Record<string, unknown>, source: string): string | null {
  const explicit = stringField(item, ["repo", "repoUrl", "repositoryUrl", "githubUrl", "installUrl"]);
  const github = explicit ? githubOwnerRepo(explicit) : null;
  if (github) return `https://github.com/${github}.git`;
  return isGithubSource(source) ? `https://github.com/${source}.git` : null;
}

function isGithubSource(source: string): boolean {
  return /^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(source);
}

function githubOwnerRepo(value: string): string | null {
  const trimmed = value.trim().replace(/\.git$/, "");
  const httpsMatch = trimmed.match(/^https:\/\/github\.com\/([^/\s]+\/[^/\s]+)(?:\/.*)?$/i);
  if (httpsMatch?.[1]) return httpsMatch[1];
  const sshMatch = trimmed.match(/^git@github\.com:([^/\s]+\/[^/\s]+)(?:\/.*)?$/i);
  if (sshMatch?.[1]) return sshMatch[1];
  return isGithubSource(trimmed) ? trimmed : null;
}

function stringField(item: Record<string, unknown>, names: string[]): string | null {
  for (const name of names) {
    const value = item[name];
    if (typeof value === "string" && value.trim()) return value.trim();
  }
  return null;
}

function numberField(item: Record<string, unknown>, names: string[]): number | null {
  for (const name of names) {
    const value = item[name];
    if (typeof value === "number" && Number.isFinite(value)) return Math.max(0, Math.trunc(value));
    if (typeof value === "string" && value.trim()) {
      const parsed = Number.parseInt(value, 10);
      if (Number.isFinite(parsed)) return Math.max(0, parsed);
    }
  }
  return null;
}

function arrayField(item: Record<string, unknown>, names: string[]): string[] {
  for (const name of names) {
    const value = item[name];
    if (Array.isArray(value)) {
      return value.filter((entry): entry is string => typeof entry === "string" && entry.trim().length > 0);
    }
  }
  return [];
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function interestingHeaders(headers: Headers): Record<string, string> {
  const names = [
    "retry-after",
    "x-ratelimit-daily-remaining",
    "x-ratelimit-remaining",
    "x-ratelimit-reset",
    "ratelimit-remaining"
  ];
  const result: Record<string, string> = {};
  for (const name of names) {
    const value = headers.get(name);
    if (value) result[name] = value;
  }
  return result;
}

function stableJson(value: unknown): string {
  return JSON.stringify(sortForJson(value));
}

function sortForJson(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(sortForJson);
  if (!isRecord(value)) return value;
  return Object.keys(value)
    .sort()
    .reduce<Record<string, unknown>>((accumulator, key) => {
      accumulator[key] = sortForJson(value[key]);
      return accumulator;
    }, {});
}
