#!/bin/bash

# Script to replace all console statements with logger utility
# Apple Principal Engineer ICT Level 7+ Standards

set -e

echo "🎯 Agent 1 Extended: Fixing ALL console statements..."
echo "=================================================="

# Counter
fixed=0
total=0

# Read files from lint output
while IFS= read -r filepath; do
  # Extract relative path from absolute path
  file="${filepath#/Users/billyribeiro/Documents/Revolution-trading-pros/frontend/}"
  
  # Skip if file doesn't exist
  if [ ! -f "$file" ]; then
    continue
  fi
  
  ((total++))
  
  # Check if file already imports logger
  if ! grep -q "from '\$lib/utils/logger'" "$file" 2>/dev/null && \
     ! grep -q 'from "$lib/utils/logger"' "$file" 2>/dev/null; then
    
    # Determine import style based on file extension
    if [[ "$file" == *.ts ]] || [[ "$file" == *.svelte ]]; then
      # Find the last import statement and add logger import after it
      if grep -q "^import " "$file"; then
        # Add import after last import
        sed -i '' '/^import /!b;:a;n;/^import /ba;i\
import { logger } from '"'"'$lib/utils/logger'"'"';
' "$file"
      else
        # No imports, add at top after any comments
        sed -i '' '1i\
import { logger } from '"'"'$lib/utils/logger'"'"';\

' "$file"
      fi
    fi
  fi
  
  # Replace console statements
  sed -i '' 's/console\.debug(/logger.debug(/g' "$file"
  sed -i '' 's/console\.error(/logger.error(/g' "$file"
  sed -i '' 's/console\.warn(/logger.warn(/g' "$file"
  sed -i '' 's/console\.info(/logger.info(/g' "$file"
  sed -i '' 's/console\.log(/logger.info(/g' "$file"
  
  ((fixed++))
  echo "✓ Fixed: $file"
  
done < <(npm run lint 2>&1 | grep -E "^/" | sort -u)

echo ""
echo "=================================================="
echo "✅ Fixed $fixed out of $total files"
echo "🎯 Running lint to verify..."
echo ""

# Run lint again to check
npm run lint 2>&1 | tail -5

