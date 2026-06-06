use serde_json::{Map, Value};

use crate::error::ParseError;
use crate::models::{Basics, CvDocument, Education, Experience, Project, Skill};

pub fn parse_json_cv(input: &str) -> Result<CvDocument, ParseError> {
    let value: Value = serde_json::from_str(input)?;
    normalize_cv(value)
}

fn normalize_cv(value: Value) -> Result<CvDocument, ParseError> {
    let root = value.as_object().ok_or(ParseError::UnsupportedTopLevel)?;

    let basics_node = get_object(root, &["basics", "profile", "contact"]);
    let basics = Basics {
        name: string_at_aliases(basics_node, &["name", "full_name", "fullName"])
            .or_else(|| string_at_aliases(Some(root), &["name", "full_name", "fullName"])),
        email: string_at_aliases(basics_node, &["email", "mail"])
            .or_else(|| string_at_aliases(Some(root), &["email", "mail"])),
        phone: string_at_aliases(basics_node, &["phone", "telephone", "mobile"])
            .or_else(|| string_at_aliases(Some(root), &["phone", "telephone", "mobile"])),
        location: string_at_aliases(basics_node, &["location", "city", "address"])
            .or_else(|| string_at_aliases(Some(root), &["location", "city", "address"])),
    };

    let summary = string_at_aliases(basics_node, &["summary", "profile", "objective"]) 
        .or_else(|| string_at_aliases(Some(root), &["summary", "profile", "objective"]));

    let experience = array_at_aliases(root, &["experience", "work", "employment", "positions"])
        .into_iter()
        .filter_map(parse_experience)
        .collect();

    let education = array_at_aliases(root, &["education", "schools", "academics"])
        .into_iter()
        .filter_map(parse_education)
        .collect();

    let skills = array_at_aliases(root, &["skills", "competencies", "technical_skills"])
        .into_iter()
        .flat_map(parse_skill)
        .collect();

    let projects = array_at_aliases(root, &["projects", "portfolio"])
        .into_iter()
        .filter_map(parse_project)
        .collect();

    Ok(CvDocument {
        basics,
        summary,
        experience,
        education,
        skills,
        projects,
    })
}

fn parse_experience(value: &Value) -> Option<Experience> {
    let object = value.as_object()?;

    Some(Experience {
        company: string_at_aliases(Some(object), &["company", "employer", "organization", "name"]),
        title: string_at_aliases(Some(object), &["title", "position", "role"]),
        start_date: string_at_aliases(Some(object), &["start_date", "startDate", "from", "start"]),
        end_date: string_at_aliases(Some(object), &["end_date", "endDate", "to", "end"]),
        summary: string_at_aliases(Some(object), &["summary", "description", "details"]),
        highlights: string_array_at_aliases(Some(object), &["highlights", "achievements", "responsibilities"]),
    })
}

fn parse_education(value: &Value) -> Option<Education> {
    let object = value.as_object()?;

    Some(Education {
        institution: string_at_aliases(Some(object), &["institution", "school", "university", "name"]),
        area: string_at_aliases(Some(object), &["area", "field", "major", "subject"]),
        study_type: string_at_aliases(Some(object), &["study_type", "studyType", "degree", "qualification"]),
        start_date: string_at_aliases(Some(object), &["start_date", "startDate", "from", "start"]),
        end_date: string_at_aliases(Some(object), &["end_date", "endDate", "to", "end"]),
        score: string_at_aliases(Some(object), &["score", "grade", "gpa"]),
    })
}

fn parse_skill(value: &Value) -> Vec<Skill> {
    match value {
        Value::String(name) => vec![Skill {
            name: name.trim().to_string(),
            level: None,
            keywords: Vec::new(),
        }],
        Value::Object(object) => {
            if let Some(name) = string_at_aliases(Some(object), &["name", "skill"]) {
                vec![Skill {
                    name,
                    level: string_at_aliases(Some(object), &["level", "proficiency"]),
                    keywords: string_array_at_aliases(Some(object), &["keywords", "items"]),
                }]
            } else if let Some(category) = string_at_aliases(Some(object), &["category", "group"]) {
                let keywords = string_array_at_aliases(Some(object), &["keywords", "items", "skills"]);

                if keywords.is_empty() {
                    Vec::new()
                } else {
                    vec![Skill {
                        name: category,
                        level: None,
                        keywords,
                    }]
                }
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
}

fn parse_project(value: &Value) -> Option<Project> {
    let object = value.as_object()?;

    Some(Project {
        name: string_at_aliases(Some(object), &["name", "title"]),
        description: string_at_aliases(Some(object), &["description", "summary"]),
        highlights: string_array_at_aliases(Some(object), &["highlights", "features"]),
        technologies: string_array_at_aliases(Some(object), &["technologies", "stack", "tools"]),
    })
}

fn get_object<'a>(map: &'a Map<String, Value>, aliases: &[&str]) -> Option<&'a Map<String, Value>> {
    aliases.iter().find_map(|key| map.get(*key)?.as_object())
}

fn array_at_aliases<'a>(map: &'a Map<String, Value>, aliases: &[&str]) -> &'a [Value] {
    aliases
        .iter()
        .find_map(|key| map.get(*key)?.as_array().map(Vec::as_slice))
        .unwrap_or(&[])
}

fn string_at_aliases(map: Option<&Map<String, Value>>, aliases: &[&str]) -> Option<String> {
    let object = map?;
    aliases.iter().find_map(|key| {
        object
            .get(*key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
    })
}

fn string_array_at_aliases(map: Option<&Map<String, Value>>, aliases: &[&str]) -> Vec<String> {
    let Some(object) = map else {
        return Vec::new();
    };

    aliases
        .iter()
        .find_map(|key| object.get(*key))
        .map(string_list)
        .unwrap_or_default()
}

fn string_list(value: &Value) -> Vec<String> {
    match value {
        Value::Array(items) => items
            .iter()
            .filter_map(Value::as_str)
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToString::to_string)
            .collect(),
        Value::String(items) => items
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToString::to_string)
            .collect(),
        _ => Vec::new(),
    }
}