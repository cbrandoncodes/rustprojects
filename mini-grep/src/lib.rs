pub mod config;
pub mod error;

use std::error::Error;
use std::fs;

pub use config::Config;

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let mut matched: Vec<(usize, &str)> = if config.ignore_case {
        search_case_insensitive_with_line_numbers(&config.query, &contents)
    } else {
        search_with_line_numbers(&config.query, &contents)
    };

    if config.invert {
        matched = invert_matches(&contents, &matched);
    }

    let output = if config.context > 0 {
        let expanded = add_context(&contents, &matched, config.context);
        format_output(&expanded, config.show_line_numbers, config.count_only)
    } else {
        format_output(&matched, config.show_line_numbers, config.count_only)
    };

    Ok(output)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn search_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if line.contains(query) {
                Some((idx + 1, line))
            } else {
                None
            }
        })
        .collect()
}

pub fn search_case_insensitive_with_line_numbers<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if line.to_lowercase().contains(&query) {
                Some((idx + 1, line))
            } else {
                None
            }
        })
        .collect()
}

fn invert_matches<'a>(contents: &'a str, matches: &[(usize, &'a str)]) -> Vec<(usize, &'a str)> {
    let mut is_match = vec![false; contents.lines().count() + 1];
    for (line_no, _) in matches {
        is_match[*line_no] = true;
    }

    contents
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line_no = idx + 1;
            if !is_match[line_no] {
                Some((line_no, line))
            } else {
                None
            }
        })
        .collect()
}

fn add_context<'a>(
    contents: &'a str,
    matches: &[(usize, &'a str)],
    context: usize,
) -> Vec<(usize, &'a str)> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut selected = vec![false; lines.len() + 1];

    for (line_no, _) in matches {
        let start = line_no.saturating_sub(context).max(1);
        let end = (*line_no + context).min(lines.len());
        for idx in start..=end {
            selected[idx] = true;
        }
    }

    lines
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line_no = idx + 1;
            if selected[line_no] {
                Some((line_no, *line))
            } else {
                None
            }
        })
        .collect()
}

fn format_output(matches: &[(usize, &str)], show_line_numbers: bool, count_only: bool) -> String {
    if count_only {
        return matches.len().to_string();
    }

    matches
        .iter()
        .map(|(line_no, line)| {
            if show_line_numbers {
                format!("{line_no}: {line}")
            } else {
                (*line).to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
