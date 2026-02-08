mod embedder;
mod indexer;
mod markdown;
mod searcher;
mod watcher;

#[allow(unused_imports)]
pub use embedder::{ChunkEmbedding, Embedder, TextChunk};
pub use indexer::Indexer;
pub use markdown::parse_markdown;
#[allow(unused_imports)]
pub use markdown::{Heading, MarkdownMeta};
pub use searcher::{SearchMode, Searcher};
#[allow(unused_imports)]
pub use watcher::{ChangeType, IndexWatcher, PendingChange, RepoBatch};
