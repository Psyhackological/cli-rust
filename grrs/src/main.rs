use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let reader = BufReader::new(f);
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    find_matches(reader, &args.pattern, &mut handle)?;

    handle.flush().with_context(|| "Error flushing output")?;

    Ok(())
}

#[test]
fn find_a_match() {
    let file_content = "lorem ipsum\ndolor sit amet".as_bytes();
    let reader = BufReader::new(file_content);
    let mut result = Vec::new();
    let match_result = find_matches(reader, "lorem", &mut result);
    assert!(match_result.is_ok());
    assert_eq!(result, b"lorem ipsum\n");
}
