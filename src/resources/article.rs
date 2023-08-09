use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use super::Parser as ArticleParser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    metadata: Metadata,
    pub slug: String,
    content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    title: String,
    description: String,
    author: String,
    icon: String,
    date: String,
    tags: String,
}

impl ArticleParser for Article{
    fn parse(markdown: &str) -> Self where Self: Sized {
        let separator = "---";
        let mut sections = markdown.splitn(3, separator);
        sections.next();
        let yaml_section = sections.next().unwrap_or("");
        let content_section = sections.next().unwrap_or("");

        let metadata = from_str::<Metadata>(yaml_section).unwrap();
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

