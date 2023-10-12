#[cfg(test)]
mod tests {
    use mokareads_core::resources::article::{Article, Metadata};
    use mokareads_core::resources::cheatsheet::Language;

    // Define test data for an article
    fn create_test_article() -> Article {
        let metadata = Metadata::new(
            "Test Article",
            "Test description",
            "Test Author",
            "devicon",
            "tag1, tag2",
        );
        let content = "Test content".to_string();
        Article::new(metadata, content)
    }

    #[test]
    fn test_to_rss_item() {
        let article = create_test_article();
        let rss_item = article.to_rss_item();

        // Ensure that the RSS item's title, description, and link match the expected values
        assert_eq!(rss_item.title(), Some("Test Article"));
        assert_eq!(rss_item.description(), Some("Test description"));
        assert_eq!(
            rss_item.link(),
            Some("https://moka-reads.mkproj.com/articles/Test_Article")
        );
    }

    #[test]
    fn test_lang_in_tag() {
        let article = create_test_article();
        let rust_in_tags = article.lang_in_tag(Language::Rust);

        // Ensure that the 'rust_in_tags' variable is false since Rust is not in the tags
        assert_eq!(rust_in_tags, false);
    }
}
