use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use serenity::prelude::*;
use stobot::handler::Handler;
use stobot::out_log;

#[tokio::main]
async fn main() {
    for _ in 1..=64{
        print!("-");
        eprint!("-");
    }
    println!();
    eprintln!();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    env::args().next();
    let mut poll_period = 10;
    let mut poll_count = 5;
    if let Some(p) = stobot::parse_arg(){
        poll_period = p;
    }
    if let Some(p) = stobot::parse_arg(){
        poll_count = p;
    }
    out_log(format!("Polling period: {poll_period}").as_str());
    out_log(format!("Poll count: {poll_count}").as_str());
    let handler = Handler::new(poll_period, poll_count);
    stobot::out_log("Reading channels.txt");
    if let Ok(file) = File::open("channels.txt"){
        for line in BufReader::new(file).lines(){
            if let Ok(parsed_line) = line{
                if let Ok(parsed_id) = parsed_line.parse::<u64>(){
                    handler.add_channel(parsed_id);
                }
            }
        }
    }
    stobot::out_log("Channels:");
    let channels = handler.get_channels();
    for channel in channels.iter(){
        print!("{channel}");
    }
    println!();
    let mut client =
        Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        stobot::err_log(format!("Client error: {why}").as_str());
    }
}
