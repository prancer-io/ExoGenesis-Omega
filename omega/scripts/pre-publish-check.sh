#!/bin/bash
# ExoGenesis Omega - Pre-Publication Checks
# Validates everything is ready for crates.io publishing

set -e

OMEGA_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "🔍 ExoGenesis Omega - Pre-Publication Checklist"
echo "================================================"
echo ""

CHECKS_PASSED=0
CHECKS_FAILED=0

# Function to run a check
run_check() {
    local description=$1
    local command=$2

    echo -n "🔎 $description... "

    if eval "$command" &>/dev/null; then
        echo "✅"
        ((CHECKS_PASSED++))
        return 0
    else
        echo "❌"
        ((CHECKS_FAILED++))
        return 1
    fi
}

# Change to omega directory
cd "$OMEGA_DIR"

echo "🧪 Running Pre-Publication Checks"
echo "──────────────────────────────────"

# 1. Check Rust toolchain
run_check "Rust toolchain installed" "cargo --version"

# 2. Check git status
echo -n "🔎 Git working directory clean... "
if [[ -z $(git status --porcelain) ]]; then
    echo "✅"
    ((CHECKS_PASSED++))
else
    echo "⚠️  (uncommitted changes)"
    ((CHECKS_FAILED++))
fi

# 3. Check all tests pass
echo -n "🔎 Running test suite... "
if cargo test --workspace --release 2>&1 | grep -q "test result: ok"; then
    echo "✅"
    ((CHECKS_PASSED++))
else
    echo "❌"
    ((CHECKS_FAILED++))
fi

# 4. Check Clippy warnings
echo -n "🔎 Clippy checks... "
CLIPPY_OUTPUT=$(cargo clippy --workspace -- -W clippy::all 2>&1 || true)
CLIPPY_WARNINGS=$(echo "$CLIPPY_OUTPUT" | grep -c "^warning:" || echo "0")
if [[ $CLIPPY_WARNINGS -eq 0 ]]; then
    echo "✅"
    ((CHECKS_PASSED++))
else
    echo "⚠️  ($CLIPPY_WARNINGS warnings)"
    ((CHECKS_FAILED++))
fi

# 5. Check cargo package works for all crates
echo -n "🔎 Package validation... "
ALL_PACKAGE_OK=true
for crate_dir in "$OMEGA_DIR/crates/"*; do
    if [[ -d "$crate_dir" ]]; then
        cd "$crate_dir"
        if ! cargo package --quiet --allow-dirty &>/dev/null; then
            ALL_PACKAGE_OK=false
            break
        fi
    fi
done
cd "$OMEGA_DIR"

if $ALL_PACKAGE_OK; then
    echo "✅"
    ((CHECKS_PASSED++))
else
    echo "❌"
    ((CHECKS_FAILED++))
fi

# 6. Check documentation builds
echo -n "🔎 Documentation builds... "
if cargo doc --workspace --no-deps --quiet 2>&1 | grep -qv "error:"; then
    echo "✅"
    ((CHECKS_PASSED++))
else
    echo "❌"
    ((CHECKS_FAILED++))
fi

# 7. Check all Cargo.toml have required fields
echo -n "🔎 Crate metadata complete... "
METADATA_OK=true
for crate_toml in "$OMEGA_DIR/crates/"*/Cargo.toml; do
    if ! grep -q "^description = " "$crate_toml"; then
        METADATA_OK=false
        break
    fi
done

if $METADATA_OK; then
    echo "✅"
    ((CHECKS_PASSED++))
else
    echo "❌"
    ((CHECKS_FAILED++))
fi

# 8. Check README files exist
echo -n "🔎 README files present... "
README_COUNT=$(find "$OMEGA_DIR/crates" -name "README.md" | wc -l)
CRATE_COUNT=$(find "$OMEGA_DIR/crates" -maxdepth 1 -type d | tail -n +2 | wc -l)
if [[ $README_COUNT -eq $CRATE_COUNT ]]; then
    echo "✅"
    ((CHECKS_PASSED++))
else
    echo "⚠️  ($README_COUNT/$CRATE_COUNT)"
    ((CHECKS_FAILED++))
fi

# 9. Check LICENSE file exists
run_check "LICENSE file present" "test -f '$OMEGA_DIR/../LICENSE' -o -f '$OMEGA_DIR/LICENSE'"

# 10. Check version consistency
echo -n "🔎 Version consistency... "
WORKSPACE_VERSION=$(grep -E '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
VERSION_MISMATCH=false
for crate_toml in "$OMEGA_DIR/crates/"*/Cargo.toml; do
    if grep -q "omega-[a-z-]* = { version = " "$crate_toml"; then
        if grep "omega-[a-z-]* = { version = " "$crate_toml" | grep -qv "\"$WORKSPACE_VERSION\""; then
            VERSION_MISMATCH=true
            break
        fi
    fi
done

if ! $VERSION_MISMATCH; then
    echo "✅"
    ((CHECKS_PASSED++))
else
    echo "❌"
    ((CHECKS_FAILED++))
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Results"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Passed: $CHECKS_PASSED"
echo "❌ Failed: $CHECKS_FAILED"
echo ""

if [[ $CHECKS_FAILED -eq 0 ]]; then
    echo "🎉 All checks passed! Ready to publish to crates.io"
    echo ""
    echo "📝 Next steps:"
    echo "   1. Review version: $WORKSPACE_VERSION"
    echo "   2. Dry run: DRY_RUN=true ./scripts/publish-crates.sh"
    echo "   3. Publish: ./scripts/publish-crates.sh"
    exit 0
else
    echo "⚠️  Some checks failed. Please fix issues before publishing."
    exit 1
fi
