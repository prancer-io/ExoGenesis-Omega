# Quick Start: Publishing ExoGenesis Omega to Crates.io

This guide will help you publish all ExoGenesis Omega crates to crates.io in under 10 minutes.

## Prerequisites (2 minutes)

### 1. Get Your Crates.io API Token

```bash
# Visit: https://crates.io/settings/tokens
# Click "New Token"
# Give it a name: "ExoGenesis Omega Publishing"
# Copy the token (you'll only see it once!)
```

### 2. Login to Crates.io

```bash
cargo login <YOUR_TOKEN_HERE>
```

### 3. Set Up GitHub Secret (for CI/CD)

```
1. Go to: https://github.com/prancer-io/ExoGenesis-Omega/settings/secrets/actions
2. Click "New repository secret"
3. Name: CARGO_REGISTRY_TOKEN
4. Value: <YOUR_TOKEN_HERE>
5. Click "Add secret"
```

## Publishing Steps (5 minutes)

### Method 1: Automated (Recommended)

```bash
# Navigate to omega directory
cd omega

# Step 1: Run pre-publish checks (2 minutes)
./scripts/pre-publish-check.sh

# Step 2: Dry run to verify everything (1 minute)
DRY_RUN=true ./scripts/publish-crates.sh

# Step 3: Publish for real (2 minutes)
./scripts/publish-crates.sh
```

### Method 2: CI/CD (Even Easier!)

```bash
# Step 1: Bump version
./scripts/version-bump.sh patch

# Step 2: Commit and tag
git add -A
git commit -m "chore: bump version to 0.1.1"
git tag -a v0.1.1 -m "Release v0.1.1"

# Step 3: Push (triggers automated publishing)
git push && git push --tags

# GitHub Actions will handle the rest!
```

## What Happens During Publishing?

The script publishes crates in this order (with 30s delay between each):

1. ✅ **omega-core** (foundation types) - 10s
2. ✅ **omega-persistence** (storage) - 10s
3. ✅ **omega-agentdb** (vector DB) - 10s
4. ✅ **omega-memory** (memory system) - 10s
5. ✅ **omega-loops** (temporal loops) - 10s
6. ✅ **omega-meta-sona** (architecture search) - 10s
7. ✅ **omega-runtime** (orchestration) - 10s

**Total time**: ~4 minutes

## Verification (1 minute)

### Check Crates.io

```bash
# Search for your crates
cargo search omega-runtime
cargo search omega-core

# Or visit:
# https://crates.io/crates/omega-runtime
# https://crates.io/crates/omega-core
```

### Test Installation

```bash
# Create a test project
cargo new test-omega --bin
cd test-omega

# Add the runtime
cargo add omega-runtime

# Verify it compiles
cargo check
```

If you see "Compiling omega-runtime v0.1.0" - **SUCCESS!** 🎉

## Next Release

When you want to publish a new version:

```bash
# Bump version (patch/minor/major)
./scripts/version-bump.sh minor

# Review changes
git diff

# Commit, tag, and push
git add -A
git commit -m "chore: bump version to 0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
git push && git push --tags
```

GitHub Actions will automatically publish to crates.io!

## Troubleshooting

### "crate already exists"
**Solution**: Version already published. Bump version and try again.
```bash
./scripts/version-bump.sh patch
```

### "authentication required"
**Solution**: Login to crates.io
```bash
cargo login <YOUR_TOKEN>
```

### "dependency not found"
**Solution**: Wait 30-60 seconds and try again. Crates.io needs time to index.

### "rate limit exceeded"
**Solution**: Wait 1 hour before retrying.

## Support

- **Full Guide**: See [PUBLISHING.md](./PUBLISHING.md)
- **Issues**: https://github.com/prancer-io/ExoGenesis-Omega/issues
- **Discussions**: https://github.com/prancer-io/ExoGenesis-Omega/discussions

## Summary

✅ Get crates.io token
✅ Login: `cargo login`
✅ Check: `./scripts/pre-publish-check.sh`
✅ Publish: `./scripts/publish-crates.sh`
✅ Verify on crates.io
✅ Celebrate! 🎉

**Time**: ~8 minutes total
