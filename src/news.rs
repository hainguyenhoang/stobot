use std::collections::BTreeSet;
use std::fmt::Debug;
use std::slice::Iter;
use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize)]
pub struct News {
    news: Vec<NewsItem>
}

impl News {
    pub fn filter_news_by_platform(&mut self, platforms: &BTreeSet<String>) {
        self.news.retain(|item| !platforms.is_disjoint(&item.platforms));
    }

    pub fn count(&self) -> u64 {
        self.news.len() as u64
    }

    pub fn iter(&self) -> Iter<NewsItem> {
        return self.news.iter();
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct NewsItem {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u64,
    title: String,
    summary: String,
    platforms: BTreeSet<String>,
    //updated: String
}

impl NewsItem {
    pub fn get_id(&self) -> u64 {
        self.id
    }

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
