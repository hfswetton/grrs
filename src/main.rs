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

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines().filter(|l| l.contains(pattern)) {
        if let Err(error) = writeln!(writer, "{}", line).with_context(|| "could not write to output buffer") {
            return Err(error);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    find_matches(&content, &args.pattern, &mut handle)?;

    handle.flush().with_context(|| String::from("could not flush output buffer"))?;

    Ok(())
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).expect("error finding matches");
    assert_eq!(result, b"lorem ipsum\n");
}