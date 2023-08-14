use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use super::Parser as CheatsheetParser;

use std::collections::HashMap;
use mokareads_macros::EnumVariants;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cheatsheet{
    metadata: Metadata,
    pub slug: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Metadata {
    title: String,
    author: String,
    level: u8,
    lang: String,
    icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Level{
    Beginner = 1,
    Intermediate = 2,
    Advanced = 3,
}

impl Level{
    fn from_u8(value: u8) -> Option<Level>{
        match value{
            1 => Some(Level::Beginner),
            2 => Some(Level::Intermediate),
            3 => Some(Level::Advanced),
            _ => None,
        }
    }
}

impl CheatsheetParser for Cheatsheet{
    fn parse(markdown: &str) -> Self where Self: Sized {
        let separator = "---";
        let mut sections = markdown.splitn(3, separator);
        sections.next();
        let yaml_section = sections.next().unwrap_or("");
        let content_section = sections.next().unwrap_or("");

        let mut metadata = serde_yaml::from_str::<Metadata>(yaml_section).unwrap();
        if let None = Level::from_u8(metadata.level){
            metadata.level = 1;
        }
        let parser = Parser::new_ext(&content_section, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        let slug = metadata.title.replace(" ", "_");
        Self {
            metadata,
            slug,
            content: html_output,
        }
    }
}

#[derive(Debug,Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize, EnumVariants, Hash)]
pub enum Language{
    Kotlin,
    Rust,
    C,
    CPP,
    Zig,
    Python,
    Swift,
    Go,
    Other,
}

impl Language{
    pub fn from_str(s: &str) -> Language{
        match s{
            "kotlin" => Language::Kotlin,
            "rust" => Language::Rust,
            "c" => Language::C,
            "c++" => Language::CPP,
            "zig" => Language::Zig,
            "python" => Language::Python,
            "swift" => Language::Swift,
            "go" => Language::Go,
            _ => Language::Other
        }
    }
}

fn get_lang_vec(lang: Language, cheatsheets: &Vec<Cheatsheet>) -> Vec<Cheatsheet>{
    cheatsheets.iter().filter(|x|{
        Language::from_str(&x.metadata.lang) == lang
    }).cloned().collect()
}

pub fn get_lang_map(cheatsheets: &Vec<Cheatsheet>) -> HashMap<Language, Vec<Cheatsheet>>{
    let mut map = HashMap::new();
    let lang_vec: Vec<Language> = Language::all_variants();
    for lang in lang_vec{
        let mut vec = get_lang_vec(lang, cheatsheets);
        sort_cheatsheets(&mut vec);
        map.insert(lang, vec);
    }
    map
}

fn sort_cheatsheets(cheatsheets: &mut Vec<Cheatsheet>){
    cheatsheets.sort_by(
        |a, b|{
            let lev_a = Level::from_u8(a.metadata.level).unwrap();
            let lev_b = Level::from_u8(b.metadata.level).unwrap();
            lev_a.cmp(&lev_b)
        });
}

#[test]
fn test_sort_cheatsheets(){
    let mut cheatsheets = vec![
        Cheatsheet {
            metadata: Metadata {
                title: "Title A".to_string(),
                author: "Author A".to_string(),
                level: 3,
                lang: "English".to_string(),
                icon: "Icon A".to_string(),
            },
            slug: "slug-3".to_string(),
            content: "Content A".to_string(),
        },
        Cheatsheet {
            metadata: Metadata {
                title: "Title B".to_string(),
                author: "Author B".to_string(),
                level: 1,
                lang: "Spanish".to_string(),
                icon: "Icon B".to_string(),
            },
            slug: "slug-1".to_string(),
            content: "Content B".to_string(),
        },
        Cheatsheet {
            metadata: Metadata {
                title: "Title C".to_string(),
                author: "Author C".to_string(),
                level: 2,
                lang: "French".to_string(),
                icon: "Icon C".to_string(),
            },
            slug: "slug-2".to_string(),
            content: "Content C".to_string(),
        },
    ];

    sort_cheatsheets(&mut cheatsheets);

    let expected_sorted_cheatsheets = vec![
        Cheatsheet {
            metadata: Metadata {
                title: "Title B".to_string(),
                author: "Author B".to_string(),
                level: 1,
                lang: "Spanish".to_string(),
                icon: "Icon B".to_string(),
            },
            slug: "slug-1".to_string(),
            content: "Content B".to_string(),
        },
        Cheatsheet {
            metadata: Metadata {
                title: "Title C".to_string(),
                author: "Author C".to_string(),
                level: 2,
                lang: "French".to_string(),
                icon: "Icon C".to_string(),
            },
            slug: "slug-2".to_string(),
            content: "Content C".to_string(),
        },
        Cheatsheet {
            metadata: Metadata {
                title: "Title A".to_string(),
                author: "Author A".to_string(),
                level: 3,
                lang: "English".to_string(),
                icon: "Icon A".to_string(),
            },
            slug: "slug-3".to_string(),
            content: "Content A".to_string(),
        },
    ];

    // Use assert_eq! to compare the sorted vector with the expected result
    assert_eq!(cheatsheets, expected_sorted_cheatsheets);
}