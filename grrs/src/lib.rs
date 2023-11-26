use anyhow::{Context, Result};
use std::io::prelude::*;

pub fn find_matches<R: BufRead, W: Write>(reader: R, pattern: &str, mut writer: W) -> Result<W> {
    if pattern.is_empty() {
        return Ok(writer);
    }

    for line in reader.lines() {
        let line = line.with_context(|| "Error reading line")?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| "Error writing to output")?;
        }
    }

    Ok(writer)
}
