use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin("tnl")?;

	cmd.arg("file/does/not/exist");
	cmd.assert()
		.failure()
		.stderr(predicate::str::contains("could not read file or directory"));

	Ok(())
}

#[test]
fn file_not_ends_with_newline() -> Result<(), Box<dyn std::error::Error>> {
	let file = assert_fs::NamedTempFile::new("sample.txt")?;
	file.write_str("Lorem ipsum dolor amet.")?;

	let mut cmd = Command::cargo_bin("tnl")?;
	cmd.arg(file.path());
	cmd.assert()
		.success()
		.stdout(predicate::str::contains(file.to_string_lossy()));

	Ok(())
}

#[test]
fn file_ends_with_newline() -> Result<(), Box<dyn std::error::Error>> {
	let file = assert_fs::NamedTempFile::new("sample.txt")?;
	file.write_str("Lorem ipsum dolor amet.\n")?;

	let mut cmd = Command::cargo_bin("tnl")?;
	cmd.arg(file.path());
	cmd.assert().success().stdout(predicate::str::is_empty());

	Ok(())
}
