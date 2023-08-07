use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    metadata: Metadata,
    pub slug: String,
    content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    title: String,
    author: String,
    icon: String,
    date: String,
    tags: String,
}

impl Article {
    /// Uses a traditional metadata block to create a new Article
    /// Can be used to examine how to serialize/deserialize structs from YAML
    /// Although this is not the most efficient way to do it as it uses a lot of string manipulation
    /// Performs close to the same as the other method according to the parsers benchmark
    pub fn new(markdown: &str) -> Self {
        // Parse the metadata block
        let meta_prefix = "---";
        let meta_delimiter = ":";

        let mut title = String::new();
        let mut slug = String::new();
        let mut author = String::new();
        let mut content = String::new();

        let mut is_metadata = false;
        let mut date = String::new();
        let mut tags = String::new();
        let mut icon = String::new();

        for line in markdown.lines() {
            if line.trim() == meta_prefix {
                is_metadata = !is_metadata;
            } else if is_metadata {
                // Process metadata
                let mut parts = line.trim().splitn(2, meta_delimiter);
                if let Some(key) = parts.next() {
                    if let Some(value) = parts.next() {
                        match key.trim() {
                            "title" => {
                                title = value.trim().to_string();
                                slug = value.trim().replace(" ", "_");
                            }
                            "author" => author = value.trim().to_string(),
                            "icon" => icon = value.trim().to_string(),
                            "date" => date = value.trim().to_string(),
                            "tags" => tags = value.trim().to_string(),
                            _ => (),
                        }
                    }
                }
            } else {
                // Process content
                content.push_str(line);
                content.push('\n');
            }
        }

        let parser = Parser::new_ext(&content, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Self {
            metadata: Metadata {
                title,
                author,
                icon,
                date,
                tags,
            },
            slug,
            content: html_output,
        }
    }
    /// Uses a YAML metadata block to create a new Article
    /// This is the most efficient way to do it as it uses serde_yaml to deserialize the YAML
    pub fn new_yaml(markdown: &str) -> Self {
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
