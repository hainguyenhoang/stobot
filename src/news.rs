use std::collections::{HashSet, VecDeque};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct News {
    news: VecDeque<NewsItem>
}

#[derive(Deserialize)]
struct NewsItem {
    id: u64,
    title: String,
    summary: String,
    images: Images,
    platforms: HashSet<String>
}

#[derive(Deserialize)]
struct Images {
    img_microsite_background: Image
}

#[derive(Deserialize)]
struct Image {
    url: String
}