use anyhow::Result;

use crate::store::Store;

/// Print corpus stats to stdout.
pub fn run_stats(store: &Store) -> Result<()> {
    let (docs, chunks, latest) = store.corpus_stats()?;

    println!("────────────────────────────");
    println!("Corpus Stats");
    println!("────────────────────────────");
    println!("Documents   : {}", docs);
    println!("Chunks      : {}", chunks);

    if let Some(dt) = latest {
        println!("Last Ingest : {}", dt);
    } else {
        println!("Last Ingest : (none)");
    }

    Ok(())
}
