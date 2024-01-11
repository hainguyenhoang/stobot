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

    /// Amount of Discord messages to check for already posted news items during each poll. Discord has a limitation of 100.
    #[arg(short, long, default_value_t = 50)]
    msg_count: u8,

    /// Space separated list of platforms to filter news from. E.g.: to have news from all 3: `pc ps xbox`
    #[arg(required = true, num_args = 1..)]
    platforms: Vec<String>
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
    println!("Fresh seconds: {}", args.fresh_seconds);
    println!("Messages to check: {}", args.msg_count);
    let platforms = BTreeSet::from_iter(args.platforms);
    for platform in &platforms {
        println!("Platform: {}", platform);
    }
    let handler = Handler::new(args.poll_period, args.poll_count, args.channels_path, args.fresh_seconds, args.msg_count, platforms);
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable is unset!");
    let mut client =
        Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why}");
    }
}
