#!/usr/bin/env python3
"""
Batch fix for input fields missing id/name attributes
Generates unique IDs based on context
"""

import re
import os
from pathlib import Path

def generate_id_from_context(input_tag: str, line_num: int, file_context: str) -> tuple:
    """Generate meaningful id and name from input context"""
    
    # Extract type
    type_match = re.search(r'type\s*=\s*["\']([^"\']+)["\']', input_tag)
    input_type = type_match.group(1) if type_match else 'text'
    
    # Extract placeholder
    placeholder_match = re.search(r'placeholder\s*=\s*["\']([^"\']+)["\']', input_tag)
    placeholder = placeholder_match.group(1) if placeholder_match else ''
    
    # Extract bind:value variable name
    bind_match = re.search(r'bind:value\s*=\s*\{([^}]+)\}', input_tag)
    bind_var = bind_match.group(1) if bind_match else ''
    
    # Extract class for context
    class_match = re.search(r'class\s*=\s*["\']([^"\']+)["\']', input_tag)
    class_name = class_match.group(1) if class_match else ''
    
    # Generate ID based on available context
    if bind_var:
        # Use the bound variable name
        clean_var = bind_var.strip().replace('.', '-').replace('[', '-').replace(']', '')
        id_name = f"input-{clean_var}"
    elif placeholder:
        # Use placeholder text
        clean_placeholder = placeholder.lower().replace(' ', '-').replace('...', '')[:30]
        id_name = f"input-{clean_placeholder}"
    elif 'search' in class_name.lower() or 'search' in input_tag.lower():
        id_name = f"search-input-{line_num}"
    elif 'checkbox' in input_type:
        id_name = f"checkbox-{line_num}"
    elif 'radio' in input_type:
        id_name = f"radio-{line_num}"
    elif 'date' in input_type:
        id_name = f"date-input-{line_num}"
    else:
        id_name = f"{input_type}-input-{line_num}"
    
    # Clean up the ID
    id_name = re.sub(r'[^a-z0-9-]', '', id_name.lower())
    id_name = re.sub(r'-+', '-', id_name)
    id_name = id_name.strip('-')
    
    return id_name, id_name

def fix_input_tag(input_tag: str, id_name: str, name_attr: str) -> str:
    """Add id and name attributes to input tag"""
    
    # Check if already has id or name
    has_id = re.search(r'\bid\s*=', input_tag)
    has_name = re.search(r'\bname\s*=', input_tag)
    
    if has_id and has_name:
        return input_tag
    
    # Find the position after <input
    match = re.search(r'<input\s+', input_tag)
    if not match:
        match = re.search(r'<input\s*', input_tag)
    
    if match:
        insert_pos = match.end()
        attrs_to_add = []
        
        if not has_id:
            attrs_to_add.append(f'id="{id_name}"')
        if not has_name:
            attrs_to_add.append(f'name="{name_attr}"')
        
        attrs_str = ' '.join(attrs_to_add) + ' '
        fixed_tag = input_tag[:insert_pos] + attrs_str + input_tag[insert_pos:]
        return fixed_tag
    
    return input_tag

def fix_file(file_path: str, dry_run: bool = False) -> int:
    """Fix all inputs in a file"""
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
        lines = content.split('\n')
    
    fixed_count = 0
    i = 0
    new_lines = []
    
    while i < len(lines):
        line = lines[i]
        
        if '<input' in line and 'hidden' not in line:
            # Collect full input tag
            input_tag = line
            start_line = i
            
            while '>' not in input_tag and i < len(lines) - 1:
                i += 1
                input_tag += '\n' + lines[i]
            
            # Check if needs fixing
            has_id = re.search(r'\bid\s*=', input_tag)
            has_name = re.search(r'\bname\s*=', input_tag)
            
            if not has_id and not has_name:
                # Generate ID
                id_name, name_attr = generate_id_from_context(input_tag, start_line + 1, content)
                
                # Fix the tag
                fixed_tag = fix_input_tag(input_tag, id_name, name_attr)
                
                if fixed_tag != input_tag:
                    fixed_count += 1
                    print(f"  Line {start_line + 1}: Added id='{id_name}' name='{name_attr}'")
                    
                    # Replace in new_lines
                    tag_lines = fixed_tag.split('\n')
                    for j, tag_line in enumerate(tag_lines):
                        if start_line + j < len(new_lines):
                            new_lines[start_line + j] = tag_line
                        else:
                            new_lines.append(tag_line)
                    i = start_line + len(tag_lines) - 1
                else:
                    new_lines.append(line)
            else:
                new_lines.append(line)
        else:
            new_lines.append(line)
        
        i += 1
    
    if fixed_count > 0 and not dry_run:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(new_lines))
    
    return fixed_count

if __name__ == "__main__":
    import sys
    
    # High priority files
    priority_files = [
        "src/routes/admin/courses/create/+page.svelte",
        "src/routes/admin/consent/settings/+page.svelte",
        "src/routes/admin/boards/settings/+page.svelte",
        "src/routes/admin/indicators/create/+page.svelte",
        "src/routes/admin/blog/categories/+page.svelte",
        "src/routes/admin/crm/settings/+page.svelte",
    ]
    
    dry_run = '--dry-run' in sys.argv
    
    print("=" * 80)
    print("BATCH FIX: Adding id/name attributes to form fields")
    print("=" * 80)
    print("")
    
    total_fixed = 0
    
    for file_path in priority_files:
        if os.path.exists(file_path):
            print(f"\n📄 {file_path}")
            count = fix_file(file_path, dry_run)
            total_fixed += count
            print(f"   ✅ Fixed {count} inputs")
        else:
            print(f"\n⚠️  {file_path} - NOT FOUND")
    
    print("")
    print("=" * 80)
    print(f"Total inputs fixed: {total_fixed}")
    print("=" * 80)
