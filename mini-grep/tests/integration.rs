use mini_grep::{Config, run, search, search_case_insensitive};

fn sample_config() -> Config {
    Config {
        query: "Rust".to_string(),
        file_path: "tests/data.txt".to_string(),
        ignore_case: false,
        show_line_numbers: true,
        count_only: false,
        invert: false,
        context: 0,
    }
}

#[test]
fn search_functions_work() {
    let contents = "Rust is fast\nI like Rust\nGo is cool";
    assert_eq!(
        vec!["Rust is fast", "I like Rust"],
        search("Rust", contents)
    );
    assert_eq!(
        vec!["Rust is fast", "I like Rust"],
        search_case_insensitive("rust", contents)
    );
}

#[test]
fn run_outputs_line_numbers() {
    let config = sample_config();
    let output = run(config).expect("run should succeed");

    assert_eq!("1: Rust is fast\n2: I like Rust", output);
}

#[test]
fn count_flag_returns_match_count() {
    let mut config = sample_config();
    config.count_only = true;

    let output = run(config).expect("run should succeed");
    assert_eq!("2", output);
}

#[test]
fn invert_flag_returns_non_matches() {
    let mut config = sample_config();
    config.invert = true;

    let output = run(config).expect("run should succeed");
    assert_eq!("3: Go is cool", output);
}

#[test]
fn context_flag_returns_neighboring_lines() {
    let mut config = sample_config();
    config.query = "I like".to_string();
    config.context = 1;

    let output = run(config).expect("run should succeed");
    assert_eq!("1: Rust is fast\n2: I like Rust\n3: Go is cool", output);
}
