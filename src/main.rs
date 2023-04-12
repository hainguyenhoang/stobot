mod news;
mod handler;

use std::env;
use serenity::prelude::*;
use crate::handler::Handler;

pub fn parse_arg() -> Option<u64> {
    if let Some(next_arg) = env::args().next(){
        if let Ok(parsed_arg) = next_arg.parse::<u64>(){
            return Some(parsed_arg);
        }
    }
    None
}

#[tokio::main]
async fn main() {
    let name = env!("CARGO_PKG_NAME", "");
    let version = env!("CARGO_PKG_VERSION", "");
    println!("{name} v{version}");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    env::args().next();
    let mut poll_period = 10;
    let mut poll_count = 5;
    if let Some(p) = parse_arg(){
        poll_period = p;
    }
    if let Some(p) = parse_arg(){
        poll_count = p;
    }
    println!("Polling period: {poll_period}");
    println!("Poll count: {poll_count}");
    let handler = Handler::new(poll_period, poll_count);
    let mut client =
        Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why}");
    }
}
