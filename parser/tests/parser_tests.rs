use cv_parser::{InputFormat, parse_cv};

#[test]
fn normalizes_common_json_resume_aliases() {
    let input = r#"
    {
        "full_name": "Alex Carter",
        "email": "alex@example.com",
        "phone": "+1 555 010 9999",
        "profile": "Platform engineer with a focus on reliability.",
        "work": [
            {
                "employer": "Acme Corp",
                "role": "Senior Engineer",
                "from": "2021",
                "to": "Present",
                "description": "Built internal developer tooling.",
                "achievements": ["Cut build time by 40%", "Introduced typed CI pipelines"]
            }
        ],
        "education": [
            {
                "school": "State University",
                "degree": "BSc",
                "major": "Computer Science",
                "end": "2020"
            }
        ],
        "skills": [
            "Rust",
            { "name": "Distributed Systems", "level": "advanced", "keywords": ["Kafka", "gRPC"] }
        ]
    }
    "#;

    let cv = parse_cv(input, Some(InputFormat::Json)).expect("json CV should parse");

    assert_eq!(cv.basics.name.as_deref(), Some("Alex Carter"));
    assert_eq!(cv.summary.as_deref(), Some("Platform engineer with a focus on reliability."));
    assert_eq!(cv.experience.len(), 1);
    assert_eq!(cv.experience[0].company.as_deref(), Some("Acme Corp"));
    assert_eq!(cv.experience[0].title.as_deref(), Some("Senior Engineer"));
    assert_eq!(cv.skills.len(), 2);
    assert_eq!(cv.skills[0].name, "Rust");
}

#[test]
fn parses_plain_text_cv_sections() {
    let input = r#"
Alex Carter
Seattle, WA
alex@example.com
+1 555 010 9999

Summary
Platform engineer building resilient backend systems.

Experience
Senior Engineer at Acme Corp | 2021 - Present
Built internal developer tooling.
- Cut build time by 40%
- Introduced typed CI pipelines

Education
BSc, State University | 2016 - 2020
Computer Science

Skills
Languages: Rust, Go, TypeScript
Observability, CI/CD

Projects
Release Orchestrator
Automated multi-service releases.
- Coordinated deployments across 20 services
"#;

    let cv = parse_cv(input, Some(InputFormat::Text)).expect("text CV should parse");

    assert_eq!(cv.basics.name.as_deref(), Some("Alex Carter"));
    assert_eq!(cv.basics.location.as_deref(), Some("Seattle, WA"));
    assert_eq!(cv.experience.len(), 1);
    assert_eq!(cv.experience[0].title.as_deref(), Some("Senior Engineer"));
    assert_eq!(cv.experience[0].company.as_deref(), Some("Acme Corp"));
    assert_eq!(cv.education.len(), 1);
    assert_eq!(cv.projects.len(), 1);
    assert_eq!(cv.skills.len(), 3);
    assert_eq!(cv.skills[0].name, "Languages");
    assert_eq!(cv.skills[0].keywords, vec!["Rust", "Go", "TypeScript"]);
}