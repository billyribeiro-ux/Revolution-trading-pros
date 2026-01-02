#!/bin/bash

# Script to add URL comments to all +page.svelte files
# Usage: ./add-url-comments.sh

find src/routes -name '+page.svelte' -type f | while read -r file; do
    # Extract the route path from the file path
    route=$(echo "$file" | sed 's|src/routes||' | sed 's|/+page.svelte||' | sed 's|^$|/|')
    
    # Skip if file already has URL comment on line 2 or 3
    if head -n 3 "$file" | grep -q "URL:"; then
        echo "Skipping $file (already has URL comment)"
        continue
    fi
    
    # Check first line to determine comment style
    first_line=$(head -n 1 "$file")
    
    if [[ "$first_line" == "<!--"* ]]; then
        # HTML comment style - insert after first line
        sed -i '' "1a\\
	URL: $route\\
" "$file"
    elif [[ "$first_line" == "<script"* ]]; then
        # Script tag first - insert URL comment before script
        sed -i '' "1i\\
<!--\\
	URL: $route\\
-->\\
" "$file"
    else
        # Other cases - prepend URL comment
        sed -i '' "1i\\
<!--\\
	URL: $route\\
-->\\
" "$file"
    fi
    
    echo "Added URL comment to: $file -> $route"
done

echo "Done! Added URL comments to all +page.svelte files."
