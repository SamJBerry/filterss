use axum::{Router, extract::Query, routing::get};
use base64::{Engine, engine::general_purpose::URL_SAFE};
use feed_rs::parser;
use serde::Deserialize;
use ureq;

#[derive(Deserialize, Debug)]
struct Params {
    feed: String,
    filter: String,
}

async fn decode(params: Query<Params>) {
    let feed_url = URL_SAFE.decode(&params.feed).unwrap();
    let feed_url = std::str::from_utf8(&feed_url).unwrap();
    println!("{} with {}", &feed_url, &params.filter);
    let body: String = ureq::get(feed_url)
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();
    let mut feed = parser::parse(body.as_bytes()).unwrap();
    println!("Num entries: {}", feed.entries.len());
    feed.entries.retain(|entry| {
        let delete: bool = {
            entry
                .summary
                .as_ref()
                .is_some_and(|summary| summary.content.contains(&params.filter))
        };
        !delete
    });
    println!("Num entries after filter: {}", feed.entries.len());
    println!("{:?}", feed.entries[0].summary);
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(decode));
    println!(
        "test string: {:?}",
        URL_SAFE.encode("https://www.githubstatus.com/history.rss")
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
