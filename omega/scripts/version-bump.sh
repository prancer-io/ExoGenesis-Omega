#!/bin/bash
# ExoGenesis Omega - Automated Version Bumping Script
# Usage: ./scripts/version-bump.sh [major|minor|patch]

set -e

VERSION_TYPE=${1:-patch}
OMEGA_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "🚀 ExoGenesis Omega Version Bump Tool"
echo "======================================="
echo ""

# Function to get current version from workspace Cargo.toml
get_current_version() {
    grep -E '^version = ' "$OMEGA_DIR/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Function to bump version
bump_version() {
    local current=$1
    local bump_type=$2

    IFS='.' read -r major minor patch <<< "$current"

    case $bump_type in
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
        *)
            echo "❌ Invalid version type: $bump_type"
            echo "   Use: major, minor, or patch"
            exit 1
            ;;
    esac

    echo "$major.$minor.$patch"
}

# Get current version
CURRENT_VERSION=$(get_current_version)
echo "📦 Current version: $CURRENT_VERSION"

# Calculate new version
NEW_VERSION=$(bump_version "$CURRENT_VERSION" "$VERSION_TYPE")
echo "📦 New version: $NEW_VERSION"
echo ""

# Confirm with user
read -p "❓ Proceed with version bump to $NEW_VERSION? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Version bump cancelled"
    exit 1
fi

echo ""
echo "🔧 Updating version in workspace Cargo.toml..."

# Update workspace Cargo.toml
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$OMEGA_DIR/Cargo.toml"

# Update all crate dependencies that reference other omega crates
echo "🔧 Updating inter-crate dependencies..."

for crate_toml in "$OMEGA_DIR/crates/"*/Cargo.toml; do
    # Update omega-* dependencies to use new version
    sed -i "s/omega-\([a-z-]*\) = { version = \"[^\"]*\"/omega-\1 = { version = \"$NEW_VERSION\"/" "$crate_toml"
done

echo "✅ Version updated to $NEW_VERSION"
echo ""
echo "📝 Next steps:"
echo "   1. Review changes: git diff"
echo "   2. Run tests: cargo test --workspace"
echo "   3. Commit: git add -A && git commit -m \"chore: bump version to $NEW_VERSION\""
echo "   4. Tag: git tag -a v$NEW_VERSION -m \"Release v$NEW_VERSION\""
echo "   5. Push: git push && git push --tags"
echo ""
