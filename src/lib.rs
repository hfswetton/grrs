use clap::Parser;
use anyhow::{Context, Result};
use std::path::PathBuf;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

impl Cli {
    pub fn pattern(&self) -> &String {
        &(self.pattern)
    }

    pub fn path(&self) -> &PathBuf {
        &(self.path)
    }
}

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines().filter(|l| l.contains(pattern)) {
        if let Err(error) = writeln!(writer, "{}", line).with_context(|| "could not write to output buffer") {
            return Err(error);
        }
    }
    Ok(())
}

