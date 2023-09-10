use super::Parser as ArticleParser;
use chrono::Utc;
use pulldown_cmark::{html, Options, Parser};
use rss::Item;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Article {
    metadata: Metadata,
    pub slug: String,
    content: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    title: String,
    description: String,
    author: String,
    icon: String,
    date: String,
    tags: String,
}

impl Metadata {
    pub fn new(title: &str, description: &str, author: &str, icon: &str, tags: &str) -> Self {
        let date = Utc::now().date_naive().format("YYYY-MM-DD").to_string();
        Self {
            title: title.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            icon: icon.to_string(),
            date,
            tags: tags.to_string(),
        }
    }
}

impl ArticleParser for Article {
    fn parse(markdown: &str) -> Self
    where
        Self: Sized,
    {
        let separator = "---";
        let mut sections = markdown.splitn(3, separator);
        sections.next();
        let yaml_section = sections.next().unwrap_or("");
        let content_section = sections.next().unwrap_or("");

        let metadata = from_str::<Metadata>(yaml_section).unwrap();
        let parser = Parser::new_ext(content_section, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        let slug = metadata.title.replace(' ', "_");
        Self {
            metadata,
            slug,
            content: html_output,
        }
    }
}

impl Article {
    fn link(&self) -> String {
        format!("https://moka-reads.mkproj.com/articles/{}", self.slug)
    }
    pub fn to_rss_item(&self) -> Item {
        let mut item = Item::default();
        item.set_title(self.metadata.title.to_string());
        item.set_description(self.metadata.description.to_string());
        item.set_pub_date(self.metadata.date.to_string());
        item.set_link(self.link());
        item.set_guid(
            rss::GuidBuilder::default()
                .value(self.link())
                .permalink(true)
                .build(),
        );
        item.set_pub_date(self.metadata.date.clone());
        item
    }
    pub fn new(metadata: Metadata, content: String) -> Self {
        let slug = metadata.title.replace(' ', "_");
        Self {
            metadata,
            slug,
            content,
        }
    }
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("---\n");
        markdown.push_str(&serde_yaml::to_string(&self.metadata).unwrap());
        markdown.push_str("---\n");
        markdown.push_str(&self.content);
        markdown
    }
}

pub fn articles_rss(articles: Vec<Article>) -> rss::Channel {
    let mut channel = rss::Channel::default();
    let now = Utc::now().to_rfc2822();
    channel.set_last_build_date(now);
    channel.set_title("Moka Reads".to_string());
    channel.set_link("https://moka-reads.mkproj.com".to_string());
    channel.set_description("An Opensource Education Platform".to_string());
    channel.set_language("en".to_string());
    channel.set_ttl("60".to_string());
    channel.set_items(
        articles
            .iter()
            .map(|article| article.to_rss_item())
            .collect::<Vec<Item>>(),
    );
    channel
}
