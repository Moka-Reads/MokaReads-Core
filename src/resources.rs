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
