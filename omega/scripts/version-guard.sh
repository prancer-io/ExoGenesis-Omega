#!/bin/bash
# ExoGenesis Omega - PR Version Guard
# Validates that PR has bumped version correctly based on conventional commits

set -e

OMEGA_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$OMEGA_DIR"

echo "🛡️  ExoGenesis Omega - PR Version Guard"
echo "========================================"
echo ""

# Get base branch (main) and current branch versions
BASE_BRANCH=${1:-main}
echo "📍 Comparing against: $BASE_BRANCH"

# Get current version from PR branch
CURRENT_VERSION=$(grep -E '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📦 PR branch version: $CURRENT_VERSION"

# Get base branch version
git fetch origin $BASE_BRANCH --quiet
BASE_VERSION=$(git show origin/$BASE_BRANCH:omega/Cargo.toml | grep -E '^version = ' | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📦 Base branch version: $BASE_VERSION"
echo ""

# Parse versions
IFS='.' read -r CURRENT_MAJOR CURRENT_MINOR CURRENT_PATCH <<< "$CURRENT_VERSION"
IFS='.' read -r BASE_MAJOR BASE_MINOR BASE_PATCH <<< "$BASE_VERSION"

# Check if version was bumped
VERSION_BUMPED=false
BUMP_TYPE="none"

if [[ $CURRENT_VERSION == $BASE_VERSION ]]; then
    echo "❌ FAILURE: Version not bumped!"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "The version in your PR must be bumped from the base branch."
    echo ""
    echo "Current: $CURRENT_VERSION"
    echo "Base:    $BASE_VERSION"
    echo ""
    echo "To fix this, run:"
    echo "  cd omega"
    echo "  ./scripts/version-bump.sh [patch|minor|major]"
    echo ""
    echo "Or let CI suggest the correct bump (see analysis below)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    # Continue to analyze what bump should be
    SHOULD_FAIL=true
elif [[ $CURRENT_MAJOR -gt $BASE_MAJOR ]]; then
    VERSION_BUMPED=true
    BUMP_TYPE="major"
    echo "✅ Version bumped: MAJOR ($BASE_VERSION → $CURRENT_VERSION)"
elif [[ $CURRENT_MAJOR -eq $BASE_MAJOR ]] && [[ $CURRENT_MINOR -gt $BASE_MINOR ]]; then
    VERSION_BUMPED=true
    BUMP_TYPE="minor"
    echo "✅ Version bumped: MINOR ($BASE_VERSION → $CURRENT_VERSION)"
elif [[ $CURRENT_MAJOR -eq $BASE_MAJOR ]] && [[ $CURRENT_MINOR -eq $BASE_MINOR ]] && [[ $CURRENT_PATCH -gt $BASE_PATCH ]]; then
    VERSION_BUMPED=true
    BUMP_TYPE="patch"
    echo "✅ Version bumped: PATCH ($BASE_VERSION → $CURRENT_VERSION)"
elif [[ $CURRENT_MAJOR -lt $BASE_MAJOR ]] || \
     ([[ $CURRENT_MAJOR -eq $BASE_MAJOR ]] && [[ $CURRENT_MINOR -lt $BASE_MINOR ]]) || \
     ([[ $CURRENT_MAJOR -eq $BASE_MAJOR ]] && [[ $CURRENT_MINOR -eq $BASE_MINOR ]] && [[ $CURRENT_PATCH -lt $BASE_PATCH ]]); then
    echo "❌ FAILURE: Version downgraded!"
    echo "   Base:    $BASE_VERSION"
    echo "   Current: $CURRENT_VERSION"
    echo ""
    echo "Version cannot go backwards. This should never happen!"
    exit 1
else
    echo "⚠️  WARNING: Unexpected version change"
    echo "   Base:    $BASE_VERSION"
    echo "   Current: $CURRENT_VERSION"
    exit 1
fi

echo ""

# Analyze commits to determine what bump SHOULD be
echo "🔍 Analyzing commits in this PR..."
echo ""

# Get commits between base and current
COMMITS=$(git log origin/$BASE_BRANCH..HEAD --oneline)
COMMIT_COUNT=$(echo "$COMMITS" | grep -c . || echo "0")

if [[ $COMMIT_COUNT -eq 0 ]]; then
    echo "ℹ️  No commits found in PR"
    if [[ "$VERSION_BUMPED" == "true" ]]; then
        echo "✅ Version bumped but no commits - this is OK (version bump commit)"
        exit 0
    else
        echo "ℹ️  No version bump needed"
        exit 0
    fi
fi

echo "📊 Analyzing $COMMIT_COUNT commits..."
echo ""

# Analyze commits for required bump type
NEEDS_MAJOR=false
NEEDS_MINOR=false
NEEDS_PATCH=false

BREAKING_COUNT=0
FEATURE_COUNT=0
FIX_COUNT=0

while IFS= read -r commit; do
    # Skip version bump commits
    if echo "$commit" | grep -qE "chore:.*bump.*version"; then
        continue
    fi

    # Check for BREAKING CHANGE
    FULL_COMMIT=$(git log -1 --format=%B $(echo "$commit" | awk '{print $1}'))

    if echo "$FULL_COMMIT" | grep -qE "BREAKING CHANGE:|!:"; then
        NEEDS_MAJOR=true
        ((BREAKING_COUNT++))
        echo "💥 BREAKING: $commit"
    elif echo "$commit" | grep -qE "^[a-f0-9]+ (feat|feature)(\(.+\))?!:"; then
        NEEDS_MAJOR=true
        ((BREAKING_COUNT++))
        echo "💥 BREAKING: $commit"
    elif echo "$commit" | grep -qE "^[a-f0-9]+ (feat|feature)(\(.+\))?:"; then
        NEEDS_MINOR=true
        ((FEATURE_COUNT++))
        echo "✨ FEATURE: $commit"
    elif echo "$commit" | grep -qE "^[a-f0-9]+ fix(\(.+\))?:"; then
        NEEDS_PATCH=true
        ((FIX_COUNT++))
        echo "🐛 FIX:     $commit"
    elif echo "$commit" | grep -qE "^[a-f0-9]+ perf(\(.+\))?:"; then
        NEEDS_MINOR=true
        echo "⚡ PERF:    $commit"
    elif echo "$commit" | grep -qE "^[a-f0-9]+ (chore|docs|refactor|test|ci)(\(.+\))?:"; then
        echo "📝 OTHER:   $commit"
    else
        # Non-conventional commit - require patch at minimum
        NEEDS_PATCH=true
        echo "❓ UNKNOWN: $commit (treating as patch)"
    fi
done <<< "$COMMITS"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Commit Analysis"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💥 Breaking changes: $BREAKING_COUNT"
echo "✨ Features:         $FEATURE_COUNT"
echo "🐛 Fixes:            $FIX_COUNT"
echo ""

# Determine required bump type (priority: major > minor > patch)
REQUIRED_BUMP="none"
if [[ "$NEEDS_MAJOR" == "true" ]]; then
    REQUIRED_BUMP="major"
elif [[ "$NEEDS_MINOR" == "true" ]]; then
    REQUIRED_BUMP="minor"
elif [[ "$NEEDS_PATCH" == "true" ]]; then
    REQUIRED_BUMP="patch"
else
    # Only chore/docs commits - patch is fine but not required
    REQUIRED_BUMP="patch"
fi

echo "🎯 Required bump: $REQUIRED_BUMP"
echo "📦 Actual bump:   $BUMP_TYPE"
echo ""

# Validate bump is sufficient
BUMP_SUFFICIENT=false

case "$REQUIRED_BUMP" in
    major)
        if [[ "$BUMP_TYPE" == "major" ]]; then
            BUMP_SUFFICIENT=true
        fi
        ;;
    minor)
        if [[ "$BUMP_TYPE" == "major" ]] || [[ "$BUMP_TYPE" == "minor" ]]; then
            BUMP_SUFFICIENT=true
        fi
        ;;
    patch)
        if [[ "$BUMP_TYPE" == "major" ]] || [[ "$BUMP_TYPE" == "minor" ]] || [[ "$BUMP_TYPE" == "patch" ]]; then
            BUMP_SUFFICIENT=true
        fi
        ;;
    none)
        # Any bump is sufficient
        BUMP_SUFFICIENT=true
        ;;
esac

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📋 Version Guard Result"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [[ "${SHOULD_FAIL:-false}" == "true" ]]; then
    echo "❌ FAILED: Version not bumped"
    echo ""
    echo "To fix:"
    echo "  cd omega"
    echo "  ./scripts/version-bump.sh $REQUIRED_BUMP"
    echo "  git add -A"
    echo "  git commit -m 'chore: bump version to <new_version>'"
    echo "  git push"
    echo ""
    exit 1
elif [[ "$BUMP_SUFFICIENT" == "true" ]]; then
    echo "✅ PASSED: Version bump is correct!"
    echo ""
    echo "Base version:     $BASE_VERSION"
    echo "PR version:       $CURRENT_VERSION"
    echo "Bump type:        $BUMP_TYPE"
    echo "Required minimum: $REQUIRED_BUMP"
    echo ""

    if [[ "$BUMP_TYPE" == "major" ]] && [[ "$REQUIRED_BUMP" != "major" ]]; then
        echo "ℹ️  NOTE: You bumped MAJOR but only $REQUIRED_BUMP was required."
        echo "   This is OK - you can bump higher than required."
    elif [[ "$BUMP_TYPE" == "minor" ]] && [[ "$REQUIRED_BUMP" == "patch" ]]; then
        echo "ℹ️  NOTE: You bumped MINOR but only PATCH was required."
        echo "   This is OK - you can bump higher than required."
    fi

    exit 0
else
    echo "❌ FAILED: Version bump is insufficient!"
    echo ""
    echo "Your PR requires: $REQUIRED_BUMP bump"
    echo "But you only did: $BUMP_TYPE bump"
    echo ""
    echo "Reason:"
    if [[ "$REQUIRED_BUMP" == "major" ]]; then
        echo "  💥 Breaking changes detected ($BREAKING_COUNT)"
        echo "     You must bump MAJOR version for breaking changes"
    elif [[ "$REQUIRED_BUMP" == "minor" ]]; then
        echo "  ✨ New features detected ($FEATURE_COUNT)"
        echo "     You must bump MINOR version for new features"
    fi
    echo ""
    echo "To fix:"
    echo "  cd omega"
    echo "  ./scripts/version-bump.sh $REQUIRED_BUMP"
    echo "  git add -A"
    echo "  git commit -m 'chore: bump version to match changes'"
    echo "  git push"
    echo ""
    exit 1
fi
