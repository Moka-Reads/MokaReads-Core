use crate::Result;
use serde::{Deserialize, Serialize};
use reqwest::{header, Client};
use reqwest::header::HeaderMap;

/// Represents a GitHub Repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    name: String,
    url: String,
    description: Option<String>,
}

impl Repository {
    /// Creates a new repository given a name, url and description
    pub fn new(name: String, url: String, description: Option<String>) -> Self {
        Self {
            name,
            url,
            description,
        }
    }
    pub async fn get_awesome_lists(page: usize) -> Result<Vec<Self>>{
        // Set User-Agent header
        let user_agent = header::HeaderValue::from_static("MoKaReads-Awesome-Lists");
        let client = Client::builder()
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(header::USER_AGENT, user_agent);
                headers
            }).build()?;
        let mut awesome_lists = Vec::new();

            let url = format!("https://api.github.com/search/repositories?q=topic:awesome&page={page}");
            let response = client
                .get(&url)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            // Process the repositories
            let repositories = response["items"]
                .as_array()
                .ok_or("Invalid response format")?;

            for repo in repositories {
                let name = repo["name"]
                    .as_str()
                    .ok_or("Invalid repository name")?
                    .to_string();
                let url = repo["html_url"]
                    .as_str()
                    .ok_or("Invalid repository URL")?
                    .to_string();
                let description = repo["description"].as_str().map(|s| s.to_string());
                awesome_lists.push(Repository::new(name, url, description));
            }

        Ok(awesome_lists)
    }
}

/// A wrapper over a list of list of repos :)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AwesomeList{
    page_list: Vec<Vec<Repository>>
}

impl Default for AwesomeList{
    fn default() -> Self {
        Self{page_list: Vec::new()}
    }
}

impl AwesomeList{
    /// Get a list of repos for each page which we would want
    pub async fn new(pages: usize) -> Result<Self>{
        let mut page_list = Vec::new();
        for i in 1..=pages{
            let list = Repository::get_awesome_lists(i).await?;
            page_list.push(list);
        }

        Ok(Self{page_list})
    }
    /// Get a specific page by finding the index - 1
    pub fn get_page(&self, page: usize) -> Vec<Repository>{
        self.page_list[page - 1].clone()
    }

    /// Returns the number of repos
    pub fn count_repos(&self) -> usize{
        self.page_list.iter().flatten().count()
    }
}