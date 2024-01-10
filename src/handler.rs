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
use regex::Regex;

use crate::news::News;

pub struct Handler {
    poll_period: u64,
    poll_count: u64,
    channel_ids: Mutex<HashSet<u64>>,
    channel_txt_path: String,
    fresh_seconds: u64,
    msg_count: u64,
    platforms: BTreeSet<String>,
}

impl Handler {
    pub fn new(poll_period: u64, poll_count: u64, channel_txt_path: String, fresh_seconds: u64, msg_count: u64, platforms: BTreeSet<String>) -> Handler {
        let handler = Handler {
            poll_period,
            poll_count,
            channel_ids: Mutex::new(HashSet::new()),
            channel_txt_path,
            fresh_seconds,
            msg_count,
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
        let url = format!("https://api.arcgames.com/v1.0/games/sto/news?limit={count}&field[]=platforms&field[]=updated");
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

    fn get_ids_from_messages(messages: &Vec<Message>) -> Vec<u64> {
        let mut result: Vec<u64> = vec![];
        let re = Regex::new(r"\d+>\n*").unwrap();
        let re2 = Regex::new(r"\d+").unwrap();
        for m in messages{
            if let Some(capture) = re.captures(m.content.as_str()) {
                let new_capture = re2.captures(&capture[0]).unwrap();
                result.push(new_capture[0].parse::<u64>().unwrap());
            }
        }
        result
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
        loop {
            if let Some(mut news) = Self::get_news_from_json(self.poll_count).await {
                news.filter_news_by_platform(&self.platforms);
                for channel_id in self.get_channels().iter() {
                    let channel = ChannelId(*channel_id);
                    match channel.messages(&ctx.http, |b| b.limit(self.msg_count)).await {
                        Ok(existing_messages) => {
                            let existing_ids = Self::get_ids_from_messages(&existing_messages);
                            for item in news.iter() {
                                if !existing_ids.contains(&item.get_id()) && item.is_fresh(self.fresh_seconds){
                                    println!("Sending news with ID {} to channel with ID {}", item.get_id(), *channel_id);
                                    if let Err(why) = channel.say(&ctx.http, item.get_msg_str().as_str()).await {
                                        eprintln!("Error sending message: {why}");
                                    }
                                }
                            }
                        }
                        Err(why) => eprintln!("Error reading existing messages: {why}")
                    }
                }
            }
            task::sleep(Duration::from_secs(self.poll_period)).await;
        }
    }
}
