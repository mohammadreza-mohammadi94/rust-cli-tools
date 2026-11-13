use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod cli;
mod search;
mod output;
mod walker;

use cli::Args;
use search::SearchEngine;

fn main() -> Result<()> {
    let args = Args::parse();
    
    let mut engine = SearchEngine::new(
        &args.pattern,
        args.ignore_case,
        args.word_regexp,
        args.fixed_strings,
    )?;

    engine.set_context(args.before_context, args.after_context);
    engine.set_max_count(args.max_count);
    
    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };

    engine.search_paths(&paths, &args)?;
    
    Ok(())
}
