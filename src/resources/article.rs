use chrono::Utc;
use pulldown_cmark::{html, Options, Parser};
use rss::Item;
use serde::{Deserialize, Serialize};

use crate::resources::cheatsheet::Language;
use crate::resources::{ResourceType, SearchMetadata};

use super::Parser as ArticleParser;

/// # Moka Reads Articles   
/// Articles are built to cover different topics related to a language,
/// this could be news, tutorials, or anything else that is relevant to the language.
/// To properly organize the different articles in our repository, we use a specification
/// which will regularly update to keep up with the needs of the community.
///
/// ## Markdown Format:
///
/// ```markdown
/// ---
/// title: My Article
/// description: This is my article
/// author: John Doe
/// date: 2020-01-01
/// tags:
///    - tag1
///    - tag2
/// icon: devicon
/// ---
///
/// Content of the article
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Article {
    metadata: Metadata,
    pub slug: String,
    content: String,
}

/// # Article Metadata
/// The metadata section is a YAML document which contains the following fields:
/// - `title`: The title of the article.
/// - `description`: A short description of the article.
/// - `author`: The author of the article.
/// - `date`: The date the article was published (YYYY-MM-DD).
/// - `tags`: A list of tags for the article.
/// - `icon`: The icon to use for the article (`devicon` or `fontawesome5`).
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
        let date = Utc::now().naive_utc().format("%Y-%m-%d").to_string();
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
    fn parse_raw(markdown: &str) -> Self
    where
        Self: Sized,
    {
        let separator = "---";
        let mut sections = markdown.splitn(3, separator);
        sections.next();
        let yaml_section = sections.next().unwrap_or("");
        let content_section = sections.next().unwrap_or("");

        let metadata = serde_yaml::from_str::<Metadata>(yaml_section).unwrap();
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
        let mut article = Self::parse_raw(markdown);
        let parser = Parser::new_ext(article.content.as_str(), Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        article.content = html_output;
        article
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

impl Article {
    fn link(&self) -> String {
        format!("https://moka-reads.mkproj.com/articles/{}", self.slug)
    }
    pub fn link_short(&self) -> String {
        format!("/articles/{}", self.slug)
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
    pub fn title(&self) -> String {
        self.metadata.title.to_string()
    }
    pub fn as_search_meta(&self) -> SearchMetadata {
        SearchMetadata::new(
            self.title(),
            ResourceType::Article,
            self.link_short(),
            self.metadata.tags.clone(),
        )
    }
    pub fn lang_in_tag(&self, lang: Language) -> bool {
        let lang = lang.to_string();
        self.metadata.tags.contains(&lang)
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
