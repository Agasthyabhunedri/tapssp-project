use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use chrono::Utc;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::embedder::Embedder;
use crate::models::{Chunk, Document};
use crate::store::Store;

/// Internal struct to hold chunk metadata before we have embeddings.
struct PendingChunk {
    doc_id: Uuid,
    chunk_index: i32,
    start_char: i32,
    end_char: i32,
    text: String,
}

/// Entry point used from CLI.
/// Now batches ALL chunks from ALL files into a single embedder call.
pub fn run_ingest(
    store: &Store,
    embedder: &dyn Embedder,
    paths: &[PathBuf],
    chunk_size: usize,
    overlap: usize,
) -> Result<()> {
    println!(
        "[ingest] Using embedder: {} | chunk_size={} overlap={}",
        embedder.name(),
        chunk_size,
        overlap
    );

    let files = collect_files(paths)?;
    println!("[ingest] Found {} files to ingest", files.len());

    // Collect all chunks from all files before calling embedder.
    let mut pending_chunks: Vec<PendingChunk> = Vec::new();

    for path in files {
        let content = fs::read_to_string(&path)?;
        if content.trim().is_empty() {
            println!("[ingest] Skipping empty file {}", path.display());
            continue;
        }

        let doc = Document {
            id: Uuid::new_v4(),
            path: path.to_string_lossy().to_string(),
            created_at: Utc::now(),
        };
        store.insert_document(&doc)?;

        let chunks_text = chunk_text(&content, chunk_size, overlap);
        let num_chunks = chunks_text.len();

        for (idx, (start, end, text)) in chunks_text.into_iter().enumerate() {
            pending_chunks.push(PendingChunk {
                doc_id: doc.id,
                chunk_index: idx as i32,
                start_char: start as i32,
                end_char: end as i32,
                text,
            });
        }

        println!(
            "[ingest] {} -> {} chunks",
            doc.path,
            num_chunks
        );
    }

    if pending_chunks.is_empty() {
        println!("[ingest] No chunks to embed; nothing to do.");
        return Ok(());
    }

    // Build a single batch of all chunk texts.
    let texts: Vec<String> = pending_chunks.iter().map(|c| c.text.clone()).collect();

    // SINGLE embedding call for ALL chunks across ALL files.
    let embeddings = embedder.embed(&texts)?;
    if embeddings.len() != pending_chunks.len() {
        return Err(anyhow!(
            "embedder returned {} vectors for {} texts",
            embeddings.len(),
            pending_chunks.len()
        ));
    }

    // Insert chunks with embeddings into the store.
    for (pending, emb) in pending_chunks.into_iter().zip(embeddings.into_iter()) {
        let chunk = Chunk {
            id: Uuid::new_v4(),
            doc_id: pending.doc_id,
            chunk_index: pending.chunk_index,
            text: pending.text,
            embedding: emb,
            start_char: pending.start_char,
            end_char: pending.end_char,
        };
        store.insert_chunk(&chunk)?;
    }

    println!("[ingest] Done embedding and storing all chunks.");

    Ok(())
}

/// Recursively collect files from given paths.
fn collect_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for p in paths {
        if p.is_file() {
            files.push(p.clone());
        } else if p.is_dir() {
            for entry in WalkDir::new(p)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                files.push(entry.into_path());
            }
        } else {
            return Err(anyhow!("Path does not exist: {}", p.display()));
        }
    }
    Ok(files)
}

/// Chunk text into overlapping windows (by character).
/// Returns Vec<(start_char, end_char, text)>
fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<(usize, usize, String)> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if len == 0 || chunk_size == 0 {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut start = 0usize;

    while start < len {
        let end = usize::min(start + chunk_size, len);
        let slice: String = chars[start..end].iter().collect();
        chunks.push((start, end, slice));

        if end == len {
            break;
        }

        let next_start = if end > overlap { end - overlap } else { 0 };
        if next_start <= start {
            break;
        }
        start = next_start;
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_text_respects_overlap() {
        let text = "abcdefghijklmnopqrstuvwxyz"; // 26 chars
        let chunks = chunk_text(text, 10, 3);

        assert!(!chunks.is_empty());
        for w in chunks.windows(2) {
            let (_, end_prev, _) = &w[0];
            let (start_next, _, _) = &w[1];
            assert!(start_next < end_prev, "chunks should overlap");
        }
    }
}
