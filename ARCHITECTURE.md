# OpenClaw Bot - Architecture & How It Works

## Overview

OpenClaw Bot is a bi-directional synchronization tool that bridges Notion (for planning) and GitHub (for task execution). It helps teams plan in Notion and seamlessly sync those plans with GitHub issues for developer execution.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        OpenClaw Bot                              │
│                                                                   │
│  ┌─────────────┐      ┌──────────────┐      ┌────────────────┐ │
│  │   Config    │      │     Bot      │      │   Main Entry   │ │
│  │  (dotenv)   │─────▶│ Orchestrator │◀─────│     Point      │ │
│  └─────────────┘      └──────┬───────┘      └────────────────┘ │
│                               │                                  │
│                    ┌──────────┴──────────┐                      │
│                    │                     │                      │
│           ┌────────▼────────┐   ┌───────▼────────┐            │
│           │     Notion      │   │     GitHub     │            │
│           │   Integration   │   │  Integration   │            │
│           │   (reqwest)     │   │  (octocrab)    │            │
│           └────────┬────────┘   └───────┬────────┘            │
│                    │                     │                      │
└────────────────────┼─────────────────────┼──────────────────────┘
                     │                     │
                     │                     │
            ┌────────▼────────┐   ┌───────▼────────┐
            │  Notion API     │   │   GitHub API   │
            │  (Database)     │   │   (Issues)     │
            └─────────────────┘   └────────────────┘
```

## Component Architecture

### 1. Configuration Module (`src/config.rs`)

**Purpose**: Manages environment-based configuration

```rust
Config {
    notion_api_key: String,
    notion_database_id: String,
    github_token: String,
    github_owner: String,
    github_repo: String,
    bot_name: String,
    log_level: String,
}
```

**Key Features**:
- Loads environment variables via `dotenv` crate
- Validates required configuration at startup
- Fails fast if configuration is missing

### 2. Notion Integration (`src/notion.rs`)

**Purpose**: Handles all Notion API interactions

**API Methods**:
- `get_tasks()` - Retrieves all tasks from Notion database
- `create_task()` - Creates a new task with title, description, status
- `update_task_status()` - Updates the status of a task

**Implementation**:
- Uses `reqwest` blocking client
- Custom JSON serialization with `serde`
- Direct HTTP calls to Notion API (no official Rust client exists)

### 3. GitHub Integration (`src/github.rs`)

**Purpose**: Manages GitHub issue operations

**API Methods**:
- `get_issues()` - Lists issues from repository
- `create_issue()` - Creates new issue with title, body, labels, assignees
- `assign_issue()` - Assigns users to an issue
- `update_issue_status()` - Opens or closes issues
- `add_comment()` - Adds comments to issues

**Implementation**:
- Uses `octocrab` async client (official GitHub API wrapper)
- Full async/await support via Tokio
- Rich type support for GitHub entities

### 4. Bot Orchestrator (`src/bot.rs`)

**Purpose**: Coordinates synchronization between Notion and GitHub

**Core Methods**:
- `sync_notion_to_github()` - Syncs Notion tasks to GitHub issues
- `sync_github_to_notion()` - Syncs GitHub issues back to Notion
- `create_task_from_github_issue()` - Creates Notion task from specific issue
- `create_github_issue_from_task()` - Creates GitHub issue from Notion task

**Sync Logic**:
- Maintains task/issue mapping
- Handles status translation between platforms
- Extracts structured data from both APIs

## Data Flow Diagrams

### Sync Flow: Notion → GitHub

```
┌──────────────┐
│   Notion DB  │
│   (Tasks)    │
└──────┬───────┘
       │
       │ 1. Query tasks
       │
       ▼
┌──────────────────┐
│ NotionIntegration│
│  .get_tasks()    │
└──────┬───────────┘
       │
       │ 2. Parse task data
       │
       ▼
┌──────────────────┐
│  OpenClawBot     │
│  Extract titles, │
│  descriptions    │
└──────┬───────────┘
       │
       │ 3. Create issues
       │
       ▼
┌──────────────────┐
│GitHubIntegration │
│ .create_issue()  │
└──────┬───────────┘
       │
       │ 4. API call
       │
       ▼
┌──────────────┐
│  GitHub API  │
│   (Issues)   │
└──────────────┘
```

### Sync Flow: GitHub → Notion

```
┌──────────────┐
│  GitHub API  │
│   (Issues)   │
└──────┬───────┘
       │
       │ 1. List open issues
       │
       ▼
┌──────────────────┐
│GitHubIntegration │
│  .get_issues()   │
└──────┬───────────┘
       │
       │ 2. Parse issue data
       │
       ▼
┌──────────────────┐
│  OpenClawBot     │
│  Map status:     │
│  open→In Progress│
│  closed→Done     │
└──────┬───────────┘
       │
       │ 3. Create tasks
       │
       ▼
┌──────────────────┐
│ NotionIntegration│
│ .create_task()   │
└──────┬───────────┘
       │
       │ 4. API call
       │
       ▼
┌──────────────┐
│   Notion DB  │
│   (Tasks)    │
└──────────────┘
```

## Execution Flow

### Startup Sequence

```
main() 
  │
  ├─▶ Load .env file (dotenv)
  │
  ├─▶ Config::from_env()
  │     └─▶ Validate required variables
  │           └─▶ Fail if missing
  │
  ├─▶ OpenClawBot::new(config)
  │     ├─▶ NotionIntegration::new()
  │     └─▶ GitHubIntegration::new()
  │
  └─▶ bot.run()
        ├─▶ sync_notion_to_github()
        │     └─▶ Display Notion tasks
        │
        └─▶ sync_github_to_notion()
              └─▶ Display GitHub issues
```

### Typical Use Cases

#### Use Case 1: Planning to Execution

```
1. Team creates tasks in Notion with:
   - Task name
   - Description
   - Status: "Not Started"

2. Run OpenClaw Bot
   └─▶ Tasks become GitHub issues

3. Developers work on issues:
   - Assign themselves
   - Comment on progress
   - Close when done

4. Run OpenClaw Bot again
   └─▶ Issue status syncs back to Notion
       (closed → "Done")
```

#### Use Case 2: Issue Tracking to Planning

```
1. Developers create GitHub issues:
   - Bug reports
   - Feature requests
   - Technical debt

2. Run OpenClaw Bot
   └─▶ Issues appear in Notion database

3. Product team reviews in Notion:
   - Prioritize tasks
   - Add planning details
   - Set status

4. Changes remain in both systems
```

## Technology Stack

### Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.x | Async runtime for concurrent operations |
| `octocrab` | 0.39 | GitHub API client (async) |
| `reqwest` | 0.11 | HTTP client for Notion API (blocking) |
| `dotenv` | 0.15 | Environment variable management |
| `serde` | 1.0 | JSON serialization/deserialization |
| `anyhow` | 1.0 | Error handling with context |
| `thiserror` | 1.0 | Custom error types |

### Why Rust?

- **Performance**: Fast execution, low memory footprint (8.3MB binary)
- **Safety**: Memory-safe, no null pointer exceptions
- **Reliability**: Strong type system catches errors at compile time
- **Concurrency**: Fearless concurrency with async/await
- **Deployment**: Single binary, no runtime dependencies

## Configuration

### Environment Variables

```bash
# Notion Configuration
NOTION_API_KEY=secret_xxx          # Your Notion integration token
NOTION_DATABASE_ID=xxx-xxx-xxx     # Target database ID

# GitHub Configuration  
GITHUB_TOKEN=ghp_xxx               # Personal access token
GITHUB_OWNER=username              # Repository owner
GITHUB_REPO=my-assistant           # Repository name

# Bot Configuration (Optional)
BOT_NAME=OpenClaw Assistant        # Display name
LOG_LEVEL=INFO                     # Logging verbosity
```

### Notion Database Schema

Required properties:

| Property | Type | Description |
|----------|------|-------------|
| Name | Title | Task name |
| Description | Text | Task details |
| Status | Select | Task status (Not Started, In Progress, Done) |

## Building and Running

### Development Build

```bash
# Quick build for testing
cargo build

# Run with debugging
cargo run
```

### Production Build

```bash
# Optimized binary (8.3MB)
cargo build --release

# Run optimized version
./target/release/openclaw-bot
```

### Docker Deployment (Optional)

```dockerfile
FROM rust:1.70 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/openclaw-bot /usr/local/bin/
CMD ["openclaw-bot"]
```

## Error Handling

The bot uses Rust's `Result` type for robust error handling:

```rust
// All operations return Result<T, Error>
pub fn get_tasks(&self) -> Result<Vec<NotionTask>> { ... }
pub async fn get_issues(&self) -> Result<Vec<Issue>> { ... }

// Errors are propagated with context
.map_err(|e| anyhow::anyhow!("Failed to sync: {}", e))?
```

**Error Categories**:
- Configuration errors (missing env vars) - Exit immediately
- API errors (network, auth) - Log and continue
- Data parsing errors - Skip malformed entries

## Scheduling and Automation

### Cron Job (Linux/macOS)

```bash
# Run every hour
0 * * * * cd /path/to/my-assistant && ./target/release/openclaw-bot

# Run every 6 hours
0 */6 * * * cd /path/to/my-assistant && ./target/release/openclaw-bot
```

### GitHub Actions

```yaml
name: Sync Tasks
on:
  schedule:
    - cron: '0 */6 * * *'  # Every 6 hours
  workflow_dispatch:        # Manual trigger

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - name: Build
        run: cargo build --release
      - name: Run sync
        env:
          NOTION_API_KEY: ${{ secrets.NOTION_API_KEY }}
          NOTION_DATABASE_ID: ${{ secrets.NOTION_DATABASE_ID }}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          GITHUB_OWNER: ${{ github.repository_owner }}
          GITHUB_REPO: ${{ github.event.repository.name }}
        run: ./target/release/openclaw-bot
```

## Future Enhancements

Potential features for future development:

1. **Bidirectional Field Mapping**
   - Sync assignees between platforms
   - Sync labels/tags
   - Sync due dates

2. **Incremental Sync**
   - Track last sync timestamp
   - Only sync changed items
   - Reduce API calls

3. **Conflict Resolution**
   - Handle simultaneous updates
   - Merge strategies
   - Conflict logging

4. **Web Dashboard**
   - View sync status
   - Manual sync triggers
   - Sync history logs

5. **Multiple Repository Support**
   - Sync multiple repos to one Notion DB
   - Repository-specific views in Notion

## Troubleshooting

### Common Issues

**Issue**: "Missing required environment variable"
- **Solution**: Ensure `.env` file exists with all required variables

**Issue**: "Error retrieving tasks from Notion"
- **Solution**: Check NOTION_API_KEY and database permissions

**Issue**: "GitHub authentication failed"
- **Solution**: Verify GITHUB_TOKEN has `repo` scope

**Issue**: "Task not found in database"
- **Solution**: Ensure database has required properties (Name, Status)

## Contributing

When extending the bot:

1. Add new methods to integration modules
2. Update `OpenClawBot` orchestrator for new workflows
3. Add error handling with proper context
4. Update this documentation with new features
5. Run `cargo test` before submitting

## Security Considerations

- **Never commit `.env`** - It's in `.gitignore`
- **Rotate tokens regularly** - Both GitHub and Notion
- **Use minimal permissions** - Only grant necessary API scopes
- **Secure deployment** - Use secrets management in production
- **Audit logs** - Review bot actions periodically
