//! Word count program in Rust

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::AddAssign;
use std::path::{Path, PathBuf};

use clap::Parser;

/// Command line arguments
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Files to count
    #[arg(name = "FILE")]
    filenames: Vec<PathBuf>,

    /// Count lines
    #[arg(short, long)]
    lines: bool,

    /// Count words
    #[arg(short, long)]
    words: bool,

    /// Count characters
    #[arg(short, long)]
    chars: bool,
}

/// File information
#[derive(Debug, Default, Clone, Copy)]
struct FileInfo {
    lines: usize,
    words: usize,
    chars: usize,
}

impl FileInfo {
    /// Compute file information
    fn from_filename(filename: &Path) -> Result<Self> {
        let file: Box<dyn BufRead> = if filename.as_os_str() == "-" {
            Box::new(BufReader::new(std::io::stdin()))
        } else {
            Box::new(BufReader::new(File::open(filename)?))
        };

        let reader = std::io::BufReader::new(file);
        let mut lines = 0;
        let mut words = 0;
        let mut chars = 0;

        for line in reader.lines() {
            let line = line?;

            lines += 1;
            words += line.split_whitespace().count();
            chars += line.chars().count() + 1;
        }

        Ok(Self {
            lines,
            words,
            chars,
        })
    }

    fn format(&self, show_lines: bool, show_words: bool, show_chars: bool) -> String {
        let mut fields = Vec::new();

        for (show, value) in &[
            (show_lines, self.lines),
            (show_words, self.words),
            (show_chars, self.chars),
        ] {
            if *show {
                fields.push(format!("{:8}", value));
            }
        }

        fields.join("")
    }
}

impl AddAssign for FileInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.lines += rhs.lines;
        self.words += rhs.words;
        self.chars += rhs.chars;
    }
}

/// Entry point of CLI
fn main() {
    let mut cli = Cli::parse();

    if !cli.lines && !cli.words && !cli.chars {
        cli.lines = true;
        cli.words = true;
        cli.chars = true;
    }

    if cli.filenames.is_empty() {
        cli.filenames.push(PathBuf::from("-"));
    }

    let mut total = FileInfo::default();

    for filename in cli.filenames.iter() {
        let info = match FileInfo::from_filename(filename) {
            Ok(info) => info,
            Err(err) => {
                eprintln!("{}: {}", filename.display(), err);
                continue;
            }
        };

        total += info;

        println!(
            "{} {}",
            info.format(cli.lines, cli.words, cli.chars),
            filename.display()
        );
    }

    if cli.filenames.len() > 1 {
        println!("{} total", total.format(cli.lines, cli.words, cli.chars));
    }
}
