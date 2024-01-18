use grrs::{Cli, find_matches};
use std::io::{self, Write};
use anyhow::{Context, Result};
use clap::Parser;


fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(args.path())
        .with_context(|| format!("could not read file `{}`", args.path().display()))?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    find_matches(&content, args.pattern(), &mut handle)?;

    handle.flush().with_context(|| String::from("could not flush output buffer"))?;

    Ok(())
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).expect("error finding matches");
    assert_eq!(result, b"lorem ipsum\n");
}