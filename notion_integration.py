"""
Notion integration for OpenClaw Bot
"""
from notion_client import Client
from config import Config


class NotionIntegration:
    """Handles Notion API interactions for planning and task management"""
    
    def __init__(self):
        """Initialize Notion client"""
        self.client = Client(auth=Config.NOTION_API_KEY)
        self.database_id = Config.NOTION_DATABASE_ID
    
    def get_tasks(self):
        """
        Retrieve tasks from Notion database
        
        Returns:
            list: List of tasks from Notion
        """
        try:
            response = self.client.databases.query(
                database_id=self.database_id
            )
            return response.get('results', [])
        except Exception as e:
            print(f"Error retrieving tasks from Notion: {e}")
            return []
    
    def create_task(self, title, description=None, status="Not Started"):
        """
        Create a new task in Notion database
        
        Args:
            title (str): Task title
            description (str): Task description
            status (str): Task status
            
        Returns:
            dict: Created task data
        """
        try:
            properties = {
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
            }
            
            if description:
                properties["Description"] = {
                    "rich_text": [
                        {
                            "text": {
                                "content": description
                            }
                        }
                    ]
                }
            
            response = self.client.pages.create(
                parent={"database_id": self.database_id},
                properties=properties
            )
            return response
        except Exception as e:
            print(f"Error creating task in Notion: {e}")
            return None
    
    def update_task_status(self, page_id, status):
        """
        Update task status in Notion
        
        Args:
            page_id (str): Page ID to update
            status (str): New status
            
        Returns:
            dict: Updated task data
        """
        try:
            response = self.client.pages.update(
                page_id=page_id,
                properties={
                    "Status": {
                        "select": {
                            "name": status
                        }
                    }
                }
            )
            return response
        except Exception as e:
            print(f"Error updating task status in Notion: {e}")
            return None
