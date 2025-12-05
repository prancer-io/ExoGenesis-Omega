# Version Guard Setup Complete ✅

## What Is Version Guard?

**Version Guard** is a CI/CD workflow that **blocks PRs from merging** unless the version has been bumped correctly based on conventional commits.

## How It Works

```
Developer creates PR
        ↓
Version Guard CI runs
        ↓
Checks if version bumped
        ↓
Analyzes conventional commits
        ↓
Validates bump is sufficient
        ↓
   ✅ PASS → Can merge
   ❌ FAIL → Must fix version
```

## Quick Example

### ✅ Correct PR Flow

```bash
# 1. Create branch and make changes
git checkout -b feat/quantum-support
vim omega/crates/omega-loops/src/quantum.rs

# 2. Commit with conventional format
git commit -m "feat(loops): add quantum processor"

# 3. Bump version BEFORE pushing
cd omega
./scripts/version-bump.sh minor  # feat: = minor bump

# 4. Commit version bump
git add -A
git commit -m "chore: bump version to 0.2.0"

# 5. Push and create PR
git push origin feat/quantum-support
gh pr create

# ✅ Version Guard passes → PR can merge
```

### ❌ Incorrect PR Flow (Will Fail)

```bash
# 1. Create branch and make changes
git checkout -b feat/quantum-support
vim omega/crates/omega-loops/src/quantum.rs

# 2. Commit and push WITHOUT bumping version
git commit -m "feat(loops): add quantum processor"
git push origin feat/quantum-support
gh pr create

# ❌ Version Guard fails → PR blocked
# Error: "Version not bumped! Required: minor"
```

## Key Features

### 1. Automatic Validation ✅

- Runs on every PR to main/develop
- Compares PR version vs base branch version
- Analyzes all commits in PR
- Determines required bump type

### 2. Conventional Commit Enforcement ✅

| Commit | Required Bump | Example |
|--------|---------------|---------|
| `fix:` | PATCH | 0.1.0 → 0.1.1 |
| `feat:` | MINOR | 0.1.0 → 0.2.0 |
| `feat!:` | MAJOR | 0.1.0 → 1.0.0 |

### 3. Smart Analysis ✅

- Multiple commits → highest bump wins
- Breaking changes → always MAJOR
- Can bump higher than required (OK)
- Cannot bump lower (FAIL)

### 4. Helpful Feedback ✅

On failure, posts PR comment with:
- What version bump is required
- Why it's required (commit analysis)
- Exact commands to fix
- Link to documentation

## Files Created

```
ExoGenesis-Omega/
├── .github/workflows/
│   ├── version-guard.yml              ✅ NEW - PR version check (ACTIVE)
│   ├── auto-release.yml               ✅ MODIFIED - Auto-versioning (DISABLED)
│   ├── auto-release-filecount.yml     (disabled)
│   ├── publish.yml                    (triggered by tags)
│   └── ci.yml                         (runs tests)
├── omega/
│   ├── scripts/
│   │   ├── version-guard.sh           ✅ NEW - Version validation script
│   │   ├── auto-version.sh            (for auto-release)
│   │   └── version-bump.sh            (manual bump)
│   └── docs/
│       ├── VERSION_GUARD.md           ✅ NEW - Complete guide
│       ├── CONVENTIONAL_COMMITS.md    (commit format guide)
│       └── VERSIONING_COMPARISON.md   (method comparison)
```

## Workflow States

### Version Guard (ACTIVE) ✅

- **Trigger**: Pull requests to main/develop
- **Purpose**: Enforce version bumps in PRs
- **Developer action**: Manual version bump required
- **Merge gate**: PR blocked if version not bumped

### Auto-Release (DISABLED) 🔴

- **Trigger**: None (commented out)
- **Purpose**: Fully automatic versioning
- **Developer action**: None
- **When to use**: Solo dev, no PR workflow

## Testing Version Guard

### Test Locally Before Pushing PR

```bash
cd omega

# Test against main branch
./scripts/version-guard.sh main

# Example output:
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 📊 Commit Analysis
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 💥 Breaking changes: 0
# ✨ Features:         1
# 🐛 Fixes:            0
# 📝 Other:            0
#
# 🎯 Required bump: minor
# 📦 Actual bump:   minor
#
# ✅ PASSED: Version bump is correct!
```

## Complete PR Workflow

```bash
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 1. Start Feature Branch
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

git checkout main
git pull origin main
git checkout -b feat/my-feature

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 2. Make Changes
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

vim omega/crates/omega-agentdb/src/lib.rs
# ... implement feature ...

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 3. Commit with Conventional Format
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

git add -A
git commit -m "feat(agentdb): add batch insert API

Enables inserting multiple vectors in a single operation.
Includes comprehensive tests and examples."

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 4. Test Version Guard Locally
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

cd omega
./scripts/version-guard.sh main

# Output shows: Required bump: minor

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 5. Bump Version
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

./scripts/version-bump.sh minor

# Updates version: 0.1.0 → 0.2.0

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 6. Commit Version Bump
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

git add -A
git commit -m "chore: bump version to 0.2.0"

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 7. Push and Create PR
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

git push origin feat/my-feature
gh pr create --title "feat(agentdb): add batch insert API"

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 8. GitHub Actions Runs
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# CI workflow: Tests, clippy, format ✅
# Version Guard: Validates version bump ✅

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 9. PR Approved and Merged
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# Merge to main with version 0.2.0

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 10. Manual Tag Creation (One-Time)
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# After PR merged, create release tag:
git checkout main
git pull
git tag -a v0.2.0 -m "Release v0.2.0"
git push --tags

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 11. Publish Workflow Triggers
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# publish.yml automatically:
# - Runs tests
# - Publishes to crates.io
# - Creates GitHub release
```

## When Version Guard Fails

You'll see a PR comment like this:

```
❌ Version Guard: FAILED

This PR must bump the version in omega/Cargo.toml before merging.

Why?
Every PR must increment the version according to conventional commits:
- fix: commits → PATCH bump (0.1.0 → 0.1.1)
- feat: commits → MINOR bump (0.1.0 → 0.2.0)
- feat!: or BREAKING CHANGE: → MAJOR bump (0.1.0 → 1.0.0)

How to fix:
cd omega
./scripts/version-bump.sh [patch|minor|major]
git add -A
git commit -m "chore: bump version"
git push
```

**Just follow the instructions and push again!**

## Benefits

### For Teams ✅

- **Enforces version discipline**: Every PR increments version
- **Prevents conflicts**: No two PRs with same version
- **Clear history**: Version changes tied to PRs
- **Automatic validation**: No manual review needed

### For Solo Developers ✅

- **Consistency**: Never forget to bump version
- **Semantic versioning**: Enforced automatically
- **Professional**: Follows best practices
- **Documentation**: Commit messages explain changes

### For Users ✅

- **Predictable releases**: Every merge = new version
- **Clear changelog**: Conventional commits → automatic changelog
- **Semantic versions**: Know what changed by version number
- **Trustworthy**: Follows semver strictly

## Comparison: Version Guard vs Auto-Release

| Feature | Version Guard | Auto-Release |
|---------|---------------|--------------|
| **Version bump** | Manual (by developer) | Automatic (by CI) |
| **When** | Before PR merge | After merge to main |
| **Validation** | Required check | No validation |
| **Best for** | Team projects | Solo developers |
| **Workflow** | PR-based | Direct push |
| **Control** | Developer decides | CI decides |
| **Active?** | ✅ YES | ❌ NO (disabled) |

**Version Guard is RECOMMENDED for ExoGenesis Omega** because:
- ✅ Team collaboration expected
- ✅ PR-based workflow
- ✅ Publishing to crates.io (public API)
- ✅ Breaking changes must be explicit
- ✅ Maintainer control over versions

## Configuration

### Enable Version Guard (Already Enabled ✅)

File: `.github/workflows/version-guard.yml`

Triggers on: Pull requests to main/develop

### Make it a Required Check

To block merges when Version Guard fails:

1. Go to: GitHub → Settings → Branches
2. Select: main branch
3. Enable: "Require status checks to pass"
4. Check: "Verify Version Bump"
5. Save

Now PRs **cannot merge** if version not bumped!

## FAQ

**Q: Do I need to bump version for EVERY PR?**
A: Yes, if the PR touches files in `omega/` directory.

**Q: What if I only changed documentation?**
A: Still bump version (PATCH). Docs are part of the release.

**Q: Can I skip Version Guard for hotfixes?**
A: No. Hotfixes should bump PATCH version.

**Q: What about merge conflicts in Cargo.toml?**
A: Take the **higher version** and re-run version guard.

**Q: Can I bump higher than required?**
A: Yes! MINOR when PATCH required is OK. MAJOR when MINOR required is OK.

**Q: How do I test Version Guard locally?**
A: `cd omega && ./scripts/version-guard.sh main`

## Next Steps

### 1. First PR with Version Guard

Try it out:

```bash
git checkout -b test/version-guard
echo "# Test" >> omega/README.md
git commit -m "docs: test version guard"
cd omega && ./scripts/version-bump.sh patch
git commit -am "chore: bump version"
git push origin test/version-guard
gh pr create
```

### 2. Make Version Guard Required

Configure branch protection (see Configuration above)

### 3. Team Onboarding

Share with team:
- `omega/docs/VERSION_GUARD.md` - Full guide
- `omega/docs/CONVENTIONAL_COMMITS.md` - Commit format
- This summary document

### 4. First Real Release

When ready to publish:

```bash
# After PR merged
git checkout main
git pull
git tag -a v0.1.0 -m "Release v0.1.0"
git push --tags

# Triggers automatic publishing to crates.io
```

## Summary

✅ **Version Guard is ACTIVE**
✅ **Enforces version bumps in PRs**
✅ **Uses conventional commits**
✅ **Blocks merge if version not bumped**
✅ **Provides helpful error messages**
✅ **Works with publish workflow**

**You're all set!** 🚀

Start creating PRs with conventional commits and version bumps.
Version Guard will validate everything automatically.
