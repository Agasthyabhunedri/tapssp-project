use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;

use tapssp_project::cli::{Cli, Commands};
use tapssp_project::embedder::{Embedder, LocalEmbedder, OpenAIEmbedder};
use tapssp_project::ingest::run_ingest;
use tapssp_project::query::run_query;
use tapssp_project::stats::run_stats;
use tapssp_project::store::Store;

fn main() -> Result<()> {
    dotenv().ok(); // allow loading OPENAI_API_KEY from .env
    let cli = Cli::parse();

    let store = Store::new(&cli.db)?;

    let embedder = build_embedder(&cli);

    match &cli.command {
        Commands::Ingest {
            paths,
            chunk_size,
            overlap,
        } => {
            run_ingest(&store, embedder.as_ref(), paths, *chunk_size, *overlap)?;
        }
        Commands::Query {
            question,
            top_k,
            raw_only,
        } => {
            let results = run_query(&store, embedder.as_ref(), question, *top_k)?;
            if *raw_only {
                print_raw_results(&results);
            } else {
                print_synthesized_answer(question, &results);
            }
        }
        Commands::Stats {} => {
            run_stats(&store)?;
        }
    }

    Ok(())
}

fn build_embedder(cli: &Cli) -> Box<dyn Embedder> {
    if let Some(key) = cli.openai_api_key.clone().or_else(|| std::env::var("OPENAI_API_KEY").ok())
    {
        Box::new(OpenAIEmbedder::new(key, cli.openai_model.clone()))
    } else {
        Box::new(LocalEmbedder::new(256))
    }
}

fn print_raw_results(results: &[tapssp_project::models::SearchResult]) {
    println!("─────────────────────────────────────────────");
    for (i, r) in results.iter().enumerate() {
        println!("#{} | score = {:.4}", i + 1, r.score);
        println!("File : {}", r.document_path);
        println!("Span : {}..{}", r.chunk.start_char, r.chunk.end_char);
        println!("Text :\n{}\n", r.chunk.text.trim());
        println!("─────────────────────────────────────────────");
    }
}

/// VERY SIMPLE synthesizer: just concatenates top chunks with a header.
/// (For the project, you can extend this to an actual LLM call if desired.)
fn print_synthesized_answer(
    question: &str,
    results: &[tapssp_project::models::SearchResult],
) {
    println!("Question: {}", question);
    println!("─────────────────────────────────────────────");
    println!("(Prototype synthesized answer)\n");

    for (i, r) in results.iter().enumerate() {
        println!("[{}] From {}", i + 1, r.document_path);
        println!("Score: {:.4}", r.score);
        println!("Snippet:\n{}\n", r.chunk.text.trim());
    }

    println!("─────────────────────────────────────────────");
    println!("(Above snippets are the most relevant context chunks.)");
}
