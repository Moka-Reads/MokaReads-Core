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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Cacher {
    updated_at: String,
    articles: Vec<article::Article>,
    cheatsheets: Vec<cheatsheet::Cheatsheet>,
    guides: Vec<guide::Guide>,
}
