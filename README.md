
# 🦀 Rust RAG CLI — Retrieval-Augmented Generation in Rust  
**Final Project — CSC 595: Systems Programming in Rust**  
**Author:** *Agasthya Bhunedri · DePaul University*  
**Repository:** `tapssp-project`  
**Instructor:** *Corin Pitcher*  

---

# 🎥 Project Video Demo  
📌 **YouTube Link:** *ADD YOUR VIDEO LINK HERE*  

---

# 🏷️ Badges

![Rust](https://img.shields.io/badge/Rust-Systems%20Programming-orange)
![SQLite](https://img.shields.io/badge/DB-SQLite-blue)
![RAG](https://img.shields.io/badge/RAG-Retrieval%20Augmented%20Generation-green)
![CLI](https://img.shields.io/badge/Interface-CLI-lightgrey)
![Status](https://img.shields.io/badge/Project-Final%20Submission-brightgreen)

---

# 📘 1. Project Overview

The **Rust RAG CLI** is a fully local Retrieval-Augmented Generation (RAG) system built for CSC 595 — Systems Programming in Rust.

The tool:

- Ingests local `.md`, `.txt`, `.rs` files  
- Splits them into overlapping chunks  
- Embeds each chunk  
- Stores them in a SQLite database  
- Retrieves the most relevant chunks for a query using cosine similarity  

Demonstrates systems programming concepts:

- File I/O  
- Directory walking  
- Chunking  
- Traits  
- SQLite integration  
- CLI design  
- Embedding & vector math  

---

# 📐 2. Architecture Diagram

```
                 ┌────────────────────────────────────────┐
                 │              CLI Layer                  │
                 │    rag ingest | query | stats           │
                 └───────────────┬────────────────────────┘
                                 │
                                 ▼
      ┌────────────────────────────────────────────────────────────┐
      │                        Ingestion Pipeline                  │
      │------------------------------------------------------------│
      │ • Walk directories (docs/, src/)                           │
      │ • Load .md / .txt / .rs files                              │
      │ • Chunk text with sliding window                           │
      │ • Embed chunks (LocalHashEmbedder/OpenAI)                  │
      │ • Insert into SQLite                                       │
      └───────────────┬────────────────────────────────────────────┘
                      │
                      ▼
      ┌────────────────────────────────────────────────────────────┐
      │                         Storage Layer                       │
      │-------------------------------------------------------------│
      │ SQLite (data/rag.db) stores:                                │
      │   - documents                                               │
      │   - chunks + embeddings                                     │
      └───────────────┬────────────────────────────────────────────┘
                      │
                      ▼
      ┌────────────────────────────────────────────────────────────┐
      │                      Retrieval Engine                       │
      │-------------------------------------------------------------│
      │ • Embed query                                               │
      │ • Cosine similarity search                                  │
      │ • Rank top-k chunks                                         │
      │ • Optional synthesis                                        │
      └───────────────┬────────────────────────────────────────────┘
                      │
                      ▼
      ┌────────────────────────────────────────────────────────────┐
      │                        Output Layer                         │
      │-------------------------------------------------------------│
      │  - Ranked chunks                                            │
      │  - File paths + spans                                       │
      │  - Latency info                                             │
      └────────────────────────────────────────────────────────────┘
```

---

# 📂 3. Documents (`docs/` Folder)

Place files you want to ingest here:

```
docs/
   rust_intro.md
   systems_programming.md
   logging_example.rs
   config_guide.txt
```

Works with any UTF-8 text.

---

# 🔧 4. Build & Run Instructions

### Build
```
cargo build
```

### Test
```
cargo test
```

### Ingest documents
```
cargo run -- ingest ./docs --chunk-size 512 --overlap 64
```

### Query
```
cargo run -- query "What is Rust?" --top-k 5
```

### Show Stats
```
cargo run -- stats
```

---

# 🧠 5. Design Summary

### SQLite  
- Simple, fast, embedded  
- No external server  
- Good for systems projects  

### Embeddings  
- **LocalHashEmbedder** (offline)  
- **OpenAIEmbedder** (optional, real embeddings)  

### Chunking  
- Character-based  
- Overlapping sliding window  

### Retrieval  
- Cosine similarity  
- Sort by relevance  

---

# 🔍 6. Example Output

### Query Example
```
Top-K Matching Chunks:
1. docs/rust_intro.md ...
2. docs/systems_programming.md ...
```

### Stats Example
```
Documents: 4
Chunks: 13
Last Ingest: <timestamp>
```

---

# 🚫 7. Limitations

- Linear scan retrieval  
- Hash embeddings not semantic  
- No BM25 / ANN  
- Simple chunking  

---

# 🚀 8. Future Enhancements

- HNSW vector index  
- BM25 (Tantivy)  
- Multi-threaded ingestion  
- Semantic chunk splitting  
- TUI interface  

---

# 📦 9. Repository Structure

```
tapssp-project/
├── Cargo.toml
├── README.md
├── docs/
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── models.rs
│   ├── embedder.rs
│   ├── store.rs
│   ├── ingest.rs
│   ├── query.rs
│   └── stats.rs
└── tests/
    └── basic_flow.rs
```

---

# 🎓 Final Notes

This project satisfies all CSC 595 final project requirements and demonstrates real systems programming in Rust.

Add your **YouTube demo link** above once recorded!
