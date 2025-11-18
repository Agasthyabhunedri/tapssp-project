use std::path::Path;

use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde_json;
use uuid::Uuid;

use crate::models::{Chunk, Document};

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS chunks (
                id TEXT PRIMARY KEY,
                doc_id TEXT NOT NULL,
                chunk_index INTEGER NOT NULL,
                text TEXT NOT NULL,
                embedding TEXT NOT NULL,
                start_char INTEGER,
                end_char INTEGER,
                FOREIGN KEY (doc_id) REFERENCES documents(id) ON DELETE CASCADE
            );
        "#,
        )?;
        Ok(())
    }

    pub fn insert_document(&self, doc: &Document) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO documents (id, path, created_at)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(id) DO UPDATE SET path = excluded.path
        "#,
            params![
                doc.id.to_string(),
                doc.path,
                doc.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn insert_chunk(&self, chunk: &Chunk) -> Result<()> {
        let emb_json = serde_json::to_string(&chunk.embedding)?;
        self.conn.execute(
            r#"
            INSERT INTO chunks (id, doc_id, chunk_index, text, embedding, start_char, end_char)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
            params![
                chunk.id.to_string(),
                chunk.doc_id.to_string(),
                chunk.chunk_index,
                chunk.text,
                emb_json,
                chunk.start_char,
                chunk.end_char
            ],
        )?;
        Ok(())
    }

    fn row_to_chunk(&self, row: &Row) -> Result<Chunk> {
        let id_str: String = row.get("id")?;
        let doc_id_str: String = row.get("doc_id")?;
        let emb_json: String = row.get("embedding")?;
        let embedding: Vec<f32> = serde_json::from_str(&emb_json)?;

        Ok(Chunk {
            id: Uuid::parse_str(&id_str)?,
            doc_id: Uuid::parse_str(&doc_id_str)?,
            chunk_index: row.get("chunk_index")?,
            text: row.get("text")?,
            embedding,
            start_char: row.get("start_char")?,
            end_char: row.get("end_char")?,
        })
    }

    pub fn all_chunks_with_paths(&self) -> Result<Vec<(Chunk, String)>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT
                c.id, c.doc_id, c.chunk_index, c.text, c.embedding, c.start_char, c.end_char,
                d.path AS doc_path
            FROM chunks c
            JOIN documents d ON c.doc_id = d.id
        "#,
        )?;

        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let chunk = self.row_to_chunk(&row)?;
            let doc_path: String = row.get("doc_path")?;
            out.push((chunk, doc_path));
        }
        Ok(out)
    }

    pub fn corpus_stats(&self) -> Result<(usize, usize, Option<DateTime<Utc>>)> {
        let doc_count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM documents", [], |r| r.get(0))
            .unwrap_or(0);
        let chunk_count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM chunks", [], |r| r.get(0))
            .unwrap_or(0);

        let latest_created: Option<String> = self.conn.query_row(
            "SELECT created_at FROM documents ORDER BY created_at DESC LIMIT 1",
            [],
            |r| r.get(0),
        ).optional()?;

        let latest_dt = if let Some(s) = latest_created {
            Some(DateTime::parse_from_rfc3339(&s)?.with_timezone(&Utc))
        } else {
            None
        };

        Ok((doc_count as usize, chunk_count as usize, latest_dt))
    }
}

trait OptionalRow<T> {
    fn optional(self) -> Result<Option<T>>;
}

impl<T> OptionalRow<T> for Result<T, rusqlite::Error> {
    fn optional(self) -> Result<Option<T>> {
        match self {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
