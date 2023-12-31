use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use futures::stream::FuturesUnordered;
use mokareads_macros::EnumVariants;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

use crate::resources::ResourceType;

use super::Parser as CheatsheetParser;
use super::SearchMetadata;

/// # MoKa Reads Cheatsheet
///
/// Cheat sheets are a great way to quickly learn something new. They are also a great way to refresh your memory on something you've already learned.
/// In our website, we seperate the cheat sheets by language, and then by level of difficulty. This allows for users to see the
/// beginner, intermediate, and advanced cheat sheets for a language.
///
/// ## Markdown Format:
///
/// ```markdown
/// ---
/// title: My Cheat Sheet
/// author: John Doe
/// level: 1
/// language: python
/// icon: devicon
/// ---
/// Content of the cheat sheet
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Cheatsheet {
    metadata: Metadata,
    pub slug: String,
    content: String,
}

/// Used for creating a new cheatsheet file
impl Cheatsheet {
    pub fn new(metadata: Metadata, content: String) -> Self {
        let slug = metadata.title.replace(' ', "_");
        Self {
            metadata,
            slug,
            content,
        }
    }
    /// Asynchronously fetches a cheatsheet from a list of cheatsheets
    pub async fn fetch(cheatsheets: &[Self], slug: String, lang: String) -> Self {
        let unordered = FuturesUnordered::from_iter(cheatsheets)
            .iter()
            .find(|x| x.slug == slug && x.lang() == lang)
            .cloned();
        match unordered {
            Some(c) => c.clone(),
            None => Cheatsheet::default(),
        }
    }
    /// Synchronously finds a cheatsheet from a list of cheatsheets
    /// Reason of having a dedicated function is to be able to control the method
    pub fn find(cheatsheets: &[Self], slug: String, lang: String) -> Self {
        cheatsheets
            .iter()
            .find(|x| x.slug == slug && x.lang() == lang)
            .cloned()
            .unwrap_or_default()
    }
    pub fn to_markdown(&self) -> String {
        let mut parts = Vec::new();
        parts.push("---".to_string());
        let metadata = serde_yaml::to_string(&self.metadata).unwrap();
        parts.push(metadata);
        parts.push("---".to_string());

        parts.push(self.content.to_string());

        parts.join("\n")
    }
    pub fn lang(&self) -> String {
        self.metadata.lang.clone()
    }
    pub fn title(&self) -> String {
        self.metadata.title.to_string()
    }
    pub fn link_short(&self) -> String {
        format!("/cheatsheets/{}", &self.slug)
    }
    pub fn as_search_meta(&self) -> SearchMetadata {
        SearchMetadata::new(
            self.title(),
            ResourceType::Cheatsheet,
            self.link_short(),
            self.lang(),
        )
    }
}

/// # Cheatsheet Metadata
///
/// The metadata section is a YAML document which contains the following fields:
///
/// - `title`: The title of the cheat sheet.
/// - `author`: The author of the cheat sheet.
/// - `level`: The level of the cheat sheet (1, 2, or 3).
///   - 1: Beginner
///   - 2: Intermediate
///   - 3: Advanced
/// - `language`: The language of the cheat sheet.
/// - `icon`: The icon to use for the cheat sheet (`devicon` or `fontawesome5`).
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Metadata {
    title: String,
    author: String,
    level: u8,
    lang: String,
    icon: String,
}

impl Metadata {
    pub fn new(title: &str, author: &str, level: u8, lang: &str, icon: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            level,
            lang: lang.to_string(),
            icon: icon.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Level {
    Beginner = 1,
    Intermediate = 2,
    Advanced = 3,
}

impl Level {
    fn from_u8(value: u8) -> Option<Level> {
        match value {
            1 => Some(Level::Beginner),
            2 => Some(Level::Intermediate),
            3 => Some(Level::Advanced),
            _ => None,
        }
    }
}

impl CheatsheetParser for Cheatsheet {
    fn parse_raw(markdown: &str) -> Self
        where
            Self: Sized,
    {
        let separator = "---";
        let mut sections = markdown.splitn(3, separator);
        sections.next();
        let yaml_section = sections.next().unwrap_or("");
        let content_section = sections.next().unwrap_or("");

        let mut metadata = serde_yaml::from_str::<Metadata>(yaml_section).unwrap();
        if Level::from_u8(metadata.level).is_none() {
            metadata.level = 1;
        }
        let slug = metadata.title.replace(' ', "_");
        Self {
            metadata: metadata.clone(),
            slug,
            content: content_section.to_string(),
        }
    }

    fn parse(markdown: &str) -> Self
        where
            Self: Sized,
    {
        let mut cheatsheet = Self::parse_raw(markdown);
        let parser = Parser::new_ext(cheatsheet.content.as_str(), Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        cheatsheet.content = html_output;
        cheatsheet
    }
    fn raw_to_parsed(&self) -> Self
        where
            Self: Sized,
    {
        let parser = Parser::new_ext(&self.content, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Self {
            metadata: self.metadata.clone(),
            slug: self.slug.to_string(),
            content: html_output,
        }
    }
}

#[derive(
Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize, EnumVariants, Hash,
)]
pub enum Language {
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

impl Language {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Language {
        match s {
            "kotlin" => Language::Kotlin,
            "rust" => Language::Rust,
            "c" => Language::C,
            "c++" => Language::CPP,
            "zig" => Language::Zig,
            "python" => Language::Python,
            "swift" => Language::Swift,
            "go" => Language::Go,
            _ => Language::Other,
        }
    }
    pub fn icon_suggestion(&self) -> String {
        match self {
            Language::Kotlin => "devicon-kotlin-plain".to_string(),
            Language::Rust => "devicon-rust-plain".to_string(),
            Language::C => "devicon-c-plain".to_string(),
            Language::CPP => "devicon-cplusplus-plain".to_string(),
            Language::Zig => "devicon-zig-original".to_string(),
            Language::Python => "devicon-python-plain".to_string(),
            Language::Swift => "devicon-swift-plain".to_string(),
            Language::Go => "devicon-go-original-wordmark".to_string(),
            Language::Other => "devicon-github-original".to_string(),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::Kotlin => "kotlin",
            Language::Rust => "rust",
            Language::C => "c",
            Language::CPP => "c++",
            Language::Zig => "zig",
            Language::Python => "python",
            Language::Swift => "swift",
            Language::Go => "go",
            Language::Other => "other",
        };
        write!(f, "{}", s)
    }
}

fn get_lang_vec(lang: Language, cheatsheets: &[Cheatsheet]) -> Vec<Cheatsheet> {
    cheatsheets
        .iter()
        .filter(|x| Language::from_str(&x.metadata.lang) == lang)
        .cloned()
        .collect()
}

pub fn get_lang_map(cheatsheets: &[Cheatsheet]) -> HashMap<Language, Vec<Cheatsheet>> {
    let mut map = HashMap::new();
    let lang_vec: Vec<Language> = Language::all_variants();
    for lang in lang_vec {
        let mut vec = get_lang_vec(lang, cheatsheets);
        sort_cheatsheets(&mut vec);
        map.insert(lang, vec);
    }
    map
}

fn sort_cheatsheets(cheatsheets: &mut [Cheatsheet]) {
    cheatsheets.sort_by(|a, b| {
        let lev_a = Level::from_u8(a.metadata.level).unwrap();
        let lev_b = Level::from_u8(b.metadata.level).unwrap();
        lev_a.cmp(&lev_b)
    });
}
