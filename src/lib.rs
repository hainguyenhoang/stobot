use chrono::prelude::*;
use serde_json::Value;
use serde_json::Value::Number;
use serde_json::Value::Null;

pub mod handler;

async fn get_news_json(count: u64) -> Value {
    let request = reqwest::get(format!("https://api.arcgames.com/v1.0/games/sto/news?limit={count}")).await;
    if let Err(why) = request {
        err_log(format!("Error with the request: {why}").as_str());
        return Null;
    }
    let text = request.unwrap().text().await;
    if let Err(why) = text {
        err_log(format!("Error with the text: {why}").as_str());
        return Null;
    }
    let parsed_json: serde_json::Result<Value> = serde_json::from_str(text.unwrap().as_str());
    match parsed_json {
        Ok(json) => json,
        Err(why) => {
            err_log(format!("Error while parsing the json: {why}").as_str());
            Null
        }
    }
}

async fn get_news_ids(count: u64) -> Vec<u64>{
    let mut ids: Vec<u64> = vec![];
    let json = get_news_json(count).await;
    if let Null = json {
        return ids;
    }
    for i in 0..count{
        let i = usize::try_from(i).expect(format!("Couldn't convert {i} from u64 to usize").as_str());
        if let Number(id) = &json["news"][i]["id"]{
            if let Some(as_64) = id.as_u64(){
                ids.push(as_64);
            }
        }
    }

    ids
}

pub fn out_log(msg: &str){
    println!("{}:\t{}", Utc::now(), msg);
}

pub fn err_log(msg: &str){
    eprintln!("{}:\t{}", Utc::now(), msg);
}