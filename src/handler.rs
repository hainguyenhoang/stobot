use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

use crate::out_log;
use crate::err_log;
use crate::get_news_ids;

pub struct Handler {
    latest_news: Mutex<u64>,
    channel_ids: Mutex<HashSet<u64>>
}

impl Handler{
    pub fn build() -> Handler {
        Handler {
            latest_news: Mutex::new(0),
            channel_ids: Mutex::new(HashSet::new())
        }
    }

    fn get_latest_id(&self) -> u64 {
        *self.latest_news.lock().unwrap()
    }

    fn set_latest_id(&self, value: u64){
        let mut num = self.latest_news.lock().unwrap();
        *num = value;
    }

    fn add_channel(&self, id: u64){
        let mut map = self.channel_ids.lock().unwrap();
        map.insert(id);
        for id in map.iter(){
            let mut file = File::create("channels.txt")
                .expect("Couldn't open channels.txt");
            writeln!(file, "{id}").expect("Couldn't write to channels.txt");
        }
    }

    fn get_channels(&self) -> HashSet<u64> {
        self.channel_ids.lock().unwrap().clone()
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!stobot" {
            let id = msg.channel_id.0;
            self.add_channel(id);
            out_log(format!("Registered channel with ID {id}").as_str());
            let mut out_str = String::from("Registered channels:");
            let registered_channels = self.get_channels();
            for channel in registered_channels.iter(){
                out_str += format!(" {channel}").as_str();
            }
            out_log(&out_str);
            let response = "This channel will now have STO news posted.";
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                err_log(format!("Error sending message: {why}").as_str());
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        let news_ids = get_news_ids(1);
        out_log("It's alive!");
        match news_ids.await.get(0) {
            Some(id) => {
                self.set_latest_id(*id);
                out_log(format!("ID of latest news: {}", self.get_latest_id()).as_str());
            }
            None => err_log("Couldn't get the ID of the latest news")
        }
        loop {
            sleep(Duration::from_secs(60));
            let news_ids = get_news_ids(10);
            out_log("Checking the news...");
            let news_ids = news_ids.await;
            let new_news_count = match news_ids.iter().position(|&i| i == self.get_latest_id()){
                Some(i) => i,
                None => 10
            };
            let news_ids = &news_ids[..new_news_count];
            for news_id in news_ids.iter().rev(){
                let url = format!("https://playstartrekonline.com/en/news/article/{news_id}");
                for channel_id in self.get_channels().iter(){
                    let channel_id = *channel_id;
                    let channel = ChannelId(channel_id);
                    out_log(format!("Sending news of ID {news_id} to {channel_id}").as_str());
                    if let Err(why) = channel.say(&ctx.http, &url).await {
                        err_log(format!("Error sending message: {why}").as_str());
                    }
                }
            }
        }
    }
}

