CREATE TABLE IF NOT EXISTS skills (
  id TEXT PRIMARY KEY,
  provider TEXT NOT NULL,
  name TEXT NOT NULL,
  source TEXT NOT NULL,
  skill TEXT NOT NULL,
  repo TEXT,
  installs INTEGER NOT NULL DEFAULT 0,
  stars INTEGER,
  url TEXT,
  install_target TEXT,
  installable INTEGER NOT NULL DEFAULT 0,
  description TEXT,
  category TEXT,
  occupation TEXT,
  tags_json TEXT NOT NULL DEFAULT '[]',
  updated_at TEXT,
  last_seen_at TEXT NOT NULL,
  payload_hash TEXT NOT NULL,
  provider_rank INTEGER NOT NULL DEFAULT 100
);

CREATE INDEX IF NOT EXISTS idx_skills_installable ON skills(installable);
CREATE INDEX IF NOT EXISTS idx_skills_source_skill ON skills(source, skill);
CREATE INDEX IF NOT EXISTS idx_skills_provider_rank ON skills(provider_rank, installs DESC);

CREATE VIRTUAL TABLE IF NOT EXISTS skills_fts USING fts5(
  id UNINDEXED,
  name,
  description,
  source,
  skill,
  tags,
  category,
  occupation
);
