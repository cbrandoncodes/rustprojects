pub mod error;
pub mod models;
pub mod parsers;

pub use error::ParseError;
pub use models::{Basics, CvDocument, Education, Experience, Project, Skill};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Json,
    Text,
}

pub fn parse_cv(input: &str, format: Option<InputFormat>) -> Result<CvDocument, ParseError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    match format.unwrap_or_else(|| detect_format(trimmed)) {
        InputFormat::Json => parsers::json::parse_json_cv(trimmed),
        InputFormat::Text => Ok(parsers::text::parse_text_cv(trimmed)),
    }
}

fn detect_format(input: &str) -> InputFormat {
    if input.starts_with('{') || input.starts_with('[') {
        InputFormat::Json
    } else {
        InputFormat::Text
    }
}