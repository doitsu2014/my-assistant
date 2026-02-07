# OpenClaw Bot - My Assistant

A Python-based bot that assists with planning on Notion and managing tasks in GitHub repositories.

## Features

- 🗂️ **Notion Integration**: Manage tasks and planning in Notion databases
- 🐙 **GitHub Integration**: Create and manage issues, assign tasks to team members
- 🔄 **Bi-directional Sync**: Sync tasks between Notion and GitHub
- ⚙️ **Easy Configuration**: Environment-based configuration for flexible deployment

## Prerequisites

- Python 3.7 or higher
- Notion account with API access
- GitHub account with personal access token
- A Notion database for task management

## Setup

### 1. Clone the Repository

```bash
git clone https://github.com/doitsu2014/my-assistant.git
cd my-assistant
```

### 2. Install Dependencies

```bash
pip install -r requirements.txt
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

### 5. Run the Bot

```bash
python bot.py
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
├── bot.py                    # Main bot entry point
├── config.py                 # Configuration management
├── notion_integration.py     # Notion API integration
├── github_integration.py     # GitHub API integration
├── requirements.txt          # Python dependencies
├── .env.example             # Example environment variables
├── .gitignore               # Git ignore file
└── README.md                # This file
```

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

### Adding New Features

The bot is designed to be extensible. You can add new features by:

1. Adding new methods to `NotionIntegration` or `GitHubIntegration` classes
2. Creating new operations in the `OpenClawBot` class
3. Updating the `run()` method to include your new operations

### Scheduling

For automated syncing, you can:

- Use `cron` on Linux/macOS
- Use Task Scheduler on Windows
- Deploy to a cloud platform with scheduled functions (AWS Lambda, Google Cloud Functions, etc.)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the MIT License.

## Support

For issues, questions, or contributions, please open an issue in the GitHub repository.