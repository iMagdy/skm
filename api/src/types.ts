export interface Env {
  SEARCH_CACHE: KVNamespace;
  DB: D1Database;
  UPSTREAM_BUDGET: DurableObjectNamespace;
  SKILLS_MP_API_TOKEN?: string;
  SKILLSMP_SEARCH_URL?: string;
  SKILLS_SH_SEARCH_URL?: string;
  UPSTREAM_DAILY_CAP?: string;
  UPSTREAM_MINUTE_CAP?: string;
}

export interface UpstreamBudgetStub {
  reserveSkillsMp(now?: number): Promise<BudgetDecision>;
  updateSkillsMpHeaders(headers: Record<string, string>, now?: number): Promise<void>;
}

export interface ExecutionContextLike {
  waitUntil(promise: Promise<unknown>): void;
}

export interface BudgetDecision {
  allowed: boolean;
  reason?: "daily" | "minute" | "provider_daily";
  retryAfterSeconds?: number;
}

export interface SearchParams {
  query: string;
  limit: number;
  page: number;
  sortBy?: string;
  category?: string;
  occupation?: string;
}

export interface SkillSearchResult {
  id: string;
  name: string;
  source: string;
  skill: string;
  repo?: string | null;
  installs: number;
  stars?: number | null;
  url?: string | null;
  install_target?: string | null;
  installable: boolean;
  description?: string | null;
  category?: string | null;
  occupation?: string | null;
  tags?: string[];
  updated_at?: string | null;
}

export interface SearchResponse {
  data: SkillSearchResult[];
  meta: {
    provider: "cache" | "d1" | "skillsmp" | "skills_sh";
    cache: "miss" | "edge-hit" | "kv-hit" | "d1-hit" | "stale" | "fallback";
    query: string;
    limit: number;
    page: number;
    count: number;
    quota?: {
      skillsmp_remaining_daily?: number | null;
      skillsmp_blocked_reason?: string | null;
    };
  };
}

export interface CacheEntry {
  hash: string;
  provider: "d1" | "skillsmp" | "skills_sh";
  response: SearchResponse;
  cached_at: string;
}

export interface ProviderResult {
  ok: boolean;
  retryable: boolean;
  status: number;
  provider: "skillsmp" | "skills_sh";
  data: SkillSearchResult[];
  headers: Record<string, string>;
  error?: string;
}
