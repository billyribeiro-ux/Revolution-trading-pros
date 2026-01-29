#!/usr/bin/env python3
"""
Enhanced scan to find ALL remaining form field violations
Checks for: missing id, missing name, missing autocomplete on password/email fields
"""

import re
import os
from pathlib import Path
from typing import List, Tuple, Dict

def check_input_violations(file_path: str) -> List[Dict]:
    """Find all input violations in a file"""
    violations = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
        lines = content.split('\n')
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        if '<input' in line:
            # Collect full input tag
            input_tag = line
            line_start = i + 1
            
            while '>' not in input_tag and i < len(lines) - 1:
                i += 1
                input_tag += '\n' + lines[i]
            
            # Skip hidden inputs
            if 'hidden' in input_tag:
                i += 1
                continue
            
            # Check for violations
            has_id = re.search(r'\bid\s*=', input_tag)
            has_name = re.search(r'\bname\s*=', input_tag)
            has_autocomplete = re.search(r'\bautocomplete\s*=', input_tag)
            
            type_match = re.search(r'type\s*=\s*["\']([^"\']+)["\']', input_tag)
            input_type = type_match.group(1) if type_match else 'text'
            
            violation = {
                'line': line_start,
                'tag': input_tag[:100] + '...' if len(input_tag) > 100 else input_tag,
                'issues': []
            }
            
            # Check for missing id/name
            if not has_id and not has_name:
                violation['issues'].append('missing_id_and_name')
            elif not has_id:
                violation['issues'].append('missing_id')
            elif not has_name:
                violation['issues'].append('missing_name')
            
            # Check for missing autocomplete on sensitive fields
            if input_type in ['password', 'email', 'tel', 'username'] and not has_autocomplete:
                violation['issues'].append(f'missing_autocomplete_{input_type}')
            
            if violation['issues']:
                violations.append(violation)
        
        i += 1
    
    return violations

def scan_all_files(root_dir: str) -> Dict[str, List[Dict]]:
    """Scan all .svelte files"""
    results = {}
    
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if file.endswith('.svelte'):
                file_path = os.path.join(root, file)
                violations = check_input_violations(file_path)
                
                if violations:
                    rel_path = os.path.relpath(file_path, root_dir)
                    results[rel_path] = violations
    
    return results

if __name__ == "__main__":
    print("=" * 80)
    print("ENHANCED ACCESSIBILITY SCAN")
    print("Finding ALL remaining form field violations")
    print("=" * 80)
    print("")
    
    routes_dir = "./src/routes"
    results = scan_all_files(routes_dir)
    
    # Categorize violations
    missing_id_name = []
    missing_name_only = []
    missing_autocomplete = []
    
    for file_path, violations in results.items():
        for v in violations:
            if 'missing_id_and_name' in v['issues']:
                missing_id_name.append((file_path, v))
            elif 'missing_name' in v['issues']:
                missing_name_only.append((file_path, v))
            
            for issue in v['issues']:
                if 'missing_autocomplete' in issue:
                    missing_autocomplete.append((file_path, v, issue))
    
    print(f"📊 SUMMARY")
    print(f"Files with violations: {len(results)}")
    print(f"Inputs missing id AND name: {len(missing_id_name)}")
    print(f"Inputs missing name only: {len(missing_name_only)}")
    print(f"Inputs missing autocomplete: {len(missing_autocomplete)}")
    print("")
    
    if missing_id_name:
        print("=" * 80)
        print("❌ INPUTS MISSING BOTH ID AND NAME")
        print("=" * 80)
        for file_path, v in missing_id_name:
            print(f"\n📄 {file_path}:{v['line']}")
            print(f"   {v['tag']}")
    
    if missing_name_only:
        print("\n" + "=" * 80)
        print("⚠️  INPUTS MISSING NAME ATTRIBUTE")
        print("=" * 80)
        for file_path, v in missing_name_only:
            print(f"\n📄 {file_path}:{v['line']}")
            print(f"   {v['tag']}")
    
    if missing_autocomplete:
        print("\n" + "=" * 80)
        print("🔒 INPUTS MISSING AUTOCOMPLETE")
        print("=" * 80)
        for file_path, v, issue in missing_autocomplete:
            print(f"\n📄 {file_path}:{v['line']}")
            print(f"   Type: {issue.replace('missing_autocomplete_', '')}")
            print(f"   {v['tag']}")
    
    print("\n" + "=" * 80)
    print(f"Total violations to fix: {len(missing_id_name) + len(missing_name_only) + len(missing_autocomplete)}")
    print("=" * 80)
