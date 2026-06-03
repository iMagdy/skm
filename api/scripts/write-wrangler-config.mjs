import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const configPath = resolve("wrangler.jsonc");
const replacements = {
  __SEARCH_CACHE_KV_ID__: process.env.SEARCH_CACHE_KV_ID,
  __SKILLS_INDEX_D1_ID__: process.env.SKILLS_INDEX_D1_ID
};

let config = readFileSync(configPath, "utf8");
const missing = [];

for (const [placeholder, value] of Object.entries(replacements)) {
  if (!value) {
    missing.push(placeholder.replace(/^__|__$/g, ""));
    continue;
  }
  config = config.replaceAll(placeholder, value);
}

if (missing.length > 0) {
  throw new Error(`Missing required Wrangler config values: ${missing.join(", ")}`);
}

writeFileSync(configPath, config);
