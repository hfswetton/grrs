use clap::Parser;
use anyhow::{Context, Result};
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for line in content.lines().filter(|l| l.contains(&args.pattern)) {
        writeln!(handle, "{}", line)
            .with_context(|| String::from("could not write to output buffer"))?;
    }
    
    handle.flush().with_context(|| String::from("could not flush output buffer"))?;

    Ok(())
}