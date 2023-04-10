use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Mutex, MutexGuard};
use std::thread::sleep;
use std::time::Duration;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

use crate::get_news_ids;

pub struct Handler {
    poll_period: u64,
    poll_count: u64,
    latest_news: Mutex<u64>,
    channel_ids: Mutex<HashSet<u64>>
}

impl Handler{
    pub fn new(poll_period: u64, poll_count: u64) -> Handler {
        let handler = Handler {
            poll_period,
            poll_count,
            latest_news: Mutex::new(0),
            channel_ids: Mutex::new(HashSet::new())
        };
        println!("Reading channels.txt");
        if let Ok(file) = File::open("channels.txt"){
            for line in BufReader::new(file).lines(){
                if let Ok(parsed_line) = line{
                    if let Ok(parsed_id) = parsed_line.parse::<u64>(){
                        handler.add_channel(parsed_id);
                    }
                }
            }
        }
        print!("Channels:");
        let channels = handler.get_channels();
        for channel in channels.iter(){
            print!(" {channel}");
        }
        println!();
        handler
    }

    fn get_latest_id(&self) -> u64 {
        *self.latest_news.lock().unwrap()
    }

    fn set_latest_id(&self, value: u64){
        let mut num = self.latest_news.lock().unwrap();
        *num = value;
        println!("ID of latest news: {value}");
    }

    pub fn get_channels(&self) -> HashSet<u64> {
        self.channel_ids.lock().unwrap().clone()
    }

    fn write_channels_to_file(map: MutexGuard<HashSet<u64>>){
        let mut file = File::create("channels.txt").expect("Couldn't open channels.txt");
        for id in map.iter(){
            writeln!(file, "{id}").expect("Couldn't write to channels.txt");
        }
    }

    pub fn add_channel(&self, id: u64){
        let mut map = self.channel_ids.lock().unwrap();
        map.insert(id);
        Handler::write_channels_to_file(map);
    }

    fn remove_channel(&self, id: u64){
        let mut map = self.channel_ids.lock().unwrap();
        map.remove(&id);
        Handler::write_channels_to_file(map);
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
                for channel in registered_channels.iter(){
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
                for channel in registered_channels.iter(){
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
        let news_ids = get_news_ids(1);
        println!("It's alive!");
        match news_ids.await.get(0) {
            Some(id) => self.set_latest_id(*id),
            None => eprintln!("Couldn't get the ID of the latest news")
        }
        loop {
            sleep(Duration::from_secs(self.poll_period));
            let news_ids = get_news_ids(self.poll_count).await;
            let new_news_count = match news_ids.iter().position(|&i| i == self.get_latest_id()){
                Some(pos_of_last_news) => {
                    if pos_of_last_news > 0 { self.set_latest_id(news_ids[0]) }
                    pos_of_last_news
                },
                None => {
                    self.set_latest_id(news_ids[0]);
                    self.poll_count as usize
                }
            };
            if new_news_count > 0 {
                println!("Found {new_news_count} new news");
            }
            let news_ids = &news_ids[..new_news_count];
            for news_id in news_ids.iter().rev(){
                let url = format!("https://playstartrekonline.com/en/news/article/{news_id}");
                for channel_id in self.get_channels().iter(){
                    let channel_id = *channel_id;
                    let channel = ChannelId(channel_id);
                    println!("Sending news with ID {news_id} to channel with ID {channel_id}");
                    if let Err(why) = channel.say(&ctx.http, &url).await {
                        eprintln!("Error sending message: {why}");
                    }
                }
            }
        }
    }
}
