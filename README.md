# 🦀 Rust RAG CLI  
**Retrieval-Augmented Generation System in Rust**  
_Designed by **Agasthya Bhunedri** — DePaul University (CSC 595 · Systems Programming in Rust)_

---

## 🚀 Overview

**Goal**  
Build a **trustworthy, low-latency Q&A system** over your own documents and code using **Rust**.  
The tool **retrieves** relevant text chunks from local files and **grounds** answers produced by an **LLM**, ensuring transparency, speed, and reproducibility with clear source citations.

Instead of retraining or fine-tuning models, this project uses **Retrieval-Augmented Generation (RAG)** to inject private, up-to-date context directly into prompts — powered by Rust’s concurrency, safety, and performance guarantees.

---

## 🧠 Problem

Large Language Models (LLMs) often **hallucinate** or miss context because they don’t “know” your internal documentation, logs, or code.  
Teams need a **local**, **verifiable**, and **fast** way to query their own files.

This system provides:

- ✅ Local, source-grounded answers  
- ⚡ Low-latency text retrieval and ranking  
- 🧩 Modular API traits (Embedder, Retriever, LLM)  
- 🔒 Secure and offline-friendly operation

 ---

## ⚙️ Architecture

```text
rag ingest / query / stats
          │
          ▼
 ┌────────────────────────────┐
 │ Ingestion Pipeline         │
 │  → Parse .md / .txt / .rs │
 │  → Chunk & store in DB    │
 └──────────┬────────────────┘
            ▼
 ┌────────────────────────────┐
 │ Index Layer                │
 │  → Vector (HNSW) Index     │
 │  → Lexical (BM25) Index    │
 └──────────┬────────────────┘
            ▼
 ┌────────────────────────────┐
 │ Hybrid Retriever + Rerank  │
 └──────────┬────────────────┘
            ▼
 ┌────────────────────────────┐
 │ LLM Backend (OpenAI)       │
 │  → Context + Citations     │
 └────────────────────────────┘

```
----

## 🧩 Example Workflow

### 1️⃣ Ingest your documents
```text
rag ingest ./docs ./src --chunk-size 512 --overlap 64
```
**Output**
```text
[✔] Loaded 28 files (MD, RS, TXT)
[✔] Created 410 chunks (avg 508 chars)
[✔] Embedded via text-embedding-3-small (dim 1536)
[✔] Indexed to vector and BM25 stores
Workspace : ./data (HNSW 3.1 MB · Tantivy 6.7 MB)
```
2️⃣ Query the system
```text
rag query "How does log rotation work?" --top-k 6 --cite

```

Output
```text
─────────────────────────────────────────────
Answer:
The log rotation service spawns a background
thread that checks file size and modification
time. Files larger than 10 MB or older than 7 days
are renamed with a timestamp suffix and recreated.
Configuration is in config/log.toml. [1][2]

─────────────────────────────────────────────
Citations:
[1] src/log/rotate.rs (lines 42–68)
[2] config/log.toml (lines 1–12)
─────────────────────────────────────────────
Latency : 1.84 s  Retrieval : 0.72 s  LLM : 1.12 s
```
3️⃣ View corpus stats
```text
rag stats
```

Output
```text
────────────────────────────
Corpus Stats
────────────────────────────
Documents   : 28
Chunks      : 410
Embedding Dim : 1536
Vector Index  : 3.1 MB
Lexical Index : 6.7 MB
p50 Retrieval Latency : 0.72 s
