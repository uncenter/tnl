use clap::Parser;
use ignore::WalkBuilder;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser)]
#[clap(name = "tnl")]
struct Cli {
	path: std::path::PathBuf,
	#[clap(long, short)]
	fix: bool,
}

fn append_newline(path: &std::path::Path) -> std::io::Result<()> {
	let mut file = OpenOptions::new().append(true).open(path)?;
	writeln!(file)?;
	Ok(())
}

fn main() {
	let args = Cli::parse();

	for result in WalkBuilder::new(args.path.clone()).hidden(false).build() {
		match result {
			Ok(entry) => match std::fs::metadata(entry.path()) {
				Ok(metadata) => {
					if !metadata.is_dir() {
						if let Ok(contents) = std::fs::read_to_string(entry.path()) {
							if !contents.ends_with('\n') {
								println!("{}", entry.path().display());
								if args.fix {
									if let Err(_err) = append_newline(entry.path()) {
										eprintln!(
											"Something went wrong while fixing file '{}'.",
											entry.path().display()
										);
									}
								}
							}
						}
					}
				}
				Err(_err) => {
					eprintln!(
						"Something went wrong while reading file '{}'.",
						entry.path().display()
					);
				}
			},
			Err(_err) => {
				eprintln!(
					"Could not read file or directory '{}'.",
					args.path.to_string_lossy()
				);
				std::process::exit(1)
			}
		}
	}
}
