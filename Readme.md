# twapi-ureq-rs

Twitter OAuth library used by ureq.

[Documentation](https://docs.rs/twapi-ureq)

## Features
- Application Only Authentication
- User Authentication
- Oauth1.0 Authentication
- Oauth2.0 Authentication
- JSON support(ex. dm_event, welcome_message, media_metadata, etc.)
- Multipart support(ex. post_media_upload)

## Example
```rust
use twapi_ureq::*;
use std::io::{BufReader, Cursor, Read};

fn main() {
    // Set up Environment Variables
    // CONSUMER_KEY
    // CONSUMER_SECRET
    // ACCESS_KEY
    // ACCESS_SECRET

    // OAuth2.0 Authentication
    let client = v2::Client::new_by_env().unwrap().unwrap();

    // search(Application Only Authentication)
    let res = client
        .get(
            "https://api.twitter.com/1.1/search/tweets.json",
            &vec![("q", "東京&埼玉"), ("count", "2")],
        )
        .unwrap();
    println!("{:?}", res.into_json::<serde_json::Value>());

    // OAuth1.0 Authentication
    let client = v1::Client::new_by_env().unwrap();

    // home_timeline
    let url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
    let query_options = vec![("count", "2")];
    let res = client.get(url, &query_options).unwrap();
    println!("{:?}", res.into_json::<serde_json::Value>().unwrap());

    // statuses/update
    let url = "https://api.twitter.com/1.1/statuses/update.json";
    let form_options = vec![
        ("status", "!\"'#$%&\\()+,/:;<=>?@[\\]^`{|}~;-._* 全部"),
        ("in_reply_to_status_id", "1178811297455935488"),
    ];
    let res = client.post(url, &vec![], &form_options).unwrap();
    println!("{:?}", res.into_json::<serde_json::Value>());

    // direct_messages new(JSON support)
    let url = "https://api.twitter.com/1.1/direct_messages/events/new.json";
    let data = r#"{
                "event": {
                    "type": "message_create",
                    "message_create": {
                        "target": {
                            "recipient_id": "19522946"
                        },
                        "message_data": {
                            "text": "予定表〜①ﾊﾝｶｸだ!"
                        }
                    }
                }
            }"#;
    let data: serde_json::Value = serde_json::from_str(data).unwrap();
    let res = client.json(url, &vec![], data).unwrap();
    println!("{:?}", res.into_json::<serde_json::Value>());

    // media/upload(Multipart support)
    let metadata = std::fs::metadata("test.jpg").unwrap();
    let file_size = metadata.len();
    let f = std::fs::File::open("test.jpg").unwrap();
    let mut cursor = Cursor::new(vec![0; file_size as usize]);
    let mut reader = BufReader::new(f);
    reader.read(cursor.get_mut()).unwrap();

    let mut data = form::MultiPart::new();
    data.add_cursor("media", cursor);

    let url = "https://upload.twitter.com/1.1/media/upload.json";
    let res = client.multipart(url, &vec![], data).unwrap();
    println!("{:?}", res.into_json::<serde_json::Value>());
}
```