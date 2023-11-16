use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = match line {
            Ok(ln) => ln,
            Err(e) => return Err(e),
        };

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
