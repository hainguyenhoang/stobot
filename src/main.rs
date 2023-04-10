use std::env;

use serenity::prelude::*;

use stobot::handler::Handler;

#[tokio::main]
async fn main() {
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
    println!("Polling period: {poll_period}");
    println!("Poll count: {poll_count}");
    let handler = Handler::new(poll_period, poll_count);
    let mut client =
        Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why}");
    }
}
