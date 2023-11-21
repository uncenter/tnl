use clap::Parser;
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
}

fn append_newline(path: &std::path::Path) -> std::io::Result<()> {
	let mut file = fs::OpenOptions::new().append(true).open(path)?;
	writeln!(file)?;
	Ok(())
}

fn main() {
	let args = Cli::parse();

	let mut overrides = OverrideBuilder::new(args.path.clone());
	overrides.add("!.git/").expect("Invalid exclude pattern");

	let mut builder = WalkBuilder::new(args.path.clone());
	builder
		.hidden(false)
		.overrides(overrides.build().expect("WalkBuilder construction error"));

	for result in builder.build() {
		let entry = result
			.unwrap_or_else(|_| panic!("read file or directory '{}'", args.path.to_string_lossy()));
		let metadata = fs::metadata(entry.path())
			.unwrap_or_else(|_| panic!("read file or directory '{}'", entry.path().display()));
		if !metadata.is_dir() {
			if let Ok(contents) = fs::read_to_string(entry.path()) {
				if !contents.ends_with('\n') {
					println!("{}", entry.path().display());
					if args.fix {
						append_newline(entry.path())
							.unwrap_or_else(|_| panic!("fix file '{}'", entry.path().display()));
					}
				}
			}
		};
	}
}
