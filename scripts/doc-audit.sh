#!/bin/bash
# Audit all markdown documentation in the repository
#
# Usage:
#   ./scripts/doc-audit.sh
#
# Shows: filename, line count, and last modified date for all .md files

set -e

echo "📚 Documentation Audit"
echo "=" | awk '{s=sprintf("%80s",""); gsub(/ /,"=",$0); print}'
echo

# Header
printf "%-40s %10s  %s\n" "FILE" "LINES" "LAST MODIFIED"
echo "-" | awk '{s=sprintf("%80s",""); gsub(/ /,"-",$0); print}'

# Find all .md files, get stats, sort by path
find . -name "*.md" -not -path "*/node_modules/*" -not -path "*/target/*" -not -path "*/.git/*" | sort | while read -r file; do
    # Get line count
    lines=$(wc -l < "$file" | tr -d ' ')

    # Get last modified date (macOS compatible)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        mod_date=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M" "$file")
    else
        # Linux
        mod_date=$(stat -c "%y" "$file" | cut -d'.' -f1 | sed 's/ /  /')
    fi

    # Clean up path (remove leading ./)
    clean_path="${file#./}"

    printf "%-40s %10s  %s\n" "$clean_path" "$lines" "$mod_date"
done

echo
echo "=" | awk '{s=sprintf("%80s",""); gsub(/ /,"=",$0); print}'

# Summary stats
total_files=$(find . -name "*.md" -not -path "*/node_modules/*" -not -path "*/target/*" -not -path "*/.git/*" | wc -l | tr -d ' ')
total_lines=$(find . -name "*.md" -not -path "*/node_modules/*" -not -path "*/target/*" -not -path "*/.git/*" -exec wc -l {} + | tail -1 | awk '{print $1}')

echo "Total markdown files: $total_files"
echo "Total lines: $total_lines"
echo
