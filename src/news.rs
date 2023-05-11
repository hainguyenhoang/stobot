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

    pub fn get_different_items(&self, old_news: &Self, check_count: u64) -> Vec<&NewsItem> {
        let mut result = vec![];
        for item in &self.news[..check_count as usize] {
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
