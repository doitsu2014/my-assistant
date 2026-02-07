/// GitHub integration for OpenClaw Bot
use anyhow::Result;
use octocrab::Octocrab;
use octocrab::models::issues::Issue;
use octocrab::models::IssueState;
use octocrab::params::State;

#[derive(Debug, Clone)]
pub struct GitHubIntegration {
    client: Octocrab,
    owner: String,
    repo: String,
}

impl GitHubIntegration {
    /// Initialize GitHub client
    pub fn new(token: String, owner: String, repo: String) -> Result<Self> {
        let client = Octocrab::builder()
            .personal_token(token)
            .build()?;
        
        Ok(Self {
            client,
            owner,
            repo,
        })
    }
    
    /// Retrieve issues from GitHub repository
    pub async fn get_issues(&self, state: Option<State>) -> Result<Vec<Issue>> {
        let issues_api = self.client.issues(&self.owner, &self.repo);
        let mut issues_handler = issues_api.list();
        
        if let Some(s) = state {
            issues_handler = issues_handler.state(s);
        }
        
        let issues = issues_handler
            .send()
            .await?
            .items;
        
        Ok(issues)
    }
    
    /// Create a new issue in GitHub repository
    pub async fn create_issue(
        &self,
        title: &str,
        body: Option<&str>,
        labels: Option<Vec<String>>,
        assignees: Option<Vec<String>>,
    ) -> Result<Issue> {
        let issues_api = self.client.issues(&self.owner, &self.repo);
        let mut issue_builder = issues_api.create(title);
        
        if let Some(b) = body {
            issue_builder = issue_builder.body(b);
        }
        
        if let Some(l) = labels {
            issue_builder = issue_builder.labels(l);
        }
        
        if let Some(a) = assignees {
            issue_builder = issue_builder.assignees(a);
        }
        
        let issue = issue_builder.send().await?;
        Ok(issue)
    }
    
    /// Assign an issue to users
    pub async fn assign_issue(&self, issue_number: u64, assignees: Vec<String>) -> Result<Issue> {
        let issue = self.client
            .issues(&self.owner, &self.repo)
            .update(issue_number)
            .assignees(&assignees)
            .send()
            .await?;
        
        Ok(issue)
    }
    
    /// Update issue state (open/closed)
    pub async fn update_issue_status(&self, issue_number: u64, state: IssueState) -> Result<Issue> {
        let issue = self.client
            .issues(&self.owner, &self.repo)
            .update(issue_number)
            .state(state)
            .send()
            .await?;
        
        Ok(issue)
    }
    
    /// Add a comment to an issue
    pub async fn add_comment(&self, issue_number: u64, comment: &str) -> Result<()> {
        self.client
            .issues(&self.owner, &self.repo)
            .create_comment(issue_number, comment)
            .await?;
        
        Ok(())
    }
}
