#!/bin/bash
# ExoGenesis Omega - Automatic Version Bumping
# Analyzes git commits since last tag to determine version bump

set -e

OMEGA_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$OMEGA_DIR"

echo "🤖 ExoGenesis Omega - Automatic Version Analyzer"
echo "=================================================="
echo ""

# Get the latest tag
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
echo "📍 Latest tag: $LATEST_TAG"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep -E '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📦 Current version: $CURRENT_VERSION"
echo ""

# Get commits since last tag
COMMITS_SINCE_TAG=$(git log $LATEST_TAG..HEAD --oneline 2>/dev/null || git log --oneline)
COMMIT_COUNT=$(echo "$COMMITS_SINCE_TAG" | grep -c . || echo "0")

if [[ $COMMIT_COUNT -eq 0 ]]; then
    echo "ℹ️  No new commits since last tag."
    echo "   No version bump needed."
    exit 0
fi

echo "📊 Analyzing $COMMIT_COUNT commits since $LATEST_TAG..."
echo ""

# Initialize version bump flags
NEEDS_MAJOR=false
NEEDS_MINOR=false
NEEDS_PATCH=false

# Conventional Commit patterns
BREAKING_PATTERN="BREAKING CHANGE:|!:"
FEATURE_PATTERN="^feat(\(.+\))?:"
FIX_PATTERN="^fix(\(.+\))?:"
CHORE_PATTERN="^chore(\(.+\))?:"
DOCS_PATTERN="^docs(\(.+\))?:"
REFACTOR_PATTERN="^refactor(\(.+\))?:"
PERF_PATTERN="^perf(\(.+\))?:"

# Count by type
BREAKING_COUNT=0
FEATURE_COUNT=0
FIX_COUNT=0
CHORE_COUNT=0

# Analyze commits
while IFS= read -r commit; do
    # Check for BREAKING CHANGE
    if echo "$commit" | grep -qE "$BREAKING_PATTERN"; then
        NEEDS_MAJOR=true
        ((BREAKING_COUNT++))
        echo "💥 BREAKING: $commit"
    # Check for features
    elif echo "$commit" | grep -qE "$FEATURE_PATTERN"; then
        NEEDS_MINOR=true
        ((FEATURE_COUNT++))
        echo "✨ FEATURE:  $commit"
    # Check for fixes
    elif echo "$commit" | grep -qE "$FIX_PATTERN"; then
        NEEDS_PATCH=true
        ((FIX_COUNT++))
        echo "🐛 FIX:      $commit"
    # Performance improvements count as minor
    elif echo "$commit" | grep -qE "$PERF_PATTERN"; then
        NEEDS_MINOR=true
        echo "⚡ PERF:     $commit"
    # Other types
    elif echo "$commit" | grep -qE "$CHORE_PATTERN|$DOCS_PATTERN|$REFACTOR_PATTERN"; then
        ((CHORE_COUNT++))
        echo "📝 OTHER:    $commit"
    else
        # Non-conventional commit - treat as patch
        NEEDS_PATCH=true
        echo "❓ UNKNOWN:  $commit"
    fi
done <<< "$COMMITS_SINCE_TAG"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Commit Analysis Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💥 Breaking changes: $BREAKING_COUNT"
echo "✨ Features:         $FEATURE_COUNT"
echo "🐛 Fixes:            $FIX_COUNT"
echo "📝 Other:            $CHORE_COUNT"
echo ""

# Determine version bump type (priority: major > minor > patch)
if [[ "$NEEDS_MAJOR" == "true" ]]; then
    BUMP_TYPE="major"
    REASON="Breaking changes detected"
elif [[ "$NEEDS_MINOR" == "true" ]]; then
    BUMP_TYPE="minor"
    REASON="New features added"
elif [[ "$NEEDS_PATCH" == "true" ]]; then
    BUMP_TYPE="patch"
    REASON="Bug fixes or minor changes"
else
    echo "ℹ️  Only non-functional changes (docs, chore)."
    echo "   Recommending patch version bump for release."
    BUMP_TYPE="patch"
    REASON="Documentation or maintenance changes"
fi

echo "🎯 Recommendation: $BUMP_TYPE version bump"
echo "   Reason: $REASON"
echo ""

# Calculate new version
IFS='.' read -r major minor patch <<< "$CURRENT_VERSION"

case $BUMP_TYPE in
    major)
        major=$((major + 1))
        minor=0
        patch=0
        ;;
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
echo "CURRENT_VERSION=$CURRENT_VERSION" >> "${GITHUB_OUTPUT:-/dev/null}"

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
        echo ""
        echo "📝 Next steps:"
        echo "   git add -A"
        echo "   git commit -m 'chore: bump version to $NEW_VERSION'"
        echo "   git tag -a v$NEW_VERSION -m 'Release v$NEW_VERSION'"
        echo "   git push && git push --tags"
    else
        echo ""
        echo "❌ Version bump cancelled"
    fi
else
    # Running in CI - just output the decision
    echo "✅ Analysis complete"
    echo "   Recommended bump: $BUMP_TYPE"
    echo "   New version: $NEW_VERSION"
fi
