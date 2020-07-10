use async_std::prelude::*;
use serde::{Serialize, Deserialize};

// #[derive(
//     Serialize, Deserialize,
//     Debug)]
// struct RedditListing {
//     data: RedditListingObject,
//     before: String,
//     after: String
// }

// #[derive(
//     Serialize, Deserialize,
//     Debug)]
// struct RedditListingObject {
//     children: Vec<RedditListingObjectData>
// }

// #[derive(
//     Serialize, Deserialize,
//     Debug)]
// struct RedditListingObjectData {
//     kind: RedditListingObjectKind,
//     subreddit: String,
//     title: String,
//     name: String,
//     created_utc: u64,
//     url: String
// }

// #[derive(
//     Serialize, Deserialize,
//     Debug)]
// #[serde(rename_all = "lowercase")]
// enum RedditListingObjectKind {
//     T1,
//     T2,
//     T3,
//     T4,
//     T5,
//     T6
// }

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let interval_seconds_str = std::env::var("R_INTERVAL_SECONDS")?;
    let interval_seconds = interval_seconds_str.parse::<u64>().map_err(|_| "Failed to parse R_INTERVAL_SECONDS to u64")?;

    let client = surf::Client::new();

    // start polling reddit
    loop {
        let mut r = client.get("https://www.reddit.com/r/bapcsalescanada/new.json").set_header("User-Agent", "butt").await?;
        let body = r.body_bytes().await?;
        let v: serde_json::Value = serde_json::from_reader(std::io::Cursor::new(&body))?;
        dbg!(v);
        async_std::task::sleep(std::time::Duration::from_secs(interval_seconds)).await;
    }
}
