use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

/// Generate a list of words that only contain the specified letters from a given dictionary.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file containing the words to filter, one per line.
    #[clap(short, long, value_parser, default_value = "dictionary.txt")]
    dictionary: String,

    /// The minimum size of words to filter for.
    #[clap(short, long, value_parser, default_value_t = 1)]
    min_length: u8,

    /// The characters you want to filter for. Example: "abc" will output words from the dictionary containing some combination of a, b, and/or c.
    #[clap(short, long, value_parser)]
    letters: String,

    /// The name of the filtered output file.
    #[clap(short, long, value_parser, default_value = "output.txt")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let mut filtered_words = Vec::new();
    let mut file = File::create(args.output).unwrap();
    if let Ok(lines) = read_lines(args.dictionary) {
        for line in lines {
            if let Ok(word) = line {
                if word_made_of_letters(&word, &args.letters)
                    && word.len() >= args.min_length.into()
                {
                    filtered_words.push(word);
                }
            }
        }
    }
    for word in filtered_words {
        writeln!(file, "{}", word).unwrap();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn word_made_of_letters(word: &str, letters: &str) -> bool {
    for letter in word.chars() {
        if !letters.contains(letter) {
            return false;
        }
    }
    true
}
