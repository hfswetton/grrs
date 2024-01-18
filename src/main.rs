use grrs::{Cli, find_matches};
use std::io::{self, Write};
use anyhow::{Context, Result, ensure};
use clap::Parser;


fn main() -> Result<()> {
    let args = Cli::parse();
    ensure!(args.pattern() != "", "pattern must not be empty");

    let content = std::fs::read_to_string(args.path())
        .with_context(|| format!("could not read file `{}`", args.path().display()))?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    find_matches(&content, args.pattern(), &mut handle)?;

    handle.flush().with_context(|| String::from("could not flush output buffer"))?;

    Ok(())
}

