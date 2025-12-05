# Version Guard - PR Version Bump Enforcement

## Overview

The **Version Guard** workflow ensures every PR bumps the version in `Cargo.toml` before merging. It validates that the version bump matches the type of changes (based on conventional commits).

## How It Works

### On Every PR:

1. **Version Guard workflow runs**
2. **Compares versions**: PR branch vs base branch (main)
3. **Analyzes commits**: Determines required bump type
4. **Validates bump**: Checks if version bump is sufficient
5. **Pass or Fail**:
   - ✅ Pass: Version bumped correctly → PR can merge
   - ❌ Fail: Version not bumped or insufficient → PR blocked

## Quick Example

### ✅ Good PR (Will Pass)

```bash
# 1. Create feature branch
git checkout -b feat/add-quantum-support

# 2. Make changes
vim omega/crates/omega-loops/src/quantum.rs

# 3. Commit with conventional format
git commit -m "feat(loops): add quantum loop processor"

# 4. Bump version BEFORE pushing
cd omega
./scripts/version-bump.sh minor  # feat: requires minor bump

# 5. Commit version bump
git add -A
git commit -m "chore: bump version to 0.2.0"

# 6. Push and create PR
git push origin feat/add-quantum-support
```

**Result**: ✅ Version Guard passes, PR can be merged

---

### ❌ Bad PR (Will Fail)

```bash
# 1. Create feature branch
git checkout -b feat/add-quantum-support

# 2. Make changes
vim omega/crates/omega-loops/src/quantum.rs

# 3. Commit
git commit -m "feat(loops): add quantum loop processor"

# 4. Push WITHOUT bumping version
git push origin feat/add-quantum-support
```

**Result**: ❌ Version Guard fails with message:
```
❌ FAILED: Version not bumped

Your PR requires: minor bump
But version is:   0.1.0 (unchanged)

To fix:
  cd omega
  ./scripts/version-bump.sh minor
  git add -A
  git commit -m 'chore: bump version'
  git push
```

## Version Bump Rules

Based on conventional commits:

| Commit Type | Version Bump | Example |
|-------------|--------------|---------|
| `fix:` | **PATCH** | 0.1.0 → 0.1.1 |
| `feat:` | **MINOR** | 0.1.0 → 0.2.0 |
| `feat!:` or `BREAKING CHANGE:` | **MAJOR** | 0.1.0 → 1.0.0 |
| `docs:`, `chore:`, `test:` | **PATCH** | 0.1.0 → 0.1.1 |
| `perf:` | **MINOR** | 0.1.0 → 0.2.0 |

### Multiple Commits

If your PR has multiple commits, the **highest required bump** wins:

```bash
git commit -m "docs: update README"       # requires: patch
git commit -m "fix: correct bug"           # requires: patch
git commit -m "feat: add new feature"      # requires: minor

# Version Guard requires: MINOR bump (highest)
```

### Breaking Changes

Breaking changes **always require MAJOR bump**:

```bash
git commit -m "feat!: redesign API

BREAKING CHANGE: Intelligence::new() signature changed"

# Version Guard requires: MAJOR bump
# Even if you also have feat: and fix: commits
```

## Workflow for Contributors

### Standard PR Flow

```bash
# 1. Create feature branch from main
git checkout main
git pull origin main
git checkout -b feat/my-feature

# 2. Make your changes
# ... edit files ...

# 3. Commit with conventional format
git commit -m "feat(core): add new capability"

# 4. Before pushing, bump version
cd omega
./scripts/version-guard.sh main  # Test locally first

# Output shows required bump:
# 🎯 Required bump: minor

# 5. Apply the bump
./scripts/version-bump.sh minor

# 6. Commit version bump
git add -A
git commit -m "chore: bump version to 0.2.0"

# 7. Push and create PR
git push origin feat/my-feature
gh pr create --title "feat: add new capability"

# 8. Version Guard will check and approve ✅
```

### If Version Guard Fails

Don't panic! Just fix the version:

```bash
# 1. Check what bump is required
# Look at GitHub Actions log or run locally:
cd omega
./scripts/version-guard.sh main

# 2. Apply the correct bump
./scripts/version-bump.sh [patch|minor|major]

# 3. Commit and push
git add -A
git commit -m "chore: bump version to match changes"
git push

# 4. Version Guard will re-run and pass ✅
```

## Testing Locally

Before pushing your PR, test the version guard locally:

```bash
cd omega

# Test against main branch
./scripts/version-guard.sh main

# Output:
# ✅ PASSED: Version bump is correct!
#
# Base version:     0.1.0
# PR version:       0.2.0
# Bump type:        minor
# Required minimum: minor
```

## Advanced Scenarios

### Scenario 1: Multiple Features in One PR

```bash
git commit -m "feat(agentdb): add batch insert"
git commit -m "feat(memory): add distributed backend"
git commit -m "docs: update examples"

# Required: MINOR (features present)
./scripts/version-bump.sh minor
```

### Scenario 2: Breaking Change + Features

```bash
git commit -m "feat!: redesign loop API

BREAKING CHANGE: Loop interface changed"
git commit -m "feat(loops): add quantum support"

# Required: MAJOR (breaking change takes priority)
./scripts/version-bump.sh major
```

### Scenario 3: Only Documentation

```bash
git commit -m "docs: add README examples"
git commit -m "docs: fix typos"

# Required: PATCH (minimum bump)
./scripts/version-bump.sh patch
```

### Scenario 4: Bumping Higher Than Required

This is **allowed**:

```bash
git commit -m "fix: correct small bug"
# Required: PATCH

# But you bump MINOR (maybe planning to add features next):
./scripts/version-bump.sh minor

# Version Guard: ✅ PASSED
# Note: You bumped MINOR but only PATCH was required.
#       This is OK - you can bump higher than required.
```

## CI/CD Integration

### Version Guard Workflow

Located at: `.github/workflows/version-guard.yml`

**Triggers**: On pull requests to `main` or `develop`

**Jobs**:
1. **version-check**: Runs `version-guard.sh` script
2. **summary**: Posts result to PR

**Outputs**:
- ✅ PR comment: "Version Guard: PASSED"
- ❌ PR comment: "Version Guard: FAILED" with fix instructions

### Blocking Merge

Version Guard is a **required check** - PRs cannot merge if it fails.

**To configure** (repo admin):
1. Go to: Settings → Branches → main
2. Add branch protection rule
3. Require status checks: "Verify Version Bump"
4. Save

## Differences from Auto-Release

| Feature | Version Guard | Auto-Release |
|---------|---------------|--------------|
| **When** | On PRs | On merge to main |
| **Who bumps version** | Developer (manual) | GitHub Actions (auto) |
| **Validation** | Required before merge | N/A |
| **Commit format** | Enforced | Recommended |
| **Use case** | PR-based workflow | Direct push workflow |

**Version Guard is ACTIVE** (recommended for team projects)
**Auto-Release is DISABLED** (for solo developers)

You can switch by editing workflow files.

## FAQ

### Q: What if I forget to bump the version?

**A**: Version Guard will fail and block the PR. Follow the instructions in the error message to bump and push again.

---

### Q: Can I bump the version higher than required?

**A**: Yes! You can bump MINOR when only PATCH is required, or MAJOR when MINOR is required. This is useful if you're planning future changes.

---

### Q: What if Version Guard suggests wrong bump type?

**A**: This means your commit messages don't follow conventional commits format. Either:
1. Fix your commit messages (use `git rebase -i` to edit)
2. Override with manual bump (Version Guard accepts higher bumps)

---

### Q: Do I need to bump version for every PR?

**A**: Yes, unless your PR only touches non-code files outside `omega/` directory.

---

### Q: What about chore/docs commits?

**A**: These require at minimum a PATCH bump. We recommend bumping version for every PR to keep releases atomic.

---

### Q: Can I disable Version Guard?

**A**: Yes, but not recommended for team projects. To disable:
1. Remove `.github/workflows/version-guard.yml`
2. Enable `auto-release.yml` for fully automated versioning

---

### Q: How do I handle merge conflicts in Cargo.toml?

**A**:
1. Resolve conflicts by taking the **higher version**
2. Re-run version guard locally to verify
3. Push resolved version

Example:
```bash
# Base: 0.1.5
# Your PR: 0.2.0 (minor bump)
# Main updated to: 0.1.6

# Resolution: Take 0.2.0 (higher than 0.1.6)
# Or bump to 0.2.1 if needed
```

---

### Q: What if my PR has no code changes?

**A**: Version Guard skips if no files in `omega/` changed. Otherwise, a PATCH bump is minimum requirement.

---

## Troubleshooting

### Error: "Version not bumped"

```bash
cd omega
./scripts/version-bump.sh patch  # or minor/major
git add -A
git commit -m "chore: bump version"
git push
```

### Error: "Version bump insufficient"

You bumped PATCH but changes require MINOR or MAJOR:

```bash
# Check what's required
./scripts/version-guard.sh main

# Apply correct bump
./scripts/version-bump.sh minor  # or major
git add -A
git commit --amend --no-edit
git push --force-with-lease
```

### Error: "Version downgraded"

This should never happen. If it does:

```bash
# Check main branch version
git fetch origin main
git show origin/main:omega/Cargo.toml | grep version

# Bump to higher than main
./scripts/version-bump.sh patch
git add -A
git commit -m "chore: fix version"
git push
```

## Example PR Workflow (Complete)

```bash
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Starting a new PR
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# 1. Update main
git checkout main
git pull origin main

# Current version: 0.1.5

# 2. Create feature branch
git checkout -b feat/quantum-loops

# 3. Implement feature
vim omega/crates/omega-loops/src/quantum.rs
# ... write code ...

# 4. Write tests
vim omega/crates/omega-loops/tests/quantum_tests.rs
# ... write tests ...

# 5. Commit with conventional format
git add -A
git commit -m "feat(loops): add quantum loop processor

Implements quantum superposition for parallel loop execution.
Includes comprehensive tests and examples."

# 6. Test version guard locally
cd omega
./scripts/version-guard.sh main

# Output:
# 🎯 Required bump: minor
# 📦 Base version: 0.1.5

# 7. Bump version
./scripts/version-bump.sh minor

# Updates to: 0.2.0

# 8. Commit version bump
git add -A
git commit -m "chore: bump version to 0.2.0"

# 9. Push to remote
git push origin feat/quantum-loops

# 10. Create PR
gh pr create \
  --title "feat(loops): add quantum loop processor" \
  --body "Adds quantum superposition capabilities.

## Changes
- New quantum loop processor
- Tests with 95% coverage
- Documentation and examples

## Version
- Bumped to 0.2.0 (minor, new feature)"

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# GitHub Actions runs Version Guard
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# Version Guard: ✅ PASSED
# - Base: 0.1.5
# - PR: 0.2.0
# - Bump: minor
# - Required: minor

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# PR approved and merged
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# Merge triggers publish.yml workflow
# Publishes v0.2.0 to crates.io
```

## Summary

**Version Guard ensures**:
- ✅ Every PR bumps the version
- ✅ Bump type matches conventional commits
- ✅ No version conflicts
- ✅ Clean git history
- ✅ Atomic releases

**Developer workflow**:
1. Make changes
2. Commit with conventional format
3. Bump version locally
4. Push PR
5. Version Guard validates
6. Merge when approved

**Zero manual release work after merge!** 🎉
