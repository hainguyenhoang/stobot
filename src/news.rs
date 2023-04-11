use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct News {
    news: Vec<NewsItem>
}

impl News {
    pub fn new() -> Self {
        News {
            news: Vec::new()
        }
    }

    async fn get_news_json(count: u64) -> Result<String, reqwest::Error> {
        let url = format!("https://api.arcgames.com/v1.0/games/sto/news?limit={count}&field[]=images.img_microsite_background&field[]=platforms");
        let response = reqwest::get(url).await?;
        Ok(response.text().await?)
    }

    pub async fn get_news_from_json(count: u64) -> Option<Self> {
        match Self::get_news_json(count).await {
            Ok(text) => {
                match serde_json::from_str::<Self>(text.as_str()) {
                    Ok(item) => Some(item),
                    Err(why) => {
                        eprintln!("Error while getting the news json: {why}");
                        None
                    }
                }
            }
            Err(why) => {
                eprintln!("Error while getting the news json: {why}");
                None
            }
        }
    }

    pub fn get_different_items(&self, old_news: &Self) -> Vec<&NewsItem> {
        let mut result = vec![];
        for item in &self.news {
            if !old_news.news.contains(item){
                result.push(item);
            }
        }
        result
    }
}

#[derive(Deserialize)]
pub struct NewsItem {
    pub id: u64,
    title: String,
    summary: String,
    platforms: Vec<String>
}

impl Display for NewsItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let url = format!("https://playstartrekonline.com/en/news/article/{}", self.id);
        let mut platform_str = String::new();
        for platform in &self.platforms {
            platform_str += format!(":{platform}: ").as_str();
        }
        write!(f, "**{}**\n{}\n<{}>\n{}", self.title, self.summary, url, platform_str)
    }
}

impl PartialEq for NewsItem{
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
    }
}
