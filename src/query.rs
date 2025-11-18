use anyhow::Result;

use crate::embedder::Embedder;
use crate::models::SearchResult;
use crate::store::Store;

/// Entry point used from CLI.
pub fn run_query(
    store: &Store,
    embedder: &dyn Embedder,
    question: &str,
    top_k: usize,
) -> Result<Vec<SearchResult>> {
    let q_vecs = embedder.embed(&[question.to_string()])?;
    let q_vec = &q_vecs[0];

    let all = store.all_chunks_with_paths()?;
    let mut results = Vec::new();

    for (chunk, doc_path) in all {
        let score = cosine_similarity(q_vec, &chunk.embedding);
        results.push(SearchResult {
            chunk,
            document_path: doc_path,
            score,
        });
    }

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(top_k);
    Ok(results)
}

/// Cosine similarity between two vectors.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len().min(b.len());
    if len == 0 {
        return 0.0;
    }

    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;

    for i in 0..len {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a.sqrt() * norm_b.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_similarity_of_identical_is_one() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let s = cosine_similarity(&a, &b);
        assert!((s - 1.0).abs() < 1e-5);
    }

    #[test]
    fn cosine_similarity_of_orthogonal_is_zero() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let s = cosine_similarity(&a, &b);
        assert!(s.abs() < 1e-5);
    }
}
