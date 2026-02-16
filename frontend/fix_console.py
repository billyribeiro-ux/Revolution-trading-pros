#!/usr/bin/env python3
"""
Fix all console statements in the codebase
Apple Principal Engineer ICT Level 7+ Standards
"""

import re
import subprocess
from pathlib import Path

def get_files_with_console():
    """Get list of files with console statements from ESLint"""
    result = subprocess.run(
        ['npm', 'run', 'lint'],
        capture_output=True,
        text=True,
        cwd='/Users/billyribeiro/Documents/Revolution-trading-pros/frontend'
    )
    
    files = set()
    for line in result.stdout.split('\n') + result.stderr.split('\n'):
        if line.startswith('/Users/billyribeiro/Documents/Revolution-trading-pros/frontend/'):
            filepath = line.strip()
            # Convert to relative path
            rel_path = filepath.replace('/Users/billyribeiro/Documents/Revolution-trading-pros/frontend/', '')
            if rel_path and Path(rel_path).exists():
                files.add(rel_path)
    
    return sorted(files)

def has_logger_import(content):
    """Check if file already imports logger"""
    return "from '$lib/utils/logger'" in content or 'from "$lib/utils/logger"' in content

def add_logger_import(content):
    """Add logger import after last import statement"""
    lines = content.split('\n')
    
    # Find last import line
    last_import_idx = -1
    for i, line in enumerate(lines):
        if line.strip().startswith('import '):
            last_import_idx = i
    
    if last_import_idx >= 0:
        # Insert after last import
        lines.insert(last_import_idx + 1, "import { logger } from '$lib/utils/logger';")
    else:
        # No imports found, add at top
        lines.insert(0, "import { logger } from '$lib/utils/logger';")
    
    return '\n'.join(lines)

def replace_console_statements(content):
    """Replace all console statements with logger"""
    content = re.sub(r'console\.debug\(', 'logger.debug(', content)
    content = re.sub(r'console\.error\(', 'logger.error(', content)
    content = re.sub(r'console\.warn\(', 'logger.warn(', content)
    content = re.sub(r'console\.info\(', 'logger.info(', content)
    content = re.sub(r'console\.log\(', 'logger.info(', content)
    return content

def fix_file(filepath):
    """Fix a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if file has console statements
        if not re.search(r'console\.(debug|error|warn|info|log)\(', content):
            return False
        
        # Add logger import if needed
        if not has_logger_import(content):
            content = add_logger_import(content)
        
        # Replace console statements
        content = replace_console_statements(content)
        
        # Write back
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        return True
    except Exception as e:
        print(f"❌ Error fixing {filepath}: {e}")
        return False

def main():
    print("🎯 Agent 1 Extended: Fixing ALL console statements...")
    print("=" * 60)
    
    files = get_files_with_console()
    print(f"Found {len(files)} files with console statements\n")
    
    fixed = 0
    for filepath in files:
        if fix_file(filepath):
            fixed += 1
            print(f"✓ Fixed: {filepath}")
    
    print("\n" + "=" * 60)
    print(f"✅ Fixed {fixed} out of {len(files)} files")
    print("🎯 Done!")

if __name__ == '__main__':
    main()

