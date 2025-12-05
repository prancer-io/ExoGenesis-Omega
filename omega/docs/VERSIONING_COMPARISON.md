# Versioning Strategies Comparison

ExoGenesis Omega supports **three versioning strategies**. This document compares them to help you choose.

## TL;DR Recommendation

✅ **Use Conventional Commits** (Method 1) - Industry standard, most accurate

## Three Methods Available

### Method 1: Conventional Commits (Recommended ⭐)

**How it works**: Analyzes commit message prefixes

**Workflow**:
```bash
# You commit with semantic prefixes
git commit -m "feat: add quantum support"
git push

# GitHub Actions automatically:
# - Analyzes: "feat:" → minor bump
# - Bumps: 0.1.0 → 0.2.0
# - Tags and publishes
```

**Pros**:
- ✅ Most accurate (semantic meaning)
- ✅ Industry standard (Angular, Rust, Linux)
- ✅ Automatic changelog generation
- ✅ Clear intent in git history
- ✅ Distinguishes breaking changes
- ✅ Works for monorepos

**Cons**:
- ❌ Requires learning commit format
- ❌ Team must follow convention

**Use when**: Professional project, team collaboration

---

### Method 2: File Count Based

**How it works**: Counts changed files since last tag

**Workflow**:
```bash
# Change 8 files
git commit -m "various fixes"
git push

# GitHub Actions:
# - Counts: 8 files < 10 → patch bump
# - Bumps: 0.1.0 → 0.1.1
```

**Rules**:
- < 10 files = PATCH (0.1.0 → 0.1.1)
- ≥ 10 files = MINOR (0.1.0 → 0.2.0)
- Major always manual

**Pros**:
- ✅ Zero learning curve
- ✅ Fully automatic
- ✅ No commit format required

**Cons**:
- ❌ **Inaccurate**: 1 file can be breaking
- ❌ **Misleading**: 100 files can be non-breaking
- ❌ Can't detect breaking changes
- ❌ Documentation changes bump version
- ❌ No semantic meaning

**Use when**: Solo developer, rapid prototyping

---

### Method 3: Manual Versioning

**How it works**: You decide everything

**Workflow**:
```bash
# You manually choose
./scripts/version-bump.sh minor
git add -A
git commit -m "chore: bump to 0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
git push --tags
```

**Pros**:
- ✅ Full control
- ✅ No automation failures
- ✅ Custom versioning logic

**Cons**:
- ❌ Manual effort every release
- ❌ Easy to forget
- ❌ Inconsistent across team

**Use when**: Special releases, custom versioning needs

---

## Detailed Comparison

| Feature | Conventional Commits | File Count | Manual |
|---------|---------------------|------------|--------|
| **Accuracy** | ⭐⭐⭐⭐⭐ Excellent | ⭐⭐ Poor | ⭐⭐⭐⭐⭐ Perfect |
| **Automation** | ⭐⭐⭐⭐⭐ Full | ⭐⭐⭐⭐⭐ Full | ⭐ None |
| **Learning Curve** | ⭐⭐⭐ Medium | ⭐⭐⭐⭐⭐ None | ⭐⭐⭐⭐ Low |
| **Breaking Changes** | ⭐⭐⭐⭐⭐ Yes | ⭐ No | ⭐⭐⭐⭐⭐ Yes |
| **Changelog** | ⭐⭐⭐⭐⭐ Auto | ⭐ Manual | ⭐⭐ Manual |
| **Team Use** | ⭐⭐⭐⭐⭐ Great | ⭐⭐ Risky | ⭐⭐⭐ OK |
| **Monorepo** | ⭐⭐⭐⭐⭐ Yes | ⭐⭐ Limited | ⭐⭐⭐⭐ Yes |

## Real-World Examples

### Scenario 1: Breaking API Change

**Change**: Rename `Intelligence::create()` → `Intelligence::builder()`

**Conventional Commits**:
```bash
git commit -m "feat!: redesign Intelligence API

BREAKING CHANGE: Use Intelligence::builder() instead"
# Result: 0.1.0 → 1.0.0 ✅ CORRECT
```

**File Count**:
```bash
# Changed 1 file (src/intelligence.rs)
# Result: 0.1.0 → 0.1.1 ❌ WRONG (should be major)
```

**Winner**: Conventional Commits ✅

---

### Scenario 2: Documentation Update

**Change**: Update 50 README files with examples

**Conventional Commits**:
```bash
git commit -m "docs: add examples to all READMEs"
# Result: 0.1.0 → 0.1.1 ✅ CORRECT (patch for docs)
```

**File Count**:
```bash
# Changed 50 files
# Result: 0.1.0 → 0.2.0 ❌ WRONG (no new features)
```

**Winner**: Conventional Commits ✅

---

### Scenario 3: Bug Fix

**Change**: Fix memory leak in 1 critical file

**Conventional Commits**:
```bash
git commit -m "fix: prevent memory leak in consolidation"
# Result: 0.1.0 → 0.1.1 ✅ CORRECT
```

**File Count**:
```bash
# Changed 1 file
# Result: 0.1.0 → 0.1.1 ✅ CORRECT
```

**Winner**: Tie ✅

---

### Scenario 4: New Feature

**Change**: Add quantum loop processor (changes 12 files)

**Conventional Commits**:
```bash
git commit -m "feat: add quantum loop processor"
# Result: 0.1.0 → 0.2.0 ✅ CORRECT
```

**File Count**:
```bash
# Changed 12 files
# Result: 0.1.0 → 0.2.0 ✅ CORRECT
```

**Winner**: Tie ✅

---

## Accuracy Comparison

| Scenario | Conventional | File Count | Manual |
|----------|-------------|------------|--------|
| Breaking change (1 file) | ✅ Major | ❌ Patch | ✅ Major |
| 50 doc files updated | ✅ Patch | ❌ Minor | ✅ Patch |
| Bug fix (1 file) | ✅ Patch | ✅ Patch | ✅ Patch |
| Feature (12 files) | ✅ Minor | ✅ Minor | ✅ Minor |
| Refactor (30 files) | ✅ Patch | ❌ Minor | ✅ Patch |
| Performance (5 files) | ✅ Minor | ✅ Patch | ✅ Minor |

**Conventional Commits: 6/6 correct ✅**
**File Count: 2/6 correct (33%) ❌**
**Manual: 6/6 correct ✅**

## Which Should You Use?

### Use Conventional Commits if:
- ✅ Professional/commercial project
- ✅ Team collaboration
- ✅ Want automatic changelogs
- ✅ Following semver strictly
- ✅ Publishing to crates.io
- ✅ Open source project

### Use File Count if:
- ✅ Solo developer
- ✅ Rapid prototyping
- ✅ Internal tools only
- ✅ Don't care about accuracy
- ⚠️ **Not recommended for production**

### Use Manual if:
- ✅ Need full control
- ✅ Complex versioning rules
- ✅ Infrequent releases
- ✅ Custom release process

## How to Switch

### Enable Conventional Commits (Default)

Already enabled in `.github/workflows/auto-release.yml`

```bash
# Just commit with conventional format
git commit -m "feat: add new feature"
git push
```

### Enable File Count Based

1. Edit `.github/workflows/auto-release-filecount.yml`
2. Uncomment the `push: branches: - main` section
3. Disable auto-release.yml

### Disable Auto-Versioning (Manual Only)

1. Disable both workflows
2. Use scripts manually:
   ```bash
   ./scripts/version-bump.sh minor
   git add -A && git commit -m "chore: bump to 0.2.0"
   git tag -a v0.2.0 -m "Release"
   git push --tags
   ```

## Testing Locally

### Test Conventional Commits Analysis

```bash
cd omega
./scripts/auto-version.sh
```

### Test File Count Analysis

```bash
cd omega
./scripts/auto-version-filecount.sh
```

### Compare Both

```bash
echo "=== Conventional Commits ==="
./scripts/auto-version.sh

echo ""
echo "=== File Count ==="
./scripts/auto-version-filecount.sh
```

## Industry Standards

### Who Uses Conventional Commits?

- ✅ **Angular** - Created the standard
- ✅ **Rust** - Many Rust projects (serde, tokio, etc.)
- ✅ **Linux Kernel** - Similar format
- ✅ **Electron** - Desktop framework
- ✅ **Vue.js** - Frontend framework
- ✅ **Jest** - Testing framework

### Who Uses File Count?

- ❌ No major projects use this method
- ⚠️ Too inaccurate for serious projects

## Recommendation for ExoGenesis Omega

**Use Conventional Commits** because:

1. ✅ Publishing to crates.io (public API)
2. ✅ Following semver strictly required
3. ✅ Breaking changes must be clear
4. ✅ Professional open source project
5. ✅ Automatic changelogs needed
6. ✅ Team may contribute

## Migration Guide

### From Manual → Conventional Commits

1. Read [CONVENTIONAL_COMMITS.md](./CONVENTIONAL_COMMITS.md)
2. Start using conventional format
3. Auto-release workflow will activate
4. Delete manual version bump commits

### From File Count → Conventional Commits

1. Disable `auto-release-filecount.yml`
2. Enable `auto-release.yml`
3. Start using conventional commit format
4. First auto-release will use correct semver

## FAQ

**Q: Can I mix methods?**
A: No, choose one. Multiple auto-versioning workflows will conflict.

**Q: What if I forget conventional commit format?**
A: Workflow will treat as patch bump. You can force correct version manually.

**Q: Can I override auto-versioning?**
A: Yes, trigger workflow with manual bump type, or use manual versioning.

**Q: Do all commits need conventional format?**
A: No, but it's recommended. Non-conventional commits = patch bump.

**Q: What about merge commits?**
A: Auto-versioning analyzes all commits in merge, not merge commit message.

## Summary

| Method | Accuracy | Ease | Recommended |
|--------|----------|------|-------------|
| **Conventional Commits** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ✅ **YES** |
| **File Count** | ⭐⭐ | ⭐⭐⭐⭐⭐ | ❌ No |
| **Manual** | ⭐⭐⭐⭐⭐ | ⭐ | ⚠️ Special cases only |

**Choose Conventional Commits for ExoGenesis Omega.** 🚀
