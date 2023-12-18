use clap::Parser;
use color_eyre::eyre::{Context, Result};
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::fs;
use std::io::Write;

#[derive(Parser)]
#[clap(name = "tnl")]
struct Cli {
	path: std::path::PathBuf,
	#[clap(long, short)]
	fix: bool,
	#[clap(long, short)]
	quiet: bool,
}

fn append_newline(path: &std::path::Path) -> Result<()> {
	let mut file = fs::OpenOptions::new()
		.append(true)
		.open(path)
		.with_context(|| format!("failed to open file '{}'", path.display()))?;
	writeln!(file)
		.with_context(|| format!("failed to write newline to file '{}'", path.display()))?;
	Ok(())
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Cli::parse();

	let mut overrides = OverrideBuilder::new(args.path.clone());
	overrides.add("!.git/").context("Invalid exclude pattern")?;

	let mut builder = WalkBuilder::new(args.path.clone());
	builder.hidden(false).overrides(
		overrides
			.build()
			.context("WalkBuilder construction error")?,
	);

	let mut files_checked: i32 = 0;
	let mut files_fixed: i32 = 0;

	for result in builder.build() {
		let entry = result.with_context(|| {
			format!("could not read file or directory '{}'", args.path.display())
		})?;
		let metadata = fs::metadata(entry.path()).with_context(|| {
			format!(
				"could not read file or directory '{}'",
				entry.path().display()
			)
		})?;
		if !metadata.is_dir() {
			if let Ok(contents) = fs::read_to_string(entry.path()) {
				files_checked += 1;
				if !contents.ends_with('\n') {
					println!("{}", entry.path().display());
					if args.fix {
						append_newline(entry.path()).with_context(|| {
							format!("could not fix file '{}'", entry.path().display())
						})?;
						files_fixed += 1;
					}
				}
			}
		};
	}

	if !args.quiet {
		println!(
			"Checked {} file{}{}.",
			files_checked,
			if files_checked == 1 { "" } else { "s" },
			if args.fix { format!(", fixed {}", files_fixed) } else { "".to_string() },
		);

	};

	Ok(())
}
