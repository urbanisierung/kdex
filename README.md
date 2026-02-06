# knowledge-index

A fast, local-first CLI for indexing code repositories and knowledge bases, enabling AI-powered search across all your projects.

## Motivation

Modern developers and knowledge workers maintain dozens of repositories, documentation sites, and note collections (like Obsidian vaults). When working with AI assistants, finding the right context across these scattered sources is challenging.

**knowledge-index** solves this by:
- **Indexing everything locally** — Code, markdown, configs across all your projects
- **Enabling instant full-text search** — SQLite FTS5 provides sub-millisecond queries
- **Providing AI-ready output** — JSON mode and MCP server integration (coming soon)
- **Working offline** — No cloud dependencies, your data stays local

## Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))

## Quickstart

```bash
# Clone and build
git clone https://github.com/urbanisierung/knowledge-index.git
cd knowledge-index
cargo build --release

# Index a project
./target/release/knowledge-index index /path/to/your/project

# Search across all indexed content
./target/release/knowledge-index search "async function"

# Launch interactive TUI
./target/release/knowledge-index

# Or install globally
cargo install --path .
```

## Usage

The CLI supports two modes:

### App Mode (TUI)

```bash
knowledge-index
```

Launches a full-screen interactive interface for searching and managing indexed repositories.

### CLI Mode

```bash
# Index current directory
knowledge-index index .

# Search for code
knowledge-index search "database connection"

# List all indexed repos
knowledge-index list

# Get JSON output for scripting
knowledge-index search "TODO" --json
```

Run `knowledge-index --help` for all available commands.

## Documentation

- [Features](doc/features.md) — Feature overview
- [Documentation](doc/documentation.md) — Detailed usage guide
- [Roadmap](doc/roadmap.md) — Implementation roadmap
- [Progress](doc/progress.md) — Changelog

## License

MIT
