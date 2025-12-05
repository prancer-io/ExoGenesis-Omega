# ExoGenesis Omega - Publishing Guide

This guide explains how to publish ExoGenesis Omega crates to crates.io with automated versioning and CI/CD.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Version Management](#version-management)
- [Publishing Process](#publishing-process)
- [Automated Publishing (CI/CD)](#automated-publishing-cicd)
- [Post-Publication](#post-publication)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### 1. Crates.io Account

1. Create an account at [crates.io](https://crates.io/)
2. Generate an API token: https://crates.io/settings/tokens
3. Save your token securely

### 2. Local Authentication

```bash
# Login to crates.io
cargo login YOUR_API_TOKEN
```

### 3. GitHub Repository Setup

For automated publishing, configure GitHub secrets:

```
Settings → Secrets and variables → Actions → New repository secret
```

Add:
- `CARGO_REGISTRY_TOKEN`: Your crates.io API token

## Version Management

### Automated Version Bumping

We provide a script for automated version bumping:

```bash
# Patch version (0.1.0 → 0.1.1)
./scripts/version-bump.sh patch

# Minor version (0.1.0 → 0.2.0)
./scripts/version-bump.sh minor

# Major version (0.1.0 → 1.0.0)
./scripts/version-bump.sh major
```

This script:
- Updates workspace `Cargo.toml` version
- Updates all inter-crate dependencies
- Provides next steps for committing and tagging

### Manual Version Management

1. Update version in `/omega/Cargo.toml`:
   ```toml
   [workspace.package]
   version = "0.2.0"
   ```

2. Update inter-crate dependencies in each crate's `Cargo.toml`:
   ```toml
   omega-core = { version = "0.2.0", path = "../omega-core" }
   ```

3. Commit and tag:
   ```bash
   git add -A
   git commit -m "chore: bump version to 0.2.0"
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push && git push --tags
   ```

## Publishing Process

### Step 1: Pre-Publication Checks

Run comprehensive checks before publishing:

```bash
cd omega
./scripts/pre-publish-check.sh
```

This validates:
- ✅ Rust toolchain installed
- ✅ Git working directory clean
- ✅ All tests pass
- ✅ No Clippy warnings
- ✅ Package validation
- ✅ Documentation builds
- ✅ Metadata completeness
- ✅ README files present
- ✅ LICENSE file exists
- ✅ Version consistency

### Step 2: Dry Run

Test the publishing process without actually publishing:

```bash
cd omega
DRY_RUN=true ./scripts/publish-crates.sh
```

This simulates the full publishing workflow.

### Step 3: Publish to Crates.io

Execute the publishing script:

```bash
cd omega
./scripts/publish-crates.sh
```

The script will:
1. ✅ Verify git status
2. 📦 Display version to be published
3. ❓ Ask for confirmation
4. 🧪 Run tests for each crate
5. 📋 Check package for each crate
6. 🚀 Publish each crate in dependency order:
   - omega-core
   - omega-persistence
   - omega-agentdb
   - omega-memory
   - omega-loops
   - omega-meta-sona
   - omega-runtime
7. ⏳ Wait 30 seconds between crates for indexing

### Publishing Order (Critical!)

Crates **MUST** be published in dependency order:

```
omega-core          # No dependencies
omega-persistence   # No dependencies
omega-agentdb       # No dependencies
    ↓
omega-memory        # Depends on: omega-core
omega-loops         # Depends on: omega-core
omega-meta-sona     # Depends on: omega-core
    ↓
omega-runtime       # Depends on: all above
```

The `publish-crates.sh` script handles this automatically.

## Automated Publishing (CI/CD)

### GitHub Actions Workflow

Publishing is automated via GitHub Actions when you push a version tag.

#### Trigger Automated Publishing

```bash
# 1. Bump version
./scripts/version-bump.sh minor

# 2. Review changes
git diff

# 3. Commit and tag
git add -A
git commit -m "chore: bump version to 0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"

# 4. Push (triggers CI/CD)
git push && git push --tags
```

#### Workflow Steps

The `.github/workflows/publish.yml` workflow:

1. **Pre-checks Job**:
   - Runs all pre-publication checks
   - Validates tests pass
   - Checks Clippy and formatting

2. **Publish Job** (on tag push):
   - Extracts version from tag
   - Verifies version matches `Cargo.toml`
   - Logs into crates.io
   - Publishes all crates in order
   - Creates GitHub Release

3. **Post-publish Job**:
   - Waits for crates.io indexing
   - Verifies all crates published
   - Tests installation

#### Manual Trigger (Dry Run)

You can manually trigger a dry run from GitHub:

```
Actions → Publish to crates.io → Run workflow
✓ dry_run: true
```

## Post-Publication

### 1. Verify Publication

Check crates.io:
```bash
cargo search omega-runtime
```

Visit crates:
- https://crates.io/crates/omega-core
- https://crates.io/crates/omega-runtime
- etc.

### 2. Test Installation

Create a new project and test:
```bash
cargo new test-omega --bin
cd test-omega
cargo add omega-runtime
cargo check
```

### 3. Update Documentation

- Update main README.md with latest version
- Create release notes in CHANGELOG.md
- Update docs.rs documentation links

### 4. Announce Release

- GitHub Releases (created automatically by CI/CD)
- Social media announcements
- Community forums

### 5. Monitor

- Crates.io download statistics
- GitHub issues for bug reports
- docs.rs build status

## Troubleshooting

### Issue: "crate already exists"

**Cause**: Version already published to crates.io

**Solution**: Bump version and try again
```bash
./scripts/version-bump.sh patch
```

### Issue: "failed to verify package tarball"

**Cause**: Package validation failed

**Solution**: Run local check
```bash
cd crates/omega-core
cargo package --list
cargo package --allow-dirty
```

### Issue: "dependency not found on crates.io"

**Cause**: Publishing in wrong order or crates.io not indexed yet

**Solution**:
- Ensure publishing in dependency order
- Wait 30-60 seconds between dependent crates
- Use the automated script which handles this

### Issue: "authentication required"

**Cause**: Not logged into crates.io

**Solution**:
```bash
cargo login YOUR_API_TOKEN
```

### Issue: "rate limit exceeded"

**Cause**: Too many publish attempts

**Solution**: Wait 1 hour before retrying

### Issue: Tests fail on Windows/macOS

**Cause**: Platform-specific issues

**Solution**: Check CI logs for specific errors
```bash
# Local cross-platform testing
cargo test --target x86_64-pc-windows-msvc
cargo test --target x86_64-apple-darwin
```

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2025-12-05 | Initial release |

## Support

- **Issues**: https://github.com/prancer-io/ExoGenesis-Omega/issues
- **Discussions**: https://github.com/prancer-io/ExoGenesis-Omega/discussions
- **Email**: omega-support@prancer.io

## License

MIT License - see [LICENSE](../../LICENSE) for details
