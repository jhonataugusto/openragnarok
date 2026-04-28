# Monorepo Prep Design

## Goal

Prepare this workspace to become a shareable monorepo at `jhonataugusto/openragnarok`, with Korangar, rAthena, Docker infra, scripts, and documentation in one repository.

## Decisions

- Use a direct monorepo layout, not submodules.
- Do not make patches the normal development workflow.
- Keep documentation about upstream origins and local adaptations.
- Keep large/private/local artifacts out of Git, especially GRF files, build outputs, logs, IDE files, and backup/test runtime folders.
- Preserve `PACKETVER=20220406` and the existing local Docker workflow.

## Files To Create Or Update

- `.gitignore`: root ignore rules for Rust, C++, Docker, Windows, IDE, logs, rAthena build outputs, Korangar runtime assets, and local-only folders.
- `README.md`: contributor-friendly setup and development tutorial in pt-BR.
- `docs/UPSTREAMS.md`: notes about source origins, local adaptations, and the no-patches workflow.

## Git Shape

The nested `.git` directories under `korangar/` and `rathena-master/` will be removed after their upstream URLs and current revisions are documented. The source trees then become normal directories owned by the root monorepo.

## Validation

- `git status --short --ignored` should show source files as trackable and large/generated files as ignored.
- `git check-ignore` should confirm GRF files, Rust target output, rAthena binaries, logs, `.vs/`, backup/test runtime folders, and local executables are ignored.
- README instructions should describe first-time setup, server startup, client build/run, development workflow, and what not to commit.
