use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use article::Article;
use cheatsheet::Cheatsheet;
use guide::Guide;

use crate::resources::cheatsheet::{get_lang_map, Language};

/// MoKa Reads Article
pub mod article;
/// MoKa Reads Cheatsheets
pub mod cheatsheet;
/// MoKa Reads How to Guides
pub mod guide;

/// Parses MarkDown to a type
pub trait Parser {
    /// Parses the markdown file and keeps the markdown content
    fn parse_raw(markdown: &str) -> Self
        where
            Self: Sized;
    /// Parses the markdown file and parses the markdown content to html
    fn parse(markdown: &str) -> Self
        where
            Self: Sized;
    /// Converts a `raw` version to a `parsed` version.
    fn raw_to_parsed(&self) -> Self
        where
            Self: Sized;
}

/// A type to store all different resources with a time of update to show when resources were last cached
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Cacher {
    updated_at: String,
    articles: Vec<Article>,
    cheatsheets: Vec<Cheatsheet>,
    guides: Vec<Guide>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResourceType {
    Article,
    Cheatsheet,
    Guide,
}

impl FromStr for ResourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "article" => Ok(ResourceType::Article),
            "cheatsheet" => Ok(ResourceType::Cheatsheet),
            "guide" => Ok(ResourceType::Guide),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchMetadata {
    title: String,
    ty: ResourceType,
    link: String,
}

impl SearchMetadata {
    pub fn new(title: String, ty: ResourceType, link: String) -> Self {
        Self { title, ty, link }
    }
}

/// Hashmaps for quick navigation
/// A user will be able to search for something under the following conditions:
/// - Language of Focus
/// - Title of Resource
/// - Resource Type
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Searcher {
    langs: HashMap<Language, Vec<SearchMetadata>>,
    titles: HashMap<String, Vec<SearchMetadata>>,
    rty: HashMap<ResourceType, Vec<SearchMetadata>>,
}

impl Searcher {
    pub fn new(cacher: &Cacher) -> Self {
        let mut langs = HashMap::new();
        let mut titles = HashMap::new();
        let mut rty = HashMap::new();

        // Collect unique titles from various sources
        let mut titles_chain: Vec<String> = cacher
            .articles
            .iter()
            .map(|x| x.title())
            .chain(cacher.cheatsheets.iter().map(|x| x.title()))
            .chain(cacher.guides.iter().map(|x| x.unslug.clone()))
            .collect();
        titles_chain.sort();
        titles_chain.dedup();

        // Generate search metadata for different languages
        let lang_vec = Language::all_variants();
        let lang_map_cheatsheets = get_lang_map(&cacher.cheatsheets);

        for lang in lang_vec {
            let cheatsheet = lang_map_cheatsheets.get(&lang).unwrap();
            let mut search_metas: Vec<SearchMetadata> =
                cheatsheet.iter().map(|x| x.as_search_meta()).collect();

            search_metas.extend(
                cacher
                    .articles
                    .iter()
                    .filter(|x| x.lang_in_tag(lang))
                    .map(|x| x.as_search_meta()),
            );
            langs.insert(lang, search_metas);
        }

        // Generate search metadata for titles
        for title in &titles_chain {
            let mut search_metas = Vec::new();

            search_metas.extend(
                cacher
                    .cheatsheets
                    .iter()
                    .filter(|x| &x.title() == title)
                    .map(|x| x.as_search_meta()),
            );
            search_metas.extend(
                cacher
                    .articles
                    .iter()
                    .filter(|x| &x.title() == title)
                    .map(|x| x.as_search_meta()),
            );
            search_metas.extend(
                cacher
                    .guides
                    .iter()
                    .filter(|x| &x.unslug == title)
                    .map(|x| x.as_search_meta()),
            );

            titles.insert(title.clone(), search_metas);
        }

        // Generate search metadata for resource types
        rty.insert(
            ResourceType::Cheatsheet,
            cacher
                .cheatsheets
                .iter()
                .map(|x| x.as_search_meta())
                .collect(),
        );
        rty.insert(
            ResourceType::Article,
            cacher.articles.iter().map(|x| x.as_search_meta()).collect(),
        );
        rty.insert(
            ResourceType::Guide,
            cacher.guides.iter().map(|x| x.as_search_meta()).collect(),
        );

        Self { langs, titles, rty }
    }

    pub fn search(&self, input: String) -> Vec<SearchMetadata> {
        let lowercase_input = input.to_lowercase();
        let lang_input = Language::from_str(&lowercase_input);
        if let Some(metadata) = self.titles.get(&input) {
            return metadata.clone();
        } else if lang_input != Language::Other {
            if let Some(metadata) = self.langs.get(&lang_input) {
                return metadata.clone();
            }
        } else if let Some(metadata) = self
            .rty
            .get(&ResourceType::from_str(&lowercase_input).unwrap())
        {
            return metadata.clone();
        }

        Vec::new()
    }
}

impl Cacher {
    pub fn new(articles: Vec<Article>, cheatsheets: Vec<Cheatsheet>, guides: Vec<Guide>) -> Self {
        let updated_at = chrono::Utc::now().to_string();
        Self {
            updated_at,
            articles,
            cheatsheets,
            guides,
        }
    }

    pub fn articles(&self) -> Vec<Article> {
        self.articles.clone()
    }
    pub fn guides(&self) -> Vec<Guide> {
        self.guides.clone()
    }
    pub fn cheatsheets(&self) -> Vec<Cheatsheet> {
        self.cheatsheets.clone()
    }
}

impl From<String> for Cacher {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap_or_default()
    }
}
