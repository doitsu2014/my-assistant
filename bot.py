"""
OpenClaw Bot - Main Entry Point

A bot that assists with planning on Notion and managing tasks on GitHub repositories.
"""
import sys
from config import Config
from notion_integration import NotionIntegration
from github_integration import GitHubIntegration


class OpenClawBot:
    """Main bot class that orchestrates Notion and GitHub integrations"""
    
    def __init__(self):
        """Initialize the bot with required integrations"""
        print(f"Initializing {Config.BOT_NAME}...")
        
        # Validate configuration
        missing_config = Config.validate()
        if missing_config:
            print(f"Error: Missing required configuration: {', '.join(missing_config)}")
            print("Please create a .env file based on .env.example and fill in the required values.")
            sys.exit(1)
        
        # Initialize integrations
        try:
            self.notion = NotionIntegration()
            self.github = GitHubIntegration()
            print("✓ Bot initialized successfully!")
        except Exception as e:
            print(f"Error initializing bot: {e}")
            sys.exit(1)
    
    def _extract_task_title(self, task):
        """
        Extract task title from Notion task object
        
        Args:
            task (dict): Notion task object
            
        Returns:
            str: Task title or 'Untitled' if not found
        """
        try:
            title_data = task.get('properties', {}).get('Name', {}).get('title', [])
            if title_data:
                return title_data[0].get('text', {}).get('content', 'Untitled')
        except (KeyError, IndexError, TypeError):
            pass
        return 'Untitled'
    
    def sync_notion_to_github(self):
        """
        Sync tasks from Notion to GitHub issues
        
        This method retrieves tasks from Notion and creates corresponding
        GitHub issues for tracking and assignment.
        """
        print("\n--- Syncing Notion tasks to GitHub ---")
        
        # Get tasks from Notion
        notion_tasks = self.notion.get_tasks()
        print(f"Found {len(notion_tasks)} tasks in Notion")
        
        # Process each task
        for task in notion_tasks:
            title = self._extract_task_title(task)
            print(f"  - {title}")
        
        return notion_tasks
    
    def sync_github_to_notion(self):
        """
        Sync GitHub issues back to Notion
        
        This method retrieves GitHub issues and updates corresponding
        Notion tasks with the latest status.
        """
        print("\n--- Syncing GitHub issues to Notion ---")
        
        # Get issues from GitHub
        github_issues = self.github.get_issues()
        print(f"Found {len(github_issues)} open issues in GitHub")
        
        # Process each issue
        for issue in github_issues:
            print(f"  - #{issue.number}: {issue.title}")
        
        return github_issues
    
    def create_task_from_github_issue(self, issue_number):
        """
        Create a Notion task from a GitHub issue
        
        Args:
            issue_number (int): GitHub issue number
        """
        print(f"\n--- Creating Notion task from GitHub issue #{issue_number} ---")
        
        # Get the issue
        issue = self.github.repo.get_issue(issue_number)
        
        # Create corresponding Notion task
        task = self.notion.create_task(
            title=issue.title,
            description=issue.body or "No description provided",
            status="In Progress" if issue.state == "open" else "Done"
        )
        
        if task:
            print(f"✓ Created Notion task for issue #{issue_number}")
        else:
            print(f"✗ Failed to create Notion task")
        
        return task
    
    def create_github_issue_from_task(self, task_title, task_description=None):
        """
        Create a GitHub issue from a Notion task
        
        Args:
            task_title (str): Task title
            task_description (str): Task description
        """
        print(f"\n--- Creating GitHub issue from task: {task_title} ---")
        
        issue = self.github.create_issue(
            title=task_title,
            body=task_description
        )
        
        if issue:
            print(f"✓ Created GitHub issue #{issue.number}")
        else:
            print(f"✗ Failed to create GitHub issue")
        
        return issue
    
    def run(self):
        """
        Main bot execution loop
        
        This is a placeholder for the main bot logic.
        You can customize this to run scheduled tasks, listen to webhooks, etc.
        """
        print(f"\n{Config.BOT_NAME} is running!")
        print("\nAvailable operations:")
        print("  1. Sync Notion tasks to GitHub")
        print("  2. Sync GitHub issues to Notion")
        print("  3. Create GitHub issue from Notion task")
        print("  4. Create Notion task from GitHub issue")
        print("\nFor automated syncing, you can schedule these operations using cron or a task scheduler.")


def main():
    """Main entry point"""
    try:
        bot = OpenClawBot()
        bot.run()
        
        # Example: Show current tasks and issues
        bot.sync_notion_to_github()
        bot.sync_github_to_notion()
        
    except KeyboardInterrupt:
        print("\n\nBot stopped by user.")
    except Exception as e:
        print(f"\nError running bot: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
