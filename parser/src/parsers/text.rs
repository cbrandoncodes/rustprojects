use once_cell::sync::Lazy;
use regex::Regex;

use crate::models::{Basics, CvDocument, Education, Experience, Project, Skill};

static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b").expect("valid email regex")
});
static PHONE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?x)
        (?:\+?\d{1,3}[\s.-]?)?
        (?:\(?\d{2,4}\)?[\s.-]?)
        \d{3,4}[\s.-]?\d{3,4}
    ").expect("valid phone regex")
});
static DATE_RANGE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?ix)
        (?P<start>(?:[A-Z][a-z]{2,8}\s+)?\d{4}|present|current)
        \s*[-–]\s*
        (?P<end>(?:[A-Z][a-z]{2,8}\s+)?\d{4}|present|current)
        $")
    .expect("valid date range regex")
});

pub fn parse_text_cv(input: &str) -> CvDocument {
    let normalized_lines: Vec<&str> = input.lines().map(str::trim_end).collect();
    let non_empty_lines: Vec<&str> = normalized_lines
        .iter()
        .copied()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let name = non_empty_lines
        .first()
        .map(|line| line.trim().to_string())
        .filter(|line| !looks_like_contact(line));

    let email = EMAIL_RE.find(input).map(|capture| capture.as_str().to_string());
    let phone = PHONE_RE
        .find(input)
        .map(|capture| capture.as_str().trim().to_string())
        .filter(|value| value.chars().filter(|ch| ch.is_ascii_digit()).count() >= 7);

    let mut preamble = Vec::new();
    let mut summary_lines = Vec::new();
    let mut experience_lines = Vec::new();
    let mut education_lines = Vec::new();
    let mut skills_lines = Vec::new();
    let mut project_lines = Vec::new();
    let mut current_section = Section::Preamble;

    for line in normalized_lines {
        let trimmed = line.trim();

        if let Some(section) = Section::from_heading(trimmed) {
            current_section = section;
            continue;
        }

        match current_section {
            Section::Preamble => preamble.push(trimmed.to_string()),
            Section::Summary => summary_lines.push(trimmed.to_string()),
            Section::Experience => experience_lines.push(trimmed.to_string()),
            Section::Education => education_lines.push(trimmed.to_string()),
            Section::Skills => skills_lines.push(trimmed.to_string()),
            Section::Projects => project_lines.push(trimmed.to_string()),
        }
    }

    let summary = join_summary(&summary_lines).or_else(|| derive_summary(&preamble, name.as_deref()));
    let location = detect_location(&preamble, name.as_deref(), email.as_deref(), phone.as_deref());

    CvDocument {
        basics: Basics {
            name,
            email,
            phone,
            location,
        },
        summary,
        experience: parse_experience_blocks(&experience_lines),
        education: parse_education_blocks(&education_lines),
        skills: parse_skills(&skills_lines),
        projects: parse_project_blocks(&project_lines),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Preamble,
    Summary,
    Experience,
    Education,
    Skills,
    Projects,
}

impl Section {
    fn from_heading(line: &str) -> Option<Self> {
        let normalized = line
            .chars()
            .filter(|ch| ch.is_ascii_alphanumeric() || ch.is_ascii_whitespace())
            .collect::<String>()
            .to_ascii_lowercase();

        match normalized.trim() {
            "summary" | "profile" | "professional summary" | "objective" => Some(Self::Summary),
            "experience" | "work experience" | "professional experience" | "employment" => {
                Some(Self::Experience)
            }
            "education" | "academic background" => Some(Self::Education),
            "skills" | "technical skills" | "core competencies" => Some(Self::Skills),
            "projects" | "selected projects" | "portfolio" => Some(Self::Projects),
            _ => None,
        }
    }
}

fn looks_like_contact(line: &str) -> bool {
    EMAIL_RE.is_match(line) || PHONE_RE.is_match(line)
}

fn join_summary(lines: &[String]) -> Option<String> {
    let combined = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    if combined.is_empty() {
        None
    } else {
        Some(combined)
    }
}

fn derive_summary(preamble: &[String], name: Option<&str>) -> Option<String> {
    let filtered = preamble
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| Some(*line) != name)
        .filter(|line| !looks_like_contact(line))
        .collect::<Vec<_>>();

    if filtered.is_empty() {
        None
    } else {
        Some(filtered.join(" "))
    }
}

fn detect_location(
    preamble: &[String],
    name: Option<&str>,
    email: Option<&str>,
    phone: Option<&str>,
) -> Option<String> {
    preamble
        .iter()
        .map(|line| line.trim())
        .find(|line| {
            !line.is_empty()
                && Some(*line) != name
                && Some(*line) != email
                && Some(*line) != phone
                && !looks_like_contact(line)
                && line.split(',').count() >= 2
        })
        .map(ToString::to_string)
}

fn parse_experience_blocks(lines: &[String]) -> Vec<Experience> {
    split_blocks(lines)
        .into_iter()
        .filter_map(|block| {
            let mut entries = block.into_iter().filter(|line| !line.trim().is_empty());
            let header = entries.next()?;
            let (header, start_date, end_date) = extract_date_range(&header);
            let (title, company) = split_header(&header);
            let mut summary_lines = Vec::new();
            let mut highlights = Vec::new();

            for line in entries {
                let trimmed = line.trim();
                if trimmed.starts_with('-') || trimmed.starts_with('*') {
                    highlights.push(trimmed[1..].trim().to_string());
                } else {
                    summary_lines.push(trimmed.to_string());
                }
            }

            Some(Experience {
                company,
                title,
                start_date,
                end_date,
                summary: if summary_lines.is_empty() {
                    None
                } else {
                    Some(summary_lines.join(" "))
                },
                highlights,
            })
        })
        .collect()
}

fn parse_education_blocks(lines: &[String]) -> Vec<Education> {
    split_blocks(lines)
        .into_iter()
        .filter_map(|block| {
            let mut entries = block.into_iter().filter(|line| !line.trim().is_empty());
            let header = entries.next()?;
            let (header, start_date, end_date) = extract_date_range(&header);
            let (study_type, institution) = split_header(&header);
            let remainder = entries.collect::<Vec<_>>().join(" ");

            Some(Education {
                institution,
                area: if remainder.is_empty() { None } else { Some(remainder) },
                study_type,
                start_date,
                end_date,
                score: None,
            })
        })
        .collect()
}

fn parse_project_blocks(lines: &[String]) -> Vec<Project> {
    split_blocks(lines)
        .into_iter()
        .filter_map(|block| {
            let mut entries = block.into_iter().filter(|line| !line.trim().is_empty());
            let header = entries.next()?;
            let mut description_lines = Vec::new();
            let mut highlights = Vec::new();

            for line in entries {
                let trimmed = line.trim();
                if trimmed.starts_with('-') || trimmed.starts_with('*') {
                    highlights.push(trimmed[1..].trim().to_string());
                } else {
                    description_lines.push(trimmed.to_string());
                }
            }

            Some(Project {
                name: Some(header.trim().to_string()),
                description: if description_lines.is_empty() {
                    None
                } else {
                    Some(description_lines.join(" "))
                },
                highlights,
                technologies: Vec::new(),
            })
        })
        .collect()
}

fn parse_skills(lines: &[String]) -> Vec<Skill> {
    let mut skills = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some((category, values)) = trimmed.split_once(':') {
            let keywords = values
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>();

            if !keywords.is_empty() {
                skills.push(Skill {
                    name: category.trim().to_string(),
                    level: None,
                    keywords,
                });
            }

            continue;
        }

        skills.extend(
            trimmed
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|name| Skill {
                    name: name.to_string(),
                    level: None,
                    keywords: Vec::new(),
                }),
        );
    }

    skills
}

fn split_blocks(lines: &[String]) -> Vec<Vec<String>> {
    let mut blocks = Vec::new();
    let mut current = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(std::mem::take(&mut current));
            }
        } else {
            current.push(line.clone());
        }
    }

    if !current.is_empty() {
        blocks.push(current);
    }

    blocks
}

fn extract_date_range(header: &str) -> (String, Option<String>, Option<String>) {
    let trimmed = header.trim();

    if let Some(captures) = DATE_RANGE_RE.captures(trimmed) {
        let whole_match = captures.get(0).map(|value| value.as_str()).unwrap_or_default();
        let head = trimmed.trim_end_matches(whole_match).trim().trim_end_matches('|').trim();
        let start_date = captures.name("start").map(|value| value.as_str().to_string());
        let end_date = captures.name("end").map(|value| value.as_str().to_string());
        (head.to_string(), start_date, end_date)
    } else {
        (trimmed.to_string(), None, None)
    }
}

fn split_header(header: &str) -> (Option<String>, Option<String>) {
    for separator in [" at ", " | ", " - ", ", "] {
        if let Some((left, right)) = header.split_once(separator) {
            let left = left.trim();
            let right = right.trim();

            if !left.is_empty() || !right.is_empty() {
                return (
                    (!left.is_empty()).then(|| left.to_string()),
                    (!right.is_empty()).then(|| right.to_string()),
                );
            }
        }
    }

    (Some(header.trim().to_string()), None)
}