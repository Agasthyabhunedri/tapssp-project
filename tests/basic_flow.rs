use std::fs;

use anyhow::Result;

use tapssp_project::embedder::LocalEmbedder;
use tapssp_project::ingest::run_ingest;
use tapssp_project::query::run_query;
use tapssp_project::store::Store;

#[test]
fn basic_end_to_end_flow() -> Result<()> {
    let tmp_dir = std::env::temp_dir();
    let db_path = tmp_dir.join("rag_test.db");
    let corpus_path = tmp_dir.join("rag_test_doc.txt");

    fs::write(
        &corpus_path,
        "Rust is a systems programming language focused on safety and performance.",
    )?;

    let store = Store::new(&db_path)?;
    let embedder = LocalEmbedder::new(64);

    run_ingest(&store, &embedder, &[corpus_path.clone()], 64, 16)?;

    let results = run_query(&store, &embedder, "What is Rust?", 3)?;
    assert!(!results.is_empty());

    // clean up
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(corpus_path);
    Ok(())
}
