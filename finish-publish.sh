#!/bin/bash
# Finish publishing the remaining 2 crates after rate limit reset

set -e

echo "📦 Finishing ExoGenesis Omega v0.1.0 Publication"
echo "=============================================="
echo ""
echo "Publishing remaining crates:"
echo "  - omega-meta-sona v0.1.0"
echo "  - omega-runtime v0.1.0"
echo ""

cd "$(dirname "$0")"

# Publish omega-meta-sona
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📦 Publishing: omega-meta-sona"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cd crates/omega-meta-sona
echo "🧪 Running tests..."
cargo test --release
echo "🚀 Publishing to crates.io..."
cargo publish --allow-dirty
echo "✅ Successfully published omega-meta-sona"
echo "⏳ Waiting 30 seconds for crates.io to update..."
sleep 30
cd ../..

# Publish omega-runtime
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📦 Publishing: omega-runtime"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cd crates/omega-runtime
echo "🧪 Running tests..."
cargo test --release
echo "🚀 Publishing to crates.io..."
cargo publish --allow-dirty
echo "✅ Successfully published omega-runtime"
cd ../..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 ALL 7 CRATES PUBLISHED SUCCESSFULLY!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Published crates:"
echo "  ✅ omega-core v0.1.0"
echo "  ✅ omega-persistence v0.1.0"
echo "  ✅ omega-agentdb v0.1.0"
echo "  ✅ omega-memory v0.1.0"
echo "  ✅ omega-loops v0.1.0"
echo "  ✅ omega-meta-sona v0.1.0"
echo "  ✅ omega-runtime v0.1.0"
echo ""
echo "🔗 View on crates.io:"
echo "   https://crates.io/crates/omega-runtime"
echo ""
echo "📦 Test installation:"
echo "   cargo new test-project && cd test-project"
echo "   cargo add omega-runtime"
echo "   cargo build"
echo ""
