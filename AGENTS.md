# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` wires CLI parsing to library entry points.
- `src/lib.rs` contains command dispatch and shared logic.
- `src/config.rs` defines CLI arguments and subcommands (via `clap`).
- `.github/workflows/release.yml` builds release artifacts on tag pushes.
- `target/` is build output (generated; do not edit or commit).

## Build, Test, and Development Commands
- `cargo build` builds the debug binary into `target/debug`.
- `cargo build --release --locked` builds a locked release binary (used in CI).
- `cargo run -- tag --version 1.2.3` runs the CLI locally with the `tag` subcommand.
- `cargo fmt` formats Rust source if `rustfmt` is installed.

## Coding Style & Naming Conventions
- Rust 2024 edition with standard `rustfmt` defaults (4-space indentation).
- Module/file naming follows Rust conventions: `snake_case.rs` and `CamelCase` types.
- Keep CLI help text concise and user-facing; prefer English for new strings unless matching existing Chinese messages.

## Testing Guidelines
- No automated tests are present yet. If adding tests, place unit tests in the module file and integration tests under `tests/`.
- Use descriptive test names like `test_tag_requires_version` and document how to run them with `cargo test`.

## Commit & Pull Request Guidelines
- Recent commits are short, lowercase summaries (e.g., `config`, `structure optim`).
- Follow that style: 1 line, imperative/summary-focused, no trailing period.
- PRs should include a brief description, the motivation for the change, and any CLI output/screenshots if behavior changes.

## Release & Distribution Notes
- Releases are triggered by tags matching `v*` and package `fekit` binaries for macOS, Linux, and Windows.
- If you update CLI flags, ensure help text and release workflow remain accurate.
