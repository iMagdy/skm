# Data Model: GitHub CI Pipeline

**Feature**: 005-github-ci-pipeline
**Date**: Sat May 30 2026

## Entities

### Pipeline Workflow

The top-level CI configuration defined in `.github/workflows/ci.yml`.

| Attribute | Description |
|-----------|-------------|
| name | Workflow display name (e.g., "CI") |
| trigger | Event types that start the workflow (`pull_request`) |
| concurrency | Group key for cancellation (`${{ github.head_ref }}`) |
| jobs | Set of independent jobs to execute |

### Job

An independent unit of work within the pipeline. Each job runs in its own runner environment.

| Attribute | Description |
|-----------|-------------|
| name | Job display name (e.g., "lint", "test", "build") |
| runs-on | Runner image (e.g., `ubuntu-latest`) |
| timeout | Maximum run time in minutes (default: 15) |
| steps | Ordered list of steps to execute |

### Step

A single action or command within a job.

| Attribute | Description |
|-----------|-------------|
| name | Step display name |
| uses | Action to execute (e.g., `actions/checkout@v4`) |
| run | Shell command to execute |
| if | Conditional execution expression |

### Cache Configuration

Defines what to cache between pipeline runs.

| Attribute | Description |
|-----------|-------------|
| key | Cache key based on `Cargo.lock` hash |
| path | Directories to cache (`~/.cargo/registry`, `target/`) |
| restore-keys | Fallback keys for partial cache hits |

## Relationships

```
Pipeline Workflow (1) ──contains──> (N) Job
Job (1) ──contains──> (N) Step
Job (1) ──uses──> (1) Cache Configuration
Pipeline Workflow (1) ──reports status to──> (1) Pull Request
Job (1) ──reports──> (1) Check Status
```

## State Transitions

### Pipeline Run States

```
pending ──[runner assigned]──> running ──[all jobs pass]──> passed
                                │
                                └──[any job fails]──> failed
                                │
                                └──[timeout exceeded]──> failed
                                │
                                └──[newer run started]──> cancelled
```

### Check Status Mapping

| Pipeline State | GitHub Status | Check Name |
|----------------|---------------|------------|
| pending | `pending` | `lint` / `test` / `build` |
| running | `in_progress` | `lint` / `test` / `build` |
| passed | `success` | `lint` / `test` / `build` |
| failed | `failure` | `lint` / `test` / `build` |
| cancelled | `cancelled` | `lint` / `test` / `build` |

## Validation Rules

- Each job MUST have at least one step
- Timeout MUST be a positive integer (minutes)
- Cache key MUST change when `Cargo.lock` changes
- Concurrency group MUST be unique per PR branch

## Scale Assumptions

- Maximum concurrent PRs: ~10 (small team)
- Maximum pipeline duration: 15 minutes
- Maximum workflow file size: 500 lines
- Runner disk space: ~14GB (GitHub-hosted default)
