# Monorepo Prep Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prepare the Ragnarok workspace as a shareable direct monorepo.

**Architecture:** Keep the root repository as the single source of truth. Convert nested upstream checkouts into normal directories, keep local/generated/private files ignored, and document how collaborators set up and develop the project.

**Tech Stack:** Git, GitHub, Docker Compose, Rust/Cargo, rAthena C++ build tooling, Windows PowerShell/batch.

---

### Task 1: Add Repository Hygiene Files

**Files:**
- Create: `.gitignore`
- Create: `docs/UPSTREAMS.md`

- [ ] **Step 1: Add `.gitignore`**

Create root ignore rules covering GRF assets, Rust/Cargo build output, rAthena binaries/build output, logs, IDE files, local backup/runtime folders, and generated executables.

- [ ] **Step 2: Add upstream documentation**

Document that `korangar/` comes from `https://github.com/vE5li/korangar`, `rathena-master/` comes from `https://github.com/rathena/rathena`, and this monorepo keeps direct source instead of patches/submodules.

- [ ] **Step 3: Validate ignores**

Run `git check-ignore -v` for representative generated/private files and confirm they are ignored.

### Task 2: Rewrite Contributor README

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Replace README with pt-BR setup tutorial**

Include project overview, repository layout, prerequisites, Docker server startup, Korangar build/run, asset requirements, useful commands, development workflow, and contribution notes.

- [ ] **Step 2: Validate the README against current files**

Check that referenced paths exist and commands match the current workspace.

### Task 3: Convert Nested Repos Into Monorepo Directories

**Files:**
- Remove local metadata only: `korangar/.git/`
- Remove local metadata only: `rathena-master/.git/`

- [ ] **Step 1: Record nested repo state**

Capture branch/commit/remote information in `docs/UPSTREAMS.md`.

- [ ] **Step 2: Remove nested `.git` directories**

After path verification, remove only `D:\ragnarok\korangar\.git` and `D:\ragnarok\rathena-master\.git`.

- [ ] **Step 3: Validate root Git sees normal files**

Run `git status --short` from the root and confirm `korangar/` and `rathena-master/` are normal untracked trees, with generated/private artifacts ignored.

### Task 4: Configure GitHub Remote

**Files:**
- Git config only

- [ ] **Step 1: Add remote**

Set `origin` to `https://github.com/jhonataugusto/openragnarok.git`.

- [ ] **Step 2: Create remote repository if possible**

Use `gh repo create jhonataugusto/openragnarok` if GitHub CLI is installed and authenticated. If not available, leave the remote configured and document the exact command for manual creation.

- [ ] **Step 3: Final validation**

Run `git status --short --ignored`, `git remote -v`, and ignore checks before summarizing.
