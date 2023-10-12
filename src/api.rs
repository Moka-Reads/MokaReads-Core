use crate::Result;

/// Different API Handles to send requests to
#[derive(Debug, Clone, Copy)]
pub enum Api {
    /// Requests for the complete resources index
    Resources,
    /// Requests for the cheathseet index
    Cheatsheets,
    /// Requests for the articles index
    Articles,
    /// Represents for the guides index
    Guides,
    /// Represents the awesome lists index
    Awesome,
    /// Requests for the cheatsheet index as `HashMap<Language, Vec<Cheatsheet>>`
    LangMap,
}

/// The MoKa Reads branch either (stable or beta)
#[derive(Debug, Clone, Copy)]
pub enum Branch {
    /// Represents the `main` branch in `git`
    Stable,
    /// Represents the `beta` branch in `git`
    Beta,
}

impl Api {
    fn link(&self, branch: Branch) -> String {
        let base = match branch {
            Branch::Stable => "https://mokareads.org/api/",
            Branch::Beta => "https://beta.mokareads.org/api/",
        };
        match self {
            Api::Resources => format!("{}resources", base),
            Api::Cheatsheets => format!("{}cheatsheets", base),
            Api::Articles => format!("{}articles", base),
            Api::Guides => format!("{}guides", base),
            Api::Awesome => format!("{}awesome", base),
            Api::LangMap => format!("{}lang_map", base),
        }
    }
    pub async fn get(&self, branch: Branch) -> Result<String> {
        let link = self.link(branch);
        let response = reqwest::get(&link).await?.text().await?;
        Ok(response)
    }
}
