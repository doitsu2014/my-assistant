"""
GitHub integration for OpenClaw Bot
"""
from github import Github
from config import Config


class GitHubIntegration:
    """Handles GitHub API interactions for task and issue management"""
    
    def __init__(self):
        """Initialize GitHub client"""
        self.client = Github(Config.GITHUB_TOKEN)
        self.owner = Config.GITHUB_OWNER
        self.repo_name = Config.GITHUB_REPO
        self.repo = self.client.get_repo(f"{self.owner}/{self.repo_name}")
    
    def get_issues(self, state="open"):
        """
        Retrieve issues from GitHub repository
        
        Args:
            state (str): Issue state (open, closed, all)
            
        Returns:
            list: List of issues
        """
        try:
            issues = self.repo.get_issues(state=state)
            return list(issues)
        except Exception as e:
            print(f"Error retrieving issues from GitHub: {e}")
            return []
    
    def create_issue(self, title, body=None, labels=None, assignee=None):
        """
        Create a new issue in GitHub repository
        
        Args:
            title (str): Issue title
            body (str): Issue description
            labels (list): List of label names
            assignee (str): Username to assign
            
        Returns:
            Issue: Created issue object
        """
        try:
            kwargs = {"title": title}
            
            if body:
                kwargs["body"] = body
            if labels:
                kwargs["labels"] = labels
            if assignee:
                kwargs["assignee"] = assignee
            
            issue = self.repo.create_issue(**kwargs)
            return issue
        except Exception as e:
            print(f"Error creating issue in GitHub: {e}")
            return None
    
    def assign_issue(self, issue_number, assignee):
        """
        Assign an issue to a user
        
        Args:
            issue_number (int): Issue number
            assignee (str): Username to assign
            
        Returns:
            Issue: Updated issue object
        """
        try:
            issue = self.repo.get_issue(issue_number)
            issue.edit(assignee=assignee)
            return issue
        except Exception as e:
            print(f"Error assigning issue in GitHub: {e}")
            return None
    
    def update_issue_status(self, issue_number, state):
        """
        Update issue state (open/closed)
        
        Args:
            issue_number (int): Issue number
            state (str): New state (open or closed)
            
        Returns:
            Issue: Updated issue object
        """
        try:
            issue = self.repo.get_issue(issue_number)
            issue.edit(state=state)
            return issue
        except Exception as e:
            print(f"Error updating issue status in GitHub: {e}")
            return None
    
    def add_comment(self, issue_number, comment):
        """
        Add a comment to an issue
        
        Args:
            issue_number (int): Issue number
            comment (str): Comment text
            
        Returns:
            IssueComment: Created comment object
        """
        try:
            issue = self.repo.get_issue(issue_number)
            return issue.create_comment(comment)
        except Exception as e:
            print(f"Error adding comment to GitHub issue: {e}")
            return None
