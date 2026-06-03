# Ktesio Search API

Cloudflare Worker for `GET https://api.ktesio.dev/search-skills`.

The Worker is deliberately cache-first:

1. Cloudflare WAF should block requests missing `User-Agent: ktesio/<version>` and `X-Ktesio-Client: kt-cli`.
2. The Worker validates those headers as a second check.
3. `caches.default` and `SEARCH_CACHE` KV serve exact query responses.
4. D1 stores normalized skills and an FTS5 index for first-party search.
5. SkillsMP is used only when cache and D1 cannot satisfy a request.
6. Skills.sh is a cold fallback for SkillsMP `429`, `5xx`, or network failures.

## Cloudflare Setup

Create the durable storage resources once:

```bash
cd api
npx wrangler kv namespace create SEARCH_CACHE
npx wrangler d1 create ktesio-search-index
```

Record the returned IDs as GitHub repository variables:

- `SEARCH_CACHE_KV_ID`
- `SKILLS_INDEX_D1_ID`

For local deploys, export those values and prepare the Wrangler config:

```bash
cd api
export SEARCH_CACHE_KV_ID="..."
export SKILLS_INDEX_D1_ID="..."
npm run prepare:config
npx wrangler secret put SKILLS_MP_API_TOKEN
npx wrangler d1 migrations apply ktesio-search-index --remote
npx wrangler deploy
```

Recommended WAF custom rule expression:

```text
(http.host eq "api.ktesio.dev" and http.request.uri.path eq "/search-skills" and not (http.user_agent matches "^ktesio/[0-9]+(\\.[0-9]+){0,2}([-+][A-Za-z0-9._-]+)?$" and http.request.headers["x-ktesio-client"][0] eq "kt-cli"))
```

Use a WAF rate limiting rule on the same path for coarse abuse control.
