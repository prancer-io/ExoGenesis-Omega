# Conventional Commits Guide for ExoGenesis Omega

## Overview

ExoGenesis Omega uses **Conventional Commits** for automatic version bumping and changelog generation. This guide explains the commit message format and how it affects versioning.

## TL;DR

```bash
# Patch version bump (0.1.0 → 0.1.1)
git commit -m "fix: correct memory consolidation bug"

# Minor version bump (0.1.0 → 0.2.0)
git commit -m "feat: add quantum loop processor"

# Major version bump (0.1.0 → 1.0.0)
git commit -m "feat!: redesign loop API

BREAKING CHANGE: Loop interface now requires async/await"
```

## Commit Message Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types and Version Bumps

| Type | Description | Version Bump | Example |
|------|-------------|--------------|---------|
| `fix:` | Bug fix | **PATCH** | `fix: prevent memory leak in consolidation` |
| `feat:` | New feature | **MINOR** | `feat: add support for hybrid loops` |
| `feat!:` | Breaking change | **MAJOR** | `feat!: change loop API signature` |
| `BREAKING CHANGE:` | Breaking change in footer | **MAJOR** | See example below |
| `perf:` | Performance improvement | **MINOR** | `perf: optimize SIMD vector operations` |
| `docs:` | Documentation only | **PATCH** | `docs: update API examples` |
| `chore:` | Maintenance | **PATCH** | `chore: update dependencies` |
| `refactor:` | Code refactoring | **PATCH** | `refactor: simplify loop coordinator` |
| `test:` | Adding tests | **PATCH** | `test: add memory tier tests` |
| `ci:` | CI/CD changes | **PATCH** | `ci: add coverage reporting` |

## Examples

### Patch Version (0.1.0 → 0.1.1)

**Bug Fixes**:
```bash
git commit -m "fix: correct off-by-one error in HNSW search"
git commit -m "fix(agentdb): handle empty vector datasets"
git commit -m "fix(memory): prevent duplicate consolidation"
```

**Chores/Docs**:
```bash
git commit -m "docs: add examples for omega-runtime"
git commit -m "chore: update Rust to 1.75"
git commit -m "test: increase memory coverage to 90%"
```

### Minor Version (0.1.0 → 0.2.0)

**New Features**:
```bash
git commit -m "feat: add distributed memory backend"
git commit -m "feat(loops): implement hybrid loop processor"
git commit -m "feat(meta-sona): add PPO optimization"
```

**Performance Improvements**:
```bash
git commit -m "perf: 3x speedup in vector similarity search"
git commit -m "perf(agentdb): implement SIMD acceleration"
```

### Major Version (0.1.0 → 1.0.0)

**Breaking Changes (Method 1 - Exclamation Mark)**:
```bash
git commit -m "feat!: redesign loop execution API

The loop API now requires async/await for all operations.
Migration guide added to docs/MIGRATION.md"
```

**Breaking Changes (Method 2 - Footer)**:
```bash
git commit -m "feat: redesign memory tier system

Add support for custom tier implementations.

BREAKING CHANGE: Memory::new() signature has changed.
Use Memory::builder() instead."
```

**Breaking Changes (Method 3 - Multiple)**:
```bash
git commit -m "refactor!: remove deprecated APIs

BREAKING CHANGE: Removed Intelligence::create_sync()
BREAKING CHANGE: Changed MemoryTier enum variants"
```

## Scopes

Scopes help identify which crate is affected:

```bash
git commit -m "feat(core): add new intelligence paradigm"
git commit -m "fix(agentdb): correct HNSW index rebuild"
git commit -m "perf(memory): optimize tier consolidation"
git commit -m "docs(runtime): add health monitoring examples"
```

**Valid scopes**:
- `core` - omega-core
- `agentdb` - omega-agentdb
- `memory` - omega-memory
- `loops` - omega-loops
- `meta-sona` - omega-meta-sona
- `runtime` - omega-runtime
- `persistence` - omega-persistence
- `ci` - CI/CD workflows
- `deps` - Dependencies

## Body and Footer

### Body (Optional)

Provide additional context:

```bash
git commit -m "feat(memory): add time-decay relevance scoring

Implements exponential decay for memory relevance based on
last access time. Default half-life is 24 hours.

Resolves #42"
```

### Footer (Optional)

Reference issues, breaking changes, co-authors:

```bash
git commit -m "fix(loops): prevent deadlock in coordinator

The coordinator could deadlock when multiple loops tried
to send messages simultaneously.

Fixes #123
Refs #124
```

## Automated Version Bumping

### How It Works

1. **You commit** with conventional format:
   ```bash
   git commit -m "feat: add quantum substrate support"
   git push origin main
   ```

2. **GitHub Actions analyzes** commits since last tag:
   - Finds: `feat:` → Minor bump needed
   - Calculates: 0.1.0 → 0.2.0

3. **Automatically**:
   - Updates `Cargo.toml` version
   - Updates inter-crate dependencies
   - Commits: `chore: bump version to 0.2.0`
   - Tags: `v0.2.0`
   - Pushes tag → Triggers publishing

### Manual Check

Test locally before pushing:

```bash
cd omega
./scripts/auto-version.sh
```

Output:
```
🤖 ExoGenesis Omega - Automatic Version Analyzer
==================================================

📍 Latest tag: v0.1.0
📦 Current version: 0.1.0

📊 Analyzing 3 commits since v0.1.0...

✨ FEATURE:  feat: add quantum substrate support
🐛 FIX:      fix: correct memory leak
📝 OTHER:    docs: update README

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Commit Analysis Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💥 Breaking changes: 0
✨ Features:         1
🐛 Fixes:            1
📝 Other:            1

🎯 Recommendation: minor version bump
   Reason: New features added

📦 Current version: 0.1.0
📦 New version:     0.2.0
```

## Changelog Generation

Conventional commits enable automatic changelog generation:

```bash
# Future feature: Auto-generate CHANGELOG.md
git log v0.1.0..HEAD --oneline | grep -E '^(feat|fix|perf):'
```

## Best Practices

### ✅ Good Commits

```bash
# Clear and specific
git commit -m "feat(agentdb): add batch insert API"

# Includes context
git commit -m "fix(memory): prevent race condition in consolidation

The consolidation loop could access freed memory when
multiple tiers consolidated simultaneously."

# Breaking change with migration
git commit -m "feat!: change loop execution to async

BREAKING CHANGE: All loop processors must now be async.
See docs/MIGRATION.md for upgrade guide."
```

### ❌ Bad Commits

```bash
# Too vague
git commit -m "fix stuff"

# No type
git commit -m "added new feature"

# Breaking change without marker
git commit -m "change API signature"  # Should be "feat!" or include BREAKING CHANGE

# Wrong type
git commit -m "feat: fix typo"  # Should be "fix:" or "docs:"
```

## Overriding Auto-Versioning

### Force Specific Version Type

Sometimes you need manual control:

```bash
# Trigger workflow manually with specific bump
# GitHub → Actions → Auto Release → Run workflow → Select bump type
```

Or use manual version bump:

```bash
# Skip auto-versioning entirely
./scripts/version-bump.sh major
git add -A
git commit -m "chore: bump to 1.0.0 for stable release"
git tag -a v1.0.0 -m "Release v1.0.0"
git push && git push --tags
```

## Commit Message Tips

1. **Use present tense**: "add" not "added"
2. **Be specific**: "fix memory leak in tier 4" not "fix bug"
3. **Reference issues**: "Fixes #123" in footer
4. **Explain why**: Use body for context
5. **Keep subject < 50 chars**: Detailed info goes in body
6. **No period at end**: `feat: add feature` not `feat: add feature.`

## Quick Reference

```bash
# Patch (0.1.0 → 0.1.1)
fix: ...
docs: ...
chore: ...
test: ...
refactor: ...

# Minor (0.1.0 → 0.2.0)
feat: ...
perf: ...

# Major (0.1.0 → 1.0.0)
feat!: ...
fix!: ...
BREAKING CHANGE: ...
```

## Commit Template

Create `.gitmessage` in your home directory:

```bash
cat > ~/.gitmessage << 'EOF'
# <type>[optional scope]: <description>
#
# [optional body]
#
# [optional footer(s)]
#
# Types: feat, fix, docs, chore, refactor, test, perf, ci
# Scopes: core, agentdb, memory, loops, meta-sona, runtime, persistence
#
# Breaking changes:
# - Add ! after type: feat!:
# - Or add footer: BREAKING CHANGE: description
EOF

git config --global commit.template ~/.gitmessage
```

## Resources

- **Specification**: https://www.conventionalcommits.org/
- **Examples**: https://www.conventionalcommits.org/en/v1.0.0/#examples
- **Tooling**: https://github.com/conventional-changelog/commitlint

## Integration with ExoGenesis Omega

### Workflows

1. **auto-release.yml**: Analyzes commits, bumps version, publishes
2. **publish.yml**: Triggered by version tags, publishes to crates.io
3. **ci.yml**: Runs tests on all commits

### Scripts

- `auto-version.sh`: Analyzes commits, recommends version bump
- `version-bump.sh`: Applies version bump
- `publish-crates.sh`: Publishes to crates.io

### Full Workflow

```bash
# 1. Make changes
vim omega/crates/omega-core/src/lib.rs

# 2. Commit with conventional format
git add -A
git commit -m "feat(core): add support for quantum paradigm"

# 3. Push to main
git push origin main

# 4. GitHub Actions automatically:
#    - Analyzes commit
#    - Bumps to 0.2.0
#    - Tags v0.2.0
#    - Publishes to crates.io
```

**Zero manual versioning required!** 🎉
