"""
Configuration management for OpenClaw Bot
"""
import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()


class Config:
    """Configuration class for bot settings"""
    
    # Notion Configuration
    NOTION_API_KEY = os.getenv('NOTION_API_KEY', '')
    NOTION_DATABASE_ID = os.getenv('NOTION_DATABASE_ID', '')
    
    # GitHub Configuration
    GITHUB_TOKEN = os.getenv('GITHUB_TOKEN', '')
    GITHUB_OWNER = os.getenv('GITHUB_OWNER', '')
    GITHUB_REPO = os.getenv('GITHUB_REPO', '')
    
    # Bot Configuration
    BOT_NAME = os.getenv('BOT_NAME', 'OpenClaw Assistant')
    LOG_LEVEL = os.getenv('LOG_LEVEL', 'INFO')
    
    @classmethod
    def validate(cls):
        """Validate that required configuration is present"""
        missing = []
        
        if not cls.NOTION_API_KEY:
            missing.append('NOTION_API_KEY')
        if not cls.NOTION_DATABASE_ID:
            missing.append('NOTION_DATABASE_ID')
        if not cls.GITHUB_TOKEN:
            missing.append('GITHUB_TOKEN')
        if not cls.GITHUB_OWNER:
            missing.append('GITHUB_OWNER')
        if not cls.GITHUB_REPO:
            missing.append('GITHUB_REPO')
            
        return missing
