use clap::{Parser, ValueEnum};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "wordperms",
    version = "1.0",
    author = "Attila Repka",
    about = "Generate word permutations"
)]
struct Args {
    /// Input file (one word per line)
    #[arg(short, long)]
    input: PathBuf,

    /// Max number of words per combination
    #[arg(short = 'm', long, default_value_t = 4)]
    max_len: usize,

    /// Capitalization style
    #[arg(short, long, value_enum, default_value_t = Capitalization::All)]
    cap_style: Capitalization,

    /// Limit number of generated results
    #[arg(short, long)]
    limit: Option<usize>,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
enum Capitalization {
    #[default]
    All,
    None,
    First,
    Upper,
}

fn capitalize_variants(word: &str, style: Capitalization) -> Vec<String> {
    match style {
        Capitalization::None => vec![word.to_string()],
        Capitalization::First => vec![capitalize_first_letter(word)],
        Capitalization::Upper => vec![word.to_uppercase()],
        Capitalization::All => vec![
            word.to_string(),
            word.to_uppercase(),
            capitalize_first_letter(word),
        ],
    }
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn generate_permutations(words: &[String], len: usize, cap_style: Capitalization) -> Vec<String> {
    let results: HashSet<String> = (1..=len.min(words.len()))
        .flat_map(|len| {
            words
                .iter()
                .combinations(len)
                .par_bridge()
                .map(|combo| {
                    let mut local = HashSet::new();
                    let variants_per_word: Vec<Vec<String>> = combo
                        .iter()
                        .map(|w| capitalize_variants(w, cap_style))
                        .collect();

                    for capitalized in variants_per_word.iter().multi_cartesian_product() {
                        for perm in capitalized.iter().permutations(len) {
                            local.insert(perm.iter().map(|s| s.as_str()).collect::<String>());
                        }
                    }

                    local
                })
                .reduce(HashSet::new, |mut a, b| {
                    a.extend(b);
                    a
                })
        })
        .collect();

    results.into_iter().collect()
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let reader = BufReader::new(file);
    let input_words: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.trim().is_empty())
        .collect();

    let mut results = generate_permutations(&input_words, args.max_len, args.cap_style);
    if let Some(limit) = args.limit {
        results.truncate(limit);
    }

    match &args.output {
        Some(path) => {
            let mut file = File::create(path)?;
            for line in results {
                writeln!(file, "{line}")?;
            }
        }
        None => {
            for line in results {
                println!("{line}");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first_letter() {
        assert_eq!(capitalize_first_letter("hello"), "Hello");
        assert_eq!(capitalize_first_letter("world"), "World");
    }

    #[test]
    fn test_capitalize_variants() {
        assert_eq!(
            capitalize_variants("hello", Capitalization::None),
            vec!["hello"]
        );
        assert_eq!(
            capitalize_variants("hello", Capitalization::First),
            vec!["Hello"]
        );
        assert_eq!(
            capitalize_variants("hello", Capitalization::Upper),
            vec!["HELLO"]
        );
        assert_eq!(
            capitalize_variants("hello", Capitalization::All),
            vec!["hello", "HELLO", "Hello"]
        );
    }

    #[test]
    fn test_generate_permutations() {
        let mut actual =
            generate_permutations(&["a".to_string(), "b".to_string()], 2, Capitalization::None);
        actual.sort_unstable();
        assert_eq!(actual, vec!["a", "ab", "b", "ba"]);
        let mut actual = generate_permutations(
            &["a".to_string(), "b".to_string()],
            2,
            Capitalization::First,
        );
        actual.sort_unstable();
        assert_eq!(actual, vec!["A", "AB", "B", "BA"]);

        let mut actual = generate_permutations(
            &["a".to_string(), "b".to_string()],
            2,
            Capitalization::Upper,
        );
        actual.sort_unstable();
        assert_eq!(actual, vec!["A", "AB", "B", "BA"]);
        let mut actual =
            generate_permutations(&["a".to_string(), "b".to_string()], 2, Capitalization::All);
        actual.sort_unstable();
        assert_eq!(
            actual,
            vec![
                "A", "AB", "Ab", "B", "BA", "Ba", "a", "aB", "ab", "b", "bA", "ba"
            ]
        );
    }

    #[test]
    fn test_max_len() {
        let mut actual =
            generate_permutations(&["a".to_string(), "b".to_string()], 1, Capitalization::None);
        actual.sort_unstable();
        assert_eq!(actual, vec!["a", "b"]);
        let mut actual =
            generate_permutations(&["a".to_string(), "b".to_string()], 2, Capitalization::None);
        actual.sort_unstable();
        assert_eq!(actual, vec!["a", "ab", "b", "ba"]);
    }
}
