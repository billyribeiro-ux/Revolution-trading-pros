#!/usr/bin/env python3
import re

file_path = '/Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros/frontend/src/routes/alerts/spx-profit-pulse/+page.svelte'

with open(file_path, 'r') as f:
    content = f.read()

print(f"File size: {len(content)} characters")

# Check current state
if 'forEach((el)' in content:
    print("❌ Found buggy forEach loop")
else:
    print("✅ No forEach loop found")

if 'Hide ALL elements unconditionally' in content:
    print("✅ Found PE7 comment")
else:
    print("❌ PE7 comment not found")

# Find the exact location of the GSAP section
gsap_idx = content.find('// Only set initial hidden state')
if gsap_idx != -1:
    print(f"\nFound 'Only set initial' at index {gsap_idx}")
    print("Context:")
    print(repr(content[gsap_idx:gsap_idx+500]))
else:
    print("\n'Only set initial' not found")
    # Try without tabs
    gsap_idx2 = content.find('Only set initial hidden state')
    if gsap_idx2 != -1:
        print(f"Found without tabs at {gsap_idx2}")
        print(repr(content[gsap_idx2-50:gsap_idx2+200]))
