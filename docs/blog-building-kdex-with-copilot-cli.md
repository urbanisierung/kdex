# Building kdex: How I Built a Complete CLI Tool Using GitHub Copilot CLI

*An experiment in AI-assisted development that exceeded all expectations*

---

## TL;DR

I built [kdex](https://urbanisierung.github.io/kdex/)—a full-featured Rust CLI for indexing code, docs, and notes for AI-powered search—**entirely with GitHub Copilot CLI**. From initial brainstorming to a production-ready tool published on crates.io with CI/CD, landing page, and MCP server integration. Here's how it went.

---

## The Motivation

As developers, we accumulate knowledge everywhere: code across dozens of repositories, notes in Obsidian vaults, documentation scattered in wikis. When working with AI assistants, they have no idea what's in *your* files—you end up copy-pasting context manually.

I wanted to build a tool that solves this. A local-first CLI that indexes everything and lets AI assistants search it via the [Model Context Protocol (MCP)](https://modelcontextprotocol.io/).

But instead of writing it myself, I decided to try something different: **let Copilot CLI build the entire thing**.

---

## The Approach: Brainstorm → Roadmap → Execute

### Phase 1: Requirements Brainstorming

I started by having a conversation with Copilot CLI about what I wanted to build. We discussed:

- Target users (developers, knowledge workers, AI-assisted workflows)
- Core features (local indexing, full-text search, MCP integration)
- Tech stack decisions (Rust, SQLite with FTS5, ratatui for TUI)
- UX considerations (dual-mode CLI: interactive TUI + traditional commands)

The result was a comprehensive `.github/copilot-instructions.md` that established all the ground rules: coding standards, documentation requirements, folder structure, and project conventions.

### Phase 2: Roadmap Generation

Next, I asked Copilot CLI to create a detailed implementation roadmap. It generated a 73KB `doc/roadmap.md` with:

- 14 implementation phases
- Each phase broken into parts with specific action items
- Code examples, SQL schemas, and module structures
- User journeys for different personas
- UX specifications including edge cases

The roadmap became the source of truth. As features were completed, Copilot CLI would check off items and move to the next.

### Phase 3: Implementation

With a clear roadmap, I gave a simple instruction: **"Implement the next phase."**

And it did. Phase after phase.

---

## The Implementation Journey

### Foundation (Day 1)

- Scaffolded Rust project with proper structure
- Implemented configuration system with OS-aware paths
- Created SQLite database layer with FTS5 full-text search
- Built the indexer with binary file detection and .gitignore support
- Developed CLI commands: `index`, `search`, `list`, `update`, `remove`

### TUI & Polish (Day 2)

- Built full-screen interactive terminal UI with ratatui
- Added real-time search, file preview, repository management
- Implemented loading states, confirmation dialogs, welcome screen
- Fixed clippy warnings (the project maintains a **zero-warning policy**)

### MCP Server (Day 3)

- Integrated the [rmcp](https://crates.io/crates/rmcp) crate for MCP protocol
- Implemented 4 tools: `search`, `list_repos`, `get_file`, `get_context`
- Created integration documentation for GitHub Copilot, Claude, and Gemini

### Vector Search (Day 4)

- Added semantic search using fastembed (MiniLM embeddings)
- Implemented hybrid search with Reciprocal Rank Fusion
- Built text chunking for large files

### Remote Repositories (Day 5)

- Added ability to index GitHub repos by URL
- Implemented background sync with auto-pull
- Added SSH and token authentication for private repos
- Built config export/import for machine migration

### CI/CD & Release (Day 6)

- Set up GitHub Actions for testing on Linux/macOS/Windows
- Created release workflow with cross-platform binary builds
- Configured crates.io publishing with Trusted Publishing (OIDC)
- Built install script for curl-based installation

### Marketing & Documentation (Day 7)

- Rewrote README with engaging marketing content
- Created landing page for GitHub Pages
- Documented Ollama integration for local LLMs
- Added `add-mcp` and `self-update` commands

---

## What Got Built

The final result is a production-ready CLI with **85+ features**:

| Category | Highlights |
|----------|------------|
| **Search** | FTS5 lexical, semantic (embeddings), hybrid, fuzzy, regex |
| **Indexing** | Local directories, remote GitHub repos, auto-sync |
| **AI Integration** | MCP server for Copilot/Claude/Gemini, context builder |
| **TUI** | Full-screen interface with preview, search history |
| **Knowledge** | Backlinks, tags, graph visualization, health diagnostics |
| **Distribution** | crates.io, pre-built binaries, install script |

### Example: MCP Integration

With kdex running as an MCP server, AI assistants can search your indexed knowledge:

```bash
# Add to your AI tool's config
kdex add-mcp copilot   # Configures GitHub Copilot CLI
kdex add-mcp gemini    # Configures Gemini CLI
kdex add-mcp claude    # Configures Claude Desktop
```

Now when you ask Copilot "how do I deploy this project?", it can search your actual documentation and code.

### Example: Quick Search

```bash
# Index your knowledge
kdex index ~/code/my-project
kdex index ~/Documents/obsidian-vault
kdex add --remote owner/repo  # Clone and index GitHub repos

# Search (semantic search finds related concepts)
kdex "authentication flow" --hybrid

# Build context for AI prompts
kdex context "error handling" --max-tokens 4000
```

---

## Lessons Learned

### What Worked Exceptionally Well

1. **Structured Planning Pays Off**
   Starting with a detailed roadmap meant Copilot CLI always knew what to build next. No ambiguity, no scope creep.

2. **Consistent Conventions**
   The copilot-instructions.md established clear rules. Every file followed the same patterns. Documentation stayed in sync with code.

3. **Iterative Refinement**
   When something didn't work (CI failures, clippy warnings), I'd report the error and Copilot CLI would fix it—often in seconds.

4. **Full Context Awareness**
   Copilot CLI understood the entire codebase. It could make changes across multiple files while maintaining consistency.

### What Required Guidance

- **Design decisions**: I needed to confirm scope and choose between alternatives
- **External integrations**: Testing MCP with actual AI tools required my verification
- **CI debugging**: Sometimes needed to paste actual error messages from GitHub Actions

---

## The Numbers

| Metric | Value |
|--------|-------|
| Lines of Rust code | ~12,000 |
| Features implemented | 85+ |
| Implementation phases | 14 |
| Total implementation time | ~7 days |
| My code written manually | ~0% |
| Clippy warnings | 0 |
| Test coverage | Unit + integration tests |

---

## Try It Yourself

### Install

```bash
# Quick install (no Rust required)
curl -sSf https://urbanisierung.github.io/kdex/install.sh | sh

# Or via cargo
cargo install kdex
```

### Get Started

```bash
# Index your project
kdex index .

# Search
kdex "your query"

# Configure MCP for your AI tool
kdex add-mcp copilot

# Launch interactive mode
kdex
```

### Links

- **Landing Page**: [urbanisierung.github.io/kdex](https://urbanisierung.github.io/kdex/)
- **GitHub**: [github.com/urbanisierung/kdex](https://github.com/urbanisierung/kdex)
- **crates.io**: [crates.io/crates/kdex](https://crates.io/crates/kdex)
- **Documentation**: [doc/documentation.md](https://github.com/urbanisierung/kdex/blob/main/doc/documentation.md)

---

## Conclusion

Building kdex with Copilot CLI was genuinely impressive. The tool went from a rough idea to a published, production-ready CLI in about a week—with documentation, CI/CD, a landing page, and MCP integration for AI assistants.

The key was **structured collaboration**: I provided the vision and requirements, Copilot CLI handled the implementation. When issues arose, we debugged together.

Is this the future of software development? I don't know. But for this project, it worked remarkably well.

If you're curious about AI-assisted development, give it a try. Start with a clear plan, establish conventions upfront, and let the AI do what it does best: write code.

---

*Built with [GitHub Copilot CLI](https://docs.github.com/en/copilot) • Published February 2026*
