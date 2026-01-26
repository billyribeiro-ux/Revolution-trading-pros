#!/bin/bash

# Usage: ./retire_file.sh <file_path> <category> <reason>
# Example: ./retire_file.sh src/lib/components/OldButton.svelte components "Zero imports found"

FILE_PATH="$1"
CATEGORY="$2"
REASON="$3"
DATE=$(date +%Y-%m-%d)

if [ -z "$FILE_PATH" ] || [ -z "$CATEGORY" ] || [ -z "$REASON" ]; then
  echo "Usage: ./retire_file.sh <file_path> <category> <reason>"
  echo "Categories: components, routes, utils, stores, api, styles, assets, deprecated-syntax"
  exit 1
fi

if [ ! -f "$FILE_PATH" ]; then
  echo "Error: File not found: $FILE_PATH"
  exit 1
fi

# Create category directory if needed
mkdir -p "retired/$CATEGORY"

# Get filename
FILENAME=$(basename "$FILE_PATH")

# Preserve directory structure for nested files
RELATIVE_PATH=$(dirname "$FILE_PATH" | sed 's|^src/||')
mkdir -p "retired/$CATEGORY/$RELATIVE_PATH"

# Move file using git mv to preserve history
git mv "$FILE_PATH" "retired/$CATEGORY/$RELATIVE_PATH/$FILENAME" 2>/dev/null || mv "$FILE_PATH" "retired/$CATEGORY/$RELATIVE_PATH/$FILENAME"

echo "✅ Retired: $FILE_PATH -> retired/$CATEGORY/$RELATIVE_PATH/$FILENAME"
echo "   Reason: $REASON"
echo "   Date: $DATE"
