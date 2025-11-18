use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A single ingested document (file).
#[derive(Debug, Clone)]
pub struct Document {
    pub id: Uuid,
    pub path: String,
    pub created_at: DateTime<Utc>,
}

/// A chunk of text derived from a document, with its embedding.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: Uuid,
    pub doc_id: Uuid,
    pub chunk_index: i32,
    pub text: String,
    pub embedding: Vec<f32>,
    pub start_char: i32,
    pub end_char: i32,
}

/// Result of a similarity search.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub chunk: Chunk,
    pub document_path: String,
    pub score: f32,
}
