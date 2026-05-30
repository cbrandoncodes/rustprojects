use std::env;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub show_line_numbers: bool,
    pub count_only: bool,
    pub invert: bool,
    pub context: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let mut args = env::args().skip(1);

        let query = args.next().ok_or("missing query argument")?;
        let file_path = args.next().ok_or("missing file path argument")?;

        let mut ignore_case = env::var("IGNORE_CASE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let mut show_line_numbers = true;
        let mut count_only = false;
        let mut invert = false;
        let mut context = 0usize;

        let rest: Vec<String> = args.collect();
        let mut idx = 0usize;
        while idx < rest.len() {
            match rest[idx].as_str() {
                "--ignore-case" => ignore_case = true,
                "--no-line-numbers" => show_line_numbers = false,
                "--count" => count_only = true,
                "--invert" => invert = true,
                "--context" => {
                    let value = rest.get(idx + 1).ok_or("--context requires a number")?;
                    context = value
                        .parse::<usize>()
                        .map_err(|_| "--context expects a non-negative integer")?;
                    idx += 1;
                }
                flag => return Err(format!("unknown option: {flag}").into()),
            }
            idx += 1;
        }

        Ok(Self {
            query,
            file_path,
            ignore_case,
            show_line_numbers,
            count_only,
            invert,
            context,
        })
    }
}
