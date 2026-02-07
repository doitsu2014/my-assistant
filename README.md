# OpenClaw Bot - My Assistant

A Rust-based bot that assists with planning on Notion and managing tasks in GitHub repositories.

## 📚 Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Detailed architecture, diagrams, and how the bot works
- **[Quick Start](#setup)** - Get started in 5 minutes
- **[Usage Examples](#example-workflows)** - Common use cases

## How It Works

```
┌─────────────┐         ┌──────────────┐         ┌─────────────┐
│   Notion    │◀───────▶│  OpenClaw    │◀───────▶│   GitHub    │
│  Database   │  Sync   │     Bot      │  Sync   │   Issues    │
│  (Planning) │         │ (Rust/Tokio) │         │ (Execution) │
└─────────────┘         └──────────────┘         └─────────────┘
      │                                                  │
      │  • Task Management                              │
      │  • Status Tracking          ┌──────────┐        │  • Issue Tracking
      │  • Team Planning      ─────▶│  Config  │◀─────  │  • Assignments
      │                             │  (.env)  │        │  • Comments
      │                             └──────────┘        │
      └───────────────────────────────────────────────────┘
                   Bi-directional Sync
```

**Key Features**:
- 🗂️ **Notion Integration**: Manage tasks and planning in Notion databases
- 🐙 **GitHub Integration**: Create and manage issues, assign tasks to team members
- 🔄 **Bi-directional Sync**: Keep Notion and GitHub in sync automatically
- ⚙️ **Easy Configuration**: Environment-based configuration for flexible deployment
- 🦀 **Written in Rust**: Fast (8.3MB binary), reliable, and memory-safe

For detailed architecture, data flows, and technical deep-dive, see **[ARCHITECTURE.md](ARCHITECTURE.md)**.

## Prerequisites

- Rust 1.70 or higher (install from [rustup.rs](https://rustup.rs/))
- Notion account with API access
- GitHub account with personal access token
- A Notion database for task management

## Setup

### 1. Clone the Repository

```bash
git clone https://github.com/doitsu2014/my-assistant.git
cd my-assistant
```

### 2. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Configure Environment Variables

Copy the example environment file and fill in your credentials:

```bash
cp .env.example .env
```

Edit `.env` and provide:

- **NOTION_API_KEY**: Your Notion integration API key ([Get it here](https://www.notion.so/my-integrations))
- **NOTION_DATABASE_ID**: The ID of your Notion database
- **GITHUB_TOKEN**: Your GitHub personal access token ([Create one here](https://github.com/settings/tokens))
- **GITHUB_OWNER**: Your GitHub username or organization name
- **GITHUB_REPO**: The repository name you want to manage

### 4. Set Up Notion Database

Your Notion database should have the following properties:
- **Name** (Title): Task name
- **Description** (Text): Task description
- **Status** (Select): Task status (e.g., "Not Started", "In Progress", "Done")

### 5. Build and Run the Bot

```bash
# Build the project
cargo build --release

# Run the bot
cargo run --release
```

## Usage

The bot provides several key operations:

### Sync Notion Tasks to GitHub

Retrieves tasks from your Notion database and can create corresponding GitHub issues.

### Sync GitHub Issues to Notion

Retrieves open issues from GitHub and can create corresponding tasks in Notion.

### Create Individual Tasks/Issues

- Create a GitHub issue from a Notion task
- Create a Notion task from a GitHub issue

## Project Structure

```
my-assistant/
├── src/
│   ├── main.rs              # Main entry point
│   ├── bot.rs               # Bot orchestration logic
│   ├── config.rs            # Configuration management
│   ├── notion.rs            # Notion API integration
│   └── github.rs            # GitHub API integration
├── Cargo.toml               # Rust dependencies
├── .env.example             # Example environment variables
├── .gitignore               # Git ignore file
└── README.md                # This file
```

## Dependencies

The bot uses the following Rust crates:

- **reqwest**: HTTP client for API requests
- **tokio**: Async runtime
- **serde/serde_json**: JSON serialization
- **dotenv**: Environment variable loading
- **anyhow/thiserror**: Error handling
- **octocrab**: GitHub API client

## Example Workflows

### Automated Task Assignment

1. Create tasks in Notion with descriptions
2. Run the bot to sync tasks to GitHub as issues
3. Assign issues to team members in GitHub
4. Update issue status in GitHub
5. Sync back to Notion to keep planning up-to-date

### Issue Tracking

1. Team members create issues in GitHub
2. Bot syncs issues to Notion for planning review
3. Update task priorities and planning in Notion
4. Changes reflect in GitHub for execution

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Testing

```bash
cargo test
```

### Adding New Features

The bot is designed to be extensible. You can add new features by:

1. Adding new methods to the integration modules (`notion.rs`, `github.rs`)
2. Creating new operations in the `OpenClawBot` struct in `bot.rs`
3. Updating the `run()` method to include your new operations

### Scheduling

For automated syncing, you can:

- Use `cron` on Linux/macOS
- Use Task Scheduler on Windows
- Deploy to a cloud platform with scheduled functions (AWS Lambda, Google Cloud Functions, etc.)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

For technical details on extending the bot, see the [Architecture Documentation](ARCHITECTURE.md#contributing).

## Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Complete technical documentation
  - Architecture diagrams
  - Component descriptions
  - Data flow diagrams
  - API integration details
  - Deployment strategies
  - Error handling patterns
  - Future enhancements

## License

This project is open source and available under the MIT License.

## Support

For issues, questions, or contributions, please open an issue in the GitHub repository.