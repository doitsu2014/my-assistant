#!/usr/bin/env python3
"""
Simple verification script to test bot structure without dependencies
"""

import sys
import os

def check_files():
    """Check that all required files exist"""
    required_files = [
        'bot.py',
        'config.py',
        'notion_integration.py',
        'github_integration.py',
        'requirements.txt',
        '.env.example',
        '.gitignore',
        'README.md'
    ]
    
    missing_files = []
    for file in required_files:
        if not os.path.exists(file):
            missing_files.append(file)
    
    if missing_files:
        print(f"❌ Missing files: {', '.join(missing_files)}")
        return False
    
    print("✓ All required files present")
    return True

def check_python_syntax():
    """Check Python files compile"""
    python_files = [
        'bot.py',
        'config.py',
        'notion_integration.py',
        'github_integration.py'
    ]
    
    for file in python_files:
        try:
            with open(file, 'r') as f:
                compile(f.read(), file, 'exec')
            print(f"✓ {file} syntax OK")
        except SyntaxError as e:
            print(f"❌ {file} has syntax error: {e}")
            return False
    
    return True

def check_requirements():
    """Check requirements.txt has necessary dependencies"""
    with open('requirements.txt', 'r') as f:
        content = f.read()
    
    required_packages = [
        'python-dotenv',
        'notion-client',
        'PyGithub'
    ]
    
    missing = []
    for pkg in required_packages:
        if pkg not in content:
            missing.append(pkg)
    
    if missing:
        print(f"❌ Missing packages in requirements.txt: {', '.join(missing)}")
        return False
    
    print("✓ All required packages in requirements.txt")
    return True

def main():
    """Run all checks"""
    print("=== OpenClaw Bot Structure Verification ===\n")
    
    checks = [
        ("File structure", check_files),
        ("Python syntax", check_python_syntax),
        ("Requirements", check_requirements)
    ]
    
    all_passed = True
    for name, check_func in checks:
        print(f"\n{name}:")
        if not check_func():
            all_passed = False
    
    print("\n" + "="*40)
    if all_passed:
        print("✓ All checks passed! Project structure is ready.")
        print("\nNext steps:")
        print("1. Install dependencies: pip install -r requirements.txt")
        print("2. Configure .env file based on .env.example")
        print("3. Run the bot: python bot.py")
        return 0
    else:
        print("❌ Some checks failed. Please fix the issues above.")
        return 1

if __name__ == "__main__":
    sys.exit(main())
