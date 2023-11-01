use std::collections::BTreeSet;
use std::fmt::Debug;
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

    pub fn count(&self) -> u64 {
        self.news.len() as u64
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct NewsItem {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    title: String,
    summary: String,
    platforms: BTreeSet<String>
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

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn news_item_from_id(id: u64) -> NewsItem {
        NewsItem {
            id,
            title: "".to_string(),
            summary: "".to_string(),
            platforms: vec![],
        }
    }
    
    #[test]
    fn test_different_items(){
        let mut a = News::new();
        a.news.push(news_item_from_id(4));
        a.news.push(news_item_from_id(3));
        a.news.push(news_item_from_id(2));
        a.news.push(news_item_from_id(1));
        a.news.push(news_item_from_id(0));

        let mut b = News::new();
        b.news.push(news_item_from_id(5));
        b.news.push(news_item_from_id(3));
        b.news.push(news_item_from_id(2));
        b.news.push(news_item_from_id(1));
        b.news.push(news_item_from_id(0));

        let diff = b.get_different_items(&a, 3);
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].id, 5)
    }

    #[test]
    fn test_newsitem_eq() {
        let a = news_item_from_id(1337);
        let b = news_item_from_id(1337);
        let c = news_item_from_id(69420);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }

    #[test]
    fn test_msg_str() {
        let item = NewsItem {
            id: 1337,
            title: "title".to_string(),
            summary: "summary".to_string(),
            platforms: vec!["a".to_string(), "ps".to_string(), "xbox".to_string()],
        };
        let expected = fs::read_to_string("test_data/msg_str.txt").unwrap();
        assert_eq!(item.get_msg_str(), expected);
    }
}