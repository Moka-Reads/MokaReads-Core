/// MoKa Reads Article
pub mod article;
/// MoKa Reads Cheatsheets
pub mod cheatsheet;
/// MoKa Reads How to Guides
pub mod guide;

/// Parses MarkDown to a type
pub trait Parser {
    fn parse(markdown: &str) -> Self
    where
        Self: Sized;
}

use article::Article;
use cheatsheet::Cheatsheet;
use guide::Guide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Cacher {
    updated_at: String,
    articles: Vec<Article>,
    cheatsheets: Vec<Cheatsheet>,
    guides: Vec<Guide>,
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
