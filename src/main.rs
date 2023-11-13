use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
	path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Cli::parse();

	let file_contents = std::fs::read_to_string(&args.path)
		.with_context(|| format!("could not read file `{}`", &args.path.display()))?;

	if !file_contents.ends_with('\n') {
		println!("{}", &args.path.display());
	}

	Ok(())
}
