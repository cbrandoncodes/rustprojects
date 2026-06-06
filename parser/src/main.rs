use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use cv_parser::{InputFormat, parse_cv};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliFormat {
    Auto,
    Json,
    Text,
}

impl From<CliFormat> for Option<InputFormat> {
    fn from(value: CliFormat) -> Self {
        match value {
            CliFormat::Auto => None,
            CliFormat::Json => Some(InputFormat::Json),
            CliFormat::Text => Some(InputFormat::Text),
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "cv-parser",
    version,
    about = "Normalize JSON or plain-text CVs into a standard JSON structure"
)]
struct Cli {
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    #[arg(long, value_enum, default_value_t = CliFormat::Auto)]
    format: CliFormat,

    #[arg(long, help = "Emit compact JSON instead of pretty-printed output")]
    compact: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let contents = fs::read_to_string(&cli.input)?;
    let cv = parse_cv(&contents, cli.format.into())?;

    if cli.compact {
        println!("{}", serde_json::to_string(&cv)?);
    } else {
        println!("{}", serde_json::to_string_pretty(&cv)?);
    }

    Ok(())
}
