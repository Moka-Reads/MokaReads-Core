use crate::Result;

#[derive(Debug, Clone, Copy)]
pub enum Api{
    Resources,
    Cheatsheets,
    Articles,
    Guides,
    Awesome,
    LangMap
}

impl Api{
    fn link(&self) -> String{
        let base = "https://mokareads.org/api/";
        match self{
            Api::Resources => format!("{}resources", base),
            Api::Cheatsheets => format!("{}cheatsheets", base),
            Api::Articles => format!("{}articles", base),
            Api::Guides => format!("{}guides", base),
            Api::Awesome => format!("{}awesome", base),
            Api::LangMap => format!("{}lang_map", base),
        }
    }
    pub async fn get(&self) -> Result<String>{
        let link = self.link();
        let response = reqwest::get(&link).await?.text().await?;
        Ok(response)
    }
}