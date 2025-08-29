use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn grep_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let pat = match args.next() {
        Some(p) => p,
        None => Err("Usage: grep PATTERN FILE...")?,
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = std::io::stdin();
        // stdin.lock() return type implements BufRead
        grep(pat, stdin.lock())?;
    } else {
        for path in files {
            let file = std::fs::File::open(path)?;
            grep(&pat, BufReader::new(file))?;
        }
    }

    Ok(())
}

pub fn grep(pat: impl AsRef<str>, reader: impl BufRead) -> Result<(), std::io::Error> {
    let pat: &str = pat.as_ref();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(pat) {
            println!("{line}");
        }
    }
    Ok(())
}
