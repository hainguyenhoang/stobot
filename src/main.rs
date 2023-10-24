mod news;
mod handler;

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

    /// Number of news to poll in each period and to save for the check during the next poll period
    #[arg(long, default_value_t = 10)]
    poll_count: u64,

    /// How many new news to check compared to all of the previous poll's news, must not be larger than --poll-count
    #[arg(long, default_value_t = 5)]
    check_count: u64,

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
    println!("Check count: {}", args.check_count);
    if args.check_count > args.poll_count {
        panic!("--check-count is larger than --poll-count!");
    }
    let handler = Handler::new(args.poll_period, args.poll_count, args.check_count, args.channels_path, args.platforms);
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client =
        Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why}");
    }
}
