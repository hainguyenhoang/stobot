use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;
use async_std::task;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

use crate::news::News;

pub struct Handler {
    poll_period: u64,
    poll_count: u64,
    check_count: u64,
    channel_ids: Mutex<HashSet<u64>>,
    channel_txt_path: String,
    platforms: BTreeSet<String>
}

impl Handler {
    pub fn new(poll_period: u64, poll_count: u64, check_count:u64, channel_txt_path: String, platforms: BTreeSet<String>) -> Handler {
        let handler = Handler {
            poll_period,
            poll_count,
            check_count,
            channel_ids: Mutex::new(HashSet::new()),
            channel_txt_path,
            platforms
        };
        println!("Reading {}", handler.channel_txt_path);
        if let Ok(file) = File::open(&handler.channel_txt_path) {
            for line in BufReader::new(file).lines() {
                if let Ok(parsed_line) = line {
                    if let Ok(parsed_id) = parsed_line.parse::<u64>() {
                        handler.add_channel(parsed_id);
                    }
                }
            }
        }
        print!("Channels:");
        let channels = handler.get_channels();
        for channel in channels.iter() {
            print!(" {channel}");
        }
        println!();
        handler
    }

    pub fn get_channels(&self) -> HashSet<u64> {
        self.channel_ids.lock().unwrap().clone()
    }

    fn write_channels_to_file(&self, map: MutexGuard<HashSet<u64>>) {
        let mut file = File::create(&self.channel_txt_path).expect(format!(
            "Couldn't open {}", self.channel_txt_path).as_str());
        for id in map.iter() {
            writeln!(file, "{id}").expect(format!(
                "Couldn't write to {}", self.channel_txt_path).as_str());
        }
    }

    pub fn add_channel(&self, id: u64) {
        let mut map = self.channel_ids.lock().unwrap();
        map.insert(id);
        self.write_channels_to_file(map);
    }

    fn remove_channel(&self, id: u64) {
        let mut map = self.channel_ids.lock().unwrap();
        map.remove(&id);
        self.write_channels_to_file(map);
    }

    async fn get_news_json(count: u64) -> Result<String, reqwest::Error> {
        let url = format!("https://api.arcgames.com/v1.0/games/sto/news?limit={count}&field[]=platforms");
        let response = reqwest::get(url).await?;
        Ok(response.text().await?)
    }

    async fn get_news_from_json(count: u64) -> Option<News> {
        match Self::get_news_json(count).await {
            Ok(text) => {
                match serde_json::from_str::<News>(text.as_str()) {
                    Ok(item) => {
                        if item.count() == count {
                            Some(item)
                        } else {
                            eprintln!("Expected {} news, got {} instead", count, item.count());
                            None
                        }
                    }
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
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let id = msg.channel_id.0;
        match msg.content.as_str() {
            "!stobot" => {
                self.add_channel(id);
                println!("Registered channel with ID {id}");
                let mut out_str = String::from("Registered channels:");
                let registered_channels = self.get_channels();
                for channel in registered_channels.iter() {
                    out_str += format!(" {channel}").as_str();
                }
                println!("{out_str}");
                let response = "This channel will now have STO news posted.";
                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    eprintln!("Error sending message: {why}");
                }
            }
            "!unstobot" => {
                self.remove_channel(id);
                println!("Removed channel with ID {id}");
                let mut out_str = String::from("Registered channels:");
                let registered_channels = self.get_channels();
                for channel in registered_channels.iter() {
                    out_str += format!(" {channel}").as_str();
                }
                println!("{out_str}");
                let response = "This channel will no longer have STO news posted.";
                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    eprintln!("Error sending message: {why}");
                }
            }
            _ => {}
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        let mut old_news = News::new();
        if let Some(news) = Self::get_news_from_json(self.poll_count).await {
            old_news = news;
        }
        loop {
            task::sleep(Duration::from_secs(self.poll_period)).await;
            if let Some(news) = Self::get_news_from_json(self.poll_count).await {
                let diff = news.get_different_items(&old_news, self.check_count, &self.platforms);
                for item in diff {
                    for channel_id in self.get_channels().iter() {
                        let channel_id = *channel_id;
                        let channel = ChannelId(channel_id);
                        println!("Sending news with ID {} to channel with ID {}", item.id, channel_id);
                        if let Err(why) = channel.say(&ctx.http, item.get_msg_str().as_str()).await {
                            eprintln!("Error sending message: {why}");
                        }
                    }
                }
                old_news = news;
            }
        }
    }
}
