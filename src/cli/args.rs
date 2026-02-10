use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "kdex",
    about = "Index and search code repositories and knowledge bases for AI-powered workflows",
    version,
    author
)]
#[command(after_help = "Examples:
  kdex                     Launch interactive TUI
  kdex index .             Index current directory
  kdex index ~/notes       Index Obsidian vault
  kdex search \"async fn\"   Search for async functions
  kdex search \"TODO\" --type markdown
  kdex list                List all indexed repositories

Shell Aliases (add to ~/.bashrc or ~/.zshrc):
  alias ki='kdex'
  alias kis='kdex search'
  alias kii='kdex index .'
")]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Suppress non-error output
    #[arg(long, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Enable verbose output
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Enable debug output with backtraces
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Index a directory (code repository or knowledge base)
    #[command(after_help = "Examples:
  kdex index                    Index current directory
  kdex index ~/projects/myapp   Index specific project
  kdex index ~/Documents/notes  Index Obsidian vault
")]
    Index {
        /// Directory to index (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Custom name for the repository
        #[arg(long)]
        name: Option<String>,
    },

    /// Search indexed content
    #[command(after_help = "Examples:
  kdex search \"database connection\"
  kdex search \"async fn\" --repo api-service
  kdex search \"TODO\" --type markdown
  kdex search \"error handling\" --semantic
  kdex search \"authentication\" --hybrid
")]
    Search {
        /// Search query (supports phrases and wildcards)
        query: String,

        /// Filter by repository name
        #[arg(long)]
        repo: Option<String>,

        /// Filter by file type (code, markdown, config)
        #[arg(long, name = "type")]
        file_type: Option<String>,

        /// Maximum number of results
        #[arg(long, default_value = "20")]
        limit: usize,

        /// Group results by repository
        #[arg(long)]
        group_by_repo: bool,

        /// Use semantic (vector) search
        #[arg(long, conflicts_with_all = ["hybrid", "lexical"])]
        semantic: bool,

        /// Use hybrid search (combines lexical + semantic)
        #[arg(long, conflicts_with_all = ["semantic", "lexical"])]
        hybrid: bool,

        /// Use lexical (full-text) search (default)
        #[arg(long, conflicts_with_all = ["semantic", "hybrid"])]
        lexical: bool,
    },

    /// Update an existing index
    #[command(after_help = "Examples:
  kdex update .            Update current directory
  kdex update --all        Update all repositories
")]
    Update {
        /// Repository path to update
        path: Option<PathBuf>,

        /// Update all indexed repositories
        #[arg(long)]
        all: bool,
    },

    /// List all indexed repositories
    List {},

    /// Remove a repository from the index
    #[command(after_help = "Examples:
  kdex remove ~/projects/old-project
  kdex remove . --force    Skip confirmation
")]
    Remove {
        /// Repository path to remove
        path: PathBuf,

        /// Skip confirmation prompt
        #[arg(long, short)]
        force: bool,
    },

    /// Show or edit configuration
    Config {
        /// Configuration key to show/set
        key: Option<String>,

        /// Value to set
        value: Option<String>,

        /// Reset to defaults
        #[arg(long)]
        reset: bool,
    },

    /// Start MCP server for AI tool integration
    Mcp {},

    /// Watch for file changes and re-index automatically
    Watch {
        /// Watch all indexed repositories
        #[arg(long)]
        all: bool,

        /// Specific repository path to watch
        path: Option<PathBuf>,
    },

    /// Rebuild embeddings for semantic search
    #[command(after_help = "Examples:
  kdex rebuild-embeddings         Rebuild all embeddings
  kdex rebuild-embeddings --repo myproject
")]
    RebuildEmbeddings {
        /// Filter by repository name
        #[arg(long)]
        repo: Option<String>,
    },
}
