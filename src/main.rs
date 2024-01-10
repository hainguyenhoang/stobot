mod news;
mod handler;

use std::collections::BTreeSet;
use std::env;
use clap::Parser;
use serenity::prelude::*;
use crate::handler::Handler;

#[derive(Parser)]
struct Args {
    /// Path to saved channels
    #[arg(short, long, default_value = "channels.txt")]
    channels_path: String,

    /// Time in seconds inbetween checking for news
    #[arg(long, default_value_t = 10)]
    poll_period: u64,

    /// Number of news to poll in each period
    #[arg(long, default_value_t = 20)]
    poll_count: u64,

    /// Maximum time difference in seconds between now and timestamp of news item to be even considered for posting
    #[arg(short, long, default_value_t = 120)]
    fresh_seconds: u64,

    /// Filter news from these platforms
    #[arg()]
    platforms: Vec<String>
}

pub fn parse_u64_arg() -> Option<u64> {
    if let Some(next_arg) = env::args().next() {
        if let Ok(parsed_arg) = next_arg.parse::<u64>() {
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
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let args = Args::parse();
    println!("Saved channels path: {}", args.channels_path);
    println!("Polling period: {}", args.poll_period);
    println!("Poll count: {}", args.poll_count);
    let platforms = BTreeSet::from_iter(args.platforms);
    for platform in &platforms {
        println!("Platform: {}", platform);
    }
    let handler = Handler::new(args.poll_period, args.poll_count, args.channels_path, platforms);
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client =
        Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why}");
    }
}
