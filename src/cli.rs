use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Rust RAG CLI - Retrieval-Augmented Generation over local documents.
#[derive(Parser, Debug)]
#[command(name = "rag", version, about)]
pub struct Cli {
    /// Path to the SQLite database file
    #[arg(long, global = true, default_value = "data/rag.db")]
    pub db: PathBuf,

    /// Use OpenAI embeddings instead of the local hash-based embedder
    #[arg(long, global = true)]
    pub openai_api_key: Option<String>,

    /// OpenAI embedding model name (used only when --openai-api-key is set)
    #[arg(long, global = true, default_value = "text-embedding-3-small")]
    pub openai_model: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Ingest documents into the vector store
    Ingest {
        /// Files or directories to ingest (recursively walks directories)
        #[arg(required = true)]
        paths: Vec<PathBuf>,

        /// Target chunk size in characters
        #[arg(long, default_value_t = 512)]
        chunk_size: usize,

        /// Overlap between consecutive chunks in characters
        #[arg(long, default_value_t = 64)]
        overlap: usize,
    },

    /// Query the corpus
    Query {
        /// Natural-language question
        question: String,

        /// Number of chunks to retrieve
        #[arg(long, default_value_t = 6)]
        top_k: usize,

        /// Only show raw chunks, no synthesized answer
        #[arg(long)]
        raw_only: bool,
    },

    /// Show corpus statistics
    Stats {},
}
