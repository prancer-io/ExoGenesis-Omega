#!/bin/bash
# ExoGenesis Omega - Automated Crates.io Publishing Script
# Publishes all crates in correct dependency order

set -e

OMEGA_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DRY_RUN=${DRY_RUN:-false}

echo "📦 ExoGenesis Omega - Crates.io Publishing Tool"
echo "================================================"
echo ""

# Check if logged into crates.io
if ! cargo login --help &>/dev/null; then
    echo "❌ Error: cargo not found"
    exit 1
fi

# Verify git status is clean
cd "$OMEGA_DIR"
if [[ -n $(git status --porcelain) ]]; then
    echo "⚠️  Warning: Git working directory has uncommitted changes"
    echo "   Uncommitted files:"
    git status --short
    echo ""
    read -p "❓ Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Publishing cancelled"
        exit 1
    fi
fi

# Get current version
CURRENT_VERSION=$(grep -E '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📦 Publishing version: $CURRENT_VERSION"
echo ""

# Crates in dependency order (dependencies first)
PUBLISH_ORDER=(
    "omega-core"
    "omega-persistence"
    "omega-agentdb"
    "omega-memory"
    "omega-loops"
    "omega-meta-sona"
    "omega-runtime"
)

# Publish function
publish_crate() {
    local crate_name=$1
    local crate_path="$OMEGA_DIR/crates/$crate_name"

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📦 Publishing: $crate_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    cd "$crate_path"

    # Run tests
    echo "🧪 Running tests..."
    if ! cargo test --release; then
        echo "❌ Tests failed for $crate_name"
        return 1
    fi

    # Check package
    echo "📋 Checking package..."
    if ! cargo package --allow-dirty; then
        echo "❌ Package check failed for $crate_name"
        return 1
    fi

    # Publish (or dry-run)
    if [[ "$DRY_RUN" == "true" ]]; then
        echo "🔍 [DRY RUN] Would publish $crate_name"
        cargo publish --dry-run --allow-dirty
    else
        echo "🚀 Publishing $crate_name to crates.io..."
        if cargo publish --allow-dirty; then
            echo "✅ Successfully published $crate_name"
            # Wait for crates.io to update (important for dependent crates)
            echo "⏳ Waiting 30 seconds for crates.io to update..."
            sleep 30
        else
            echo "❌ Failed to publish $crate_name"
            return 1
        fi
    fi

    echo ""
}

# Confirm publishing (skip in CI)
if [[ "$DRY_RUN" != "true" ]]; then
    echo "⚠️  This will publish ${#PUBLISH_ORDER[@]} crates to crates.io"
    echo "   Crates: ${PUBLISH_ORDER[*]}"
    echo ""

    # Skip confirmation if running in CI
    if [[ -n "$CI" ]] || [[ -n "$GITHUB_ACTIONS" ]]; then
        echo "✅ Running in CI - proceeding with publish"
        echo ""
    else
        read -p "❓ Proceed with publishing? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "❌ Publishing cancelled"
            exit 1
        fi
        echo ""
    fi
fi

# Publish each crate in order
FAILED_CRATES=()
for crate in "${PUBLISH_ORDER[@]}"; do
    if ! publish_crate "$crate"; then
        FAILED_CRATES+=("$crate")
        echo "⚠️  Continuing with next crate..."
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Publishing Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [[ ${#FAILED_CRATES[@]} -eq 0 ]]; then
    if [[ "$DRY_RUN" == "true" ]]; then
        echo "✅ Dry run completed successfully"
    else
        echo "✅ All crates published successfully!"
        echo ""
        echo "🎉 ExoGenesis Omega v$CURRENT_VERSION is live on crates.io!"
        echo ""
        echo "📝 Next steps:"
        echo "   1. Create GitHub release: gh release create v$CURRENT_VERSION"
        echo "   2. Announce on social media"
        echo "   3. Monitor crates.io downloads"
        echo "   4. Update documentation site"
    fi
else
    echo "❌ Some crates failed to publish:"
    for crate in "${FAILED_CRATES[@]}"; do
        echo "   - $crate"
    done
    exit 1
fi

echo ""
