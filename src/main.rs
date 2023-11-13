use anyhow::{Context, Result};
use clap::Parser;
use ignore::WalkBuilder;
use std::path::Path;

#[derive(Parser)]
struct Cli {
	path: std::path::PathBuf,
}

fn check_trailing_newline(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
	let file_contents = std::fs::read_to_string(path)
		.with_context(|| format!("could not read file `{}`", path.display()))?;

	if !file_contents.ends_with('\n') {
		println!("{}", path.display());
	}

	Ok(())
}

fn main() {
	let args = Cli::parse();

	for result in WalkBuilder::new("./").hidden(false).build() {
		match result {
			Ok(entry) => check_trailing_newline(entry.path()),
			Err(err) => println!("ERROR: {}", err),
		}
	}
}
