use std::error::Error;
use reqwest::Client;
use tokio;
// use serde_json;
use serde::{Deserialize};
extern crate colored;


use colored::*;



#[derive(Debug, Deserialize)]
struct Item {
    pub gross: String,
    pub id: String,
    pub image: String,
    pub rank: String,
    pub title: String,
    pub weekend: String,
    pub weeks: String
}

#[derive(Debug, Deserialize)]
struct Response {
    pub errorMessage: String,
    pub items: Vec<Item>
}


async fn send_request(url: &str){
    let client = Client::new();
    let res = client.get(url);
    let res = res.send().await.unwrap().bytes().await.unwrap().to_vec();
    let res = String::from_utf8_lossy(&res.as_slice());
    let res: Response = serde_json::from_str(&res).unwrap();
    let items = res.items;
    for i in items.into_iter() {
        println!("{}", "*".repeat(80));
        println!("Gross: {0}\nId: {1}\nImage: {2}\nRank: {3}\nTitle: {4}\nWeekend: {5}\nWeeks: {6}", i.gross.red(), i.id.blue(), i.image.yellow(), i.rank.purple(), i.title.green().bold(), i.weekend.cyan(), i.weeks.magenta());
        println!("{}", "*".repeat(80));
    }
}



#[tokio::main]
async fn main()  -> std::result::Result<(), Box<dyn Error>>{
    let url = "https://imdb-api.com/en/API/BoxOffice/k_b20xi08t";
    send_request(url).await;
    Ok(())
}
