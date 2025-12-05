#!/bin/bash
# ExoGenesis Omega - File Count Based Version Bumping
# Uses number of changed files to determine version bump
# NOTE: This is less accurate than conventional commits!

set -e

OMEGA_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$OMEGA_DIR"

echo "🤖 ExoGenesis Omega - File Count Version Analyzer"
echo "=================================================="
echo ""

# Get the latest tag
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
echo "📍 Latest tag: $LATEST_TAG"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep -E '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📦 Current version: $CURRENT_VERSION"
echo ""

# Count changed files since last tag
CHANGED_FILES=$(git diff --name-only $LATEST_TAG..HEAD 2>/dev/null || git diff --name-only)
FILE_COUNT=$(echo "$CHANGED_FILES" | grep -c . || echo "0")

if [[ $FILE_COUNT -eq 0 ]]; then
    echo "ℹ️  No files changed since last tag."
    echo "   No version bump needed."
    exit 0
fi

echo "📊 Files changed since $LATEST_TAG: $FILE_COUNT"
echo ""

# Show some of the changed files
echo "Changed files (first 10):"
echo "$CHANGED_FILES" | head -10
if [[ $FILE_COUNT -gt 10 ]]; then
    echo "... and $((FILE_COUNT - 10)) more"
fi
echo ""

# Determine version bump based on file count
# < 10 files = patch (hotfix)
# >= 10 files = minor (feature)
# Major versions are always manual

if [[ $FILE_COUNT -lt 10 ]]; then
    BUMP_TYPE="patch"
    REASON="Less than 10 files changed (hotfix/bugfix)"
else
    BUMP_TYPE="minor"
    REASON="10+ files changed (feature/enhancement)"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Analysis Result"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Files changed: $FILE_COUNT"
echo "Bump type:     $BUMP_TYPE"
echo "Reason:        $REASON"
echo ""
echo "⚠️  Note: For MAJOR version bumps (breaking changes),"
echo "   please use manual version bumping."
echo ""

# Calculate new version
IFS='.' read -r major minor patch <<< "$CURRENT_VERSION"

case $BUMP_TYPE in
    minor)
        minor=$((minor + 1))
        patch=0
        ;;
    patch)
        patch=$((patch + 1))
        ;;
esac

NEW_VERSION="$major.$minor.$patch"

echo "📦 Current version: $CURRENT_VERSION"
echo "📦 New version:     $NEW_VERSION"
echo ""

# Export for use in CI
echo "BUMP_TYPE=$BUMP_TYPE" >> "${GITHUB_OUTPUT:-/dev/null}"
echo "NEW_VERSION=$NEW_VERSION" >> "${GITHUB_OUTPUT:-/dev/null}"
echo "FILE_COUNT=$FILE_COUNT" >> "${GITHUB_OUTPUT:-/dev/null}"

# If running interactively, ask to apply
if [[ -t 0 ]]; then
    read -p "❓ Apply this version bump? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo ""
        echo "🔧 Applying version bump..."
        "$OMEGA_DIR/scripts/version-bump.sh" "$BUMP_TYPE"
        echo ""
        echo "✅ Version bumped to $NEW_VERSION"
    else
        echo ""
        echo "❌ Version bump cancelled"
    fi
else
    echo "✅ Analysis complete"
fi
