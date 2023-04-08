use std::env;
use std::fs::File;

use serenity::prelude::*;
use stobot::handler::Handler;
use stobot::out_log;
use stobot::err_log;

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
    out_log("Reading channels.txt");
    if let Ok(file) =  File::open("channels.txt"){

    }
    let handler = Handler::build();
    let mut client =
        Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        err_log(format!("Client error: {why}").as_str());
    }
}