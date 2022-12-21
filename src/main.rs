use futures::prelude::*;
use serde_json::{Value};
use reqwest::Client;
use tokio;

const CONCURRENT_REQUESTS: usize = 2;

#[tokio::main]
async fn main() {
  let client = Client::new();

  let urls = vec![
    "https://api.github.com/users/rafaelcastrocouto",
    "https://api.github.com/users/spicylobstergames"
  ];

  let bodies = stream::iter(urls)
    .map(|url| {
      let client = &client;
      async move {
        let resp = client
          .get(url)
          .header(hyper::header::USER_AGENT, "uda")
          .send().await?;
        resp.bytes().await
      }
    })
    .buffer_unordered(CONCURRENT_REQUESTS);

  bodies
    .for_each(|b| async {
      match b {
        Ok(b) => {
          let str = String::from_utf8(b.to_vec()).unwrap();
          let mut obj:Value = serde_json::from_str(&str).unwrap();
          println!("{}", obj["name"]);
        },
        Err(err) => eprintln!("Got an error: {}", err),
      }
    })
    .await;
}