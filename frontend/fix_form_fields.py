#!/usr/bin/env python3
"""
Comprehensive Form Field Accessibility Fixer
Finds ALL input fields missing id/name attributes across entire codebase
Apple ICT Level 7 Standard
"""

import re
import os
from pathlib import Path
from typing import List, Tuple, Dict

def find_inputs_without_id_name(file_path: str) -> List[Tuple[int, str]]:
    """Find all input elements without id or name attributes"""
    violations = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Check if line contains <input
        if '<input' in line:
            # Collect the full input tag (might span multiple lines)
            input_tag = line
            line_start = i + 1
            
            # Keep reading until we find the closing >
            while '>' not in input_tag and i < len(lines) - 1:
                i += 1
                input_tag += lines[i]
            
            # Check if this input has id or name attribute
            has_id = re.search(r'\bid\s*=', input_tag)
            has_name = re.search(r'\bname\s*=', input_tag)
            
            # Skip if it's a hidden file input or already has id/name
            is_hidden = 'hidden' in input_tag
            
            if not has_id and not has_name and not is_hidden:
                violations.append((line_start, input_tag.strip()))
        
        i += 1
    
    return violations

def scan_directory(root_dir: str) -> Dict[str, List[Tuple[int, str]]]:
    """Scan all .svelte files in directory"""
    results = {}
    
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if file.endswith('.svelte'):
                file_path = os.path.join(root, file)
                violations = find_inputs_without_id_name(file_path)
                
                if violations:
                    rel_path = os.path.relpath(file_path, root_dir)
                    results[rel_path] = violations
    
    return results

def generate_report(results: Dict[str, List[Tuple[int, str]]]) -> str:
    """Generate detailed report of all violations"""
    report = []
    report.append("=" * 80)
    report.append("FORM FIELD ACCESSIBILITY VIOLATIONS REPORT")
    report.append("=" * 80)
    report.append("")
    
    total_violations = sum(len(v) for v in results.values())
    report.append(f"Total Files with Violations: {len(results)}")
    report.append(f"Total Input Fields Missing id/name: {total_violations}")
    report.append("")
    report.append("=" * 80)
    report.append("")
    
    for file_path, violations in sorted(results.items()):
        report.append(f"\n📄 {file_path}")
        report.append(f"   Violations: {len(violations)}")
        report.append("-" * 80)
        
        for line_num, input_tag in violations:
            report.append(f"   Line {line_num}:")
            # Show first 100 chars of input tag
            preview = input_tag[:100] + "..." if len(input_tag) > 100 else input_tag
            report.append(f"   {preview}")
            report.append("")
    
    return "\n".join(report)

if __name__ == "__main__":
    # Scan the routes directory
    routes_dir = "./src/routes"
    
    print("🔍 Scanning for input fields without id/name attributes...")
    print(f"📂 Directory: {routes_dir}")
    print("")
    
    results = scan_directory(routes_dir)
    
    # Generate report
    report = generate_report(results)
    
    # Save report
    report_file = "form_field_violations_report.txt"
    with open(report_file, 'w', encoding='utf-8') as f:
        f.write(report)
    
    print(report)
    print("")
    print(f"📝 Full report saved to: {report_file}")
    print("")
    print("=" * 80)
    print("PRIORITY FILES TO FIX:")
    print("=" * 80)
    
    # Show top 10 files with most violations
    sorted_files = sorted(results.items(), key=lambda x: len(x[1]), reverse=True)[:10]
    for i, (file_path, violations) in enumerate(sorted_files, 1):
        print(f"{i}. {file_path} ({len(violations)} violations)")
