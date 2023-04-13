use serde::Deserialize;
use serde_aux::prelude::*;

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
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    title: String,
    summary: String,
    platforms: Vec<String>
}

impl NewsItem {
    pub fn get_msg_str(&self) -> String {
        let new_url = format!("https://playstartrekonline.com/en/news/article/{}", self.id);
        let mut result = format!("**{}**\n{}\n<{}>\n", self.title, self.summary, new_url);
        for platform in &self.platforms {
            result += "<https://www.arcgames.com/en/games";
            match platform.as_str() {
                "ps" => result += "/playstation",
                "xbox" => result += "/xbox",
                _ => {}
            }
            result = format!("{}/star-trek-online/news/detail/{}>\n", result, self.id);
        }
        result
    }
}

impl PartialEq for NewsItem{
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
    }
}
