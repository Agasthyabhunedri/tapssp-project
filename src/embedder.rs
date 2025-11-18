use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

/// Generic embedding interface.
pub trait Embedder {
    fn name(&self) -> &'static str;
    fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}

/// Simple local hash-based embedder (bag-of-words → fixed-size vector).
pub struct LocalEmbedder {
    dim: usize,
}

impl LocalEmbedder {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }

    fn hash_token(&self, token: &str) -> usize {
        let mut h: u64 = 0;
        for b in token.bytes() {
            h = h.wrapping_mul(31).wrapping_add(b as u64);
        }
        (h as usize) % self.dim
    }
}

impl Embedder for LocalEmbedder {
    fn name(&self) -> &'static str {
        "local-hash-embedding-256"
    }

    fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut all = Vec::with_capacity(texts.len());
        for t in texts {
            let mut v = vec![0f32; self.dim];
            for token in t.split_whitespace() {
                let idx = self.hash_token(token);
                v[idx] += 1.0;
            }
            all.push(v);
        }
        Ok(all)
    }
}

/// OpenAI embeddings backend.
/// Only used when an API key is provided.
pub struct OpenAIEmbedder {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct OpenAIEmbeddingRequest<'a> {
    model: &'a str,
    input: &'a [String],
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingData {
    embedding: Vec<f32>,
}

impl OpenAIEmbedder {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }
}

impl Embedder for OpenAIEmbedder {
    fn name(&self) -> &'static str {
        "openai-embeddings"
    }

    fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(vec![]);
        }

        let url = "https://api.openai.com/v1/embeddings";
        let body = OpenAIEmbeddingRequest {
            model: &self.model,
            input: texts,
        };

        let resp = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()?
            .error_for_status()?;

        let parsed: OpenAIEmbeddingResponse = resp.json()?;
        if parsed.data.len() != texts.len() {
            return Err(anyhow!(
                "Expected {} embeddings, got {}",
                texts.len(),
                parsed.data.len()
            ));
        }

        Ok(parsed.data.into_iter().map(|d| d.embedding).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_embedder_produces_fixed_dim_vectors() {
        let emb = LocalEmbedder::new(32);
        let out = emb
            .embed(&[String::from("hello world"), String::from("hello rust")])
            .unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].len(), 32);
        assert_eq!(out[1].len(), 32);
    }
}
