/// Notion integration for OpenClaw Bot
use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct NotionIntegration {
    client: Client,
    api_key: String,
    database_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotionTask {
    pub id: String,
    pub properties: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct NotionQueryResponse {
    results: Vec<NotionTask>,
}

impl NotionIntegration {
    /// Initialize Notion client
    pub fn new(api_key: String, database_id: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            database_id,
        }
    }
    
    /// Retrieve tasks from Notion database
    pub fn get_tasks(&self) -> Result<Vec<NotionTask>> {
        let url = format!("https://api.notion.com/v1/databases/{}/query", self.database_id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&json!({}))
            .send()?;
        
        if !response.status().is_success() {
            let error_text = response.text()?;
            anyhow::bail!("Error retrieving tasks from Notion: {}", error_text);
        }
        
        let query_response: NotionQueryResponse = response.json()?;
        Ok(query_response.results)
    }
    
    /// Create a new task in Notion database
    pub fn create_task(&self, title: &str, description: Option<&str>, status: &str) -> Result<NotionTask> {
        let url = "https://api.notion.com/v1/pages";
        
        let mut properties = json!({
            "Name": {
                "title": [
                    {
                        "text": {
                            "content": title
                        }
                    }
                ]
            },
            "Status": {
                "select": {
                    "name": status
                }
            }
        });
        
        if let Some(desc) = description {
            properties["Description"] = json!({
                "rich_text": [
                    {
                        "text": {
                            "content": desc
                        }
                    }
                ]
            });
        }
        
        let body = json!({
            "parent": {
                "database_id": self.database_id
            },
            "properties": properties
        });
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?;
        
        if !response.status().is_success() {
            let error_text = response.text()?;
            anyhow::bail!("Error creating task in Notion: {}", error_text);
        }
        
        let task: NotionTask = response.json()?;
        Ok(task)
    }
    
    /// Update task status in Notion
    pub fn update_task_status(&self, page_id: &str, status: &str) -> Result<NotionTask> {
        let url = format!("https://api.notion.com/v1/pages/{}", page_id);
        
        let body = json!({
            "properties": {
                "Status": {
                    "select": {
                        "name": status
                    }
                }
            }
        });
        
        let response = self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?;
        
        if !response.status().is_success() {
            let error_text = response.text()?;
            anyhow::bail!("Error updating task status in Notion: {}", error_text);
        }
        
        let task: NotionTask = response.json()?;
        Ok(task)
    }
}
