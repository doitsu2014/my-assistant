/// OpenClaw Bot - Main Bot Logic
use anyhow::Result;
use crate::config::Config;
use crate::github::GitHubIntegration;
use crate::notion::NotionIntegration;
use octocrab::params::State;

pub struct OpenClawBot {
    config: Config,
    notion: NotionIntegration,
    github: GitHubIntegration,
}

impl OpenClawBot {
    /// Initialize the bot with required integrations
    pub fn new(config: Config) -> Result<Self> {
        println!("Initializing {}...", config.bot_name);
        
        // Initialize integrations
        let notion = NotionIntegration::new(
            config.notion_api_key.clone(),
            config.notion_database_id.clone(),
        );
        
        let github = GitHubIntegration::new(
            config.github_token.clone(),
            config.github_owner.clone(),
            config.github_repo.clone(),
        )?;
        
        println!("✓ Bot initialized successfully!");
        
        Ok(Self {
            config,
            notion,
            github,
        })
    }
    
    /// Extract task title from Notion task object
    fn extract_task_title(&self, task: &crate::notion::NotionTask) -> String {
        task.properties
            .get("Name")
            .and_then(|name| name.get("title"))
            .and_then(|title| title.as_array())
            .and_then(|arr| arr.first())
            .and_then(|first| first.get("text"))
            .and_then(|text| text.get("content"))
            .and_then(|content| content.as_str())
            .unwrap_or("Untitled")
            .to_string()
    }
    
    /// Sync tasks from Notion to GitHub issues
    pub fn sync_notion_to_github(&self) -> Result<Vec<crate::notion::NotionTask>> {
        println!("\n--- Syncing Notion tasks to GitHub ---");
        
        // Get tasks from Notion
        let notion_tasks = self.notion.get_tasks()?;
        println!("Found {} tasks in Notion", notion_tasks.len());
        
        // Process each task
        for task in &notion_tasks {
            let title = self.extract_task_title(task);
            println!("  - {}", title);
        }
        
        Ok(notion_tasks)
    }
    
    /// Sync GitHub issues back to Notion
    pub async fn sync_github_to_notion(&self) -> Result<Vec<octocrab::models::issues::Issue>> {
        println!("\n--- Syncing GitHub issues to Notion ---");
        
        // Get issues from GitHub
        let github_issues = self.github.get_issues(Some(State::Open)).await?;
        println!("Found {} open issues in GitHub", github_issues.len());
        
        // Process each issue
        for issue in &github_issues {
            println!("  - #{}: {}", issue.number, issue.title);
        }
        
        Ok(github_issues)
    }
    
    /// Create a Notion task from a GitHub issue
    pub async fn create_task_from_github_issue(&self, issue_number: u64) -> Result<crate::notion::NotionTask> {
        println!("\n--- Creating Notion task from GitHub issue #{} ---", issue_number);
        
        // Get the issue
        let issues = self.github.get_issues(None).await?;
        let issue = issues
            .iter()
            .find(|i| i.number == issue_number)
            .ok_or_else(|| anyhow::anyhow!("Issue #{} not found", issue_number))?;
        
        // Create corresponding Notion task
        let status = match issue.state {
            octocrab::models::IssueState::Open => "In Progress",
            octocrab::models::IssueState::Closed => "Done",
            _ => "Not Started",
        };
        
        let description = issue.body.as_deref();
        let task = self.notion.create_task(&issue.title, description, status)?;
        
        println!("✓ Created Notion task for issue #{}", issue_number);
        
        Ok(task)
    }
    
    /// Create a GitHub issue from a Notion task
    pub async fn create_github_issue_from_task(&self, task_title: &str, task_description: Option<&str>) -> Result<octocrab::models::issues::Issue> {
        println!("\n--- Creating GitHub issue from task: {} ---", task_title);
        
        let issue = self.github.create_issue(task_title, task_description, None, None).await?;
        
        println!("✓ Created GitHub issue #{}", issue.number);
        
        Ok(issue)
    }
    
    /// Main bot execution loop
    pub async fn run(&self) -> Result<()> {
        println!("\n{} is running!", self.config.bot_name);
        println!("\nAvailable operations:");
        println!("  1. Sync Notion tasks to GitHub");
        println!("  2. Sync GitHub issues to Notion");
        println!("  3. Create GitHub issue from Notion task");
        println!("  4. Create Notion task from GitHub issue");
        println!("\nFor automated syncing, you can schedule these operations using cron or a task scheduler.");
        println!("\n--- Running sync operations ---");
        
        // Example: Show current tasks and issues
        if let Err(e) = self.sync_notion_to_github() {
            eprintln!("Error syncing Notion to GitHub: {}", e);
        }
        
        if let Err(e) = self.sync_github_to_notion().await {
            eprintln!("Error syncing GitHub to Notion: {}", e);
        }
        
        Ok(())
    }
}
