use twapi_oauth::oauth1_authorization_header;
use ureq::{Error, Response};

pub struct Client {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

impl Client {
    pub fn new(
        consumer_key: &str,
        consumer_secret: &str,
        access_key: &str,
        access_secret: &str,
    ) -> Self {
        Self {
            consumer_key: consumer_key.to_owned(),
            consumer_secret: consumer_secret.to_owned(),
            access_key: access_key.to_owned(),
            access_secret: access_secret.to_owned(),
        }
    }

    pub fn new_by_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            consumer_key: std::env::var("CONSUMER_KEY")?,
            consumer_secret: std::env::var("CONSUMER_SECRET")?,
            access_key: std::env::var("ACCESS_KEY")?,
            access_secret: std::env::var("ACCESS_SECRET")?,
        })
    }

    fn calc_oauth(&self, method: &str, url: &str, query_options: &Vec<(&str, &str)>) -> String {
        oauth1_authorization_header(
            &self.consumer_key,
            &self.consumer_secret,
            &self.access_key,
            &self.access_secret,
            method,
            url,
            &query_options,
        )
    }

    pub fn get(&self, url: &str, query_options: &Vec<(&str, &str)>) -> Result<Response, Error> {
        crate::raw::get(
            url,
            query_options,
            &self.calc_oauth("GET", url, &query_options),
        )
    }

    pub fn post(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        form_options: &Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        let mut merged_options = query_options.clone();
        for option in form_options {
            merged_options.push(*option);
        }
        crate::raw::post(
            url,
            query_options,
            form_options,
            &self.calc_oauth("POST", url, &merged_options),
        )
    }

    pub fn json(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        data: serde_json::Value,
    ) -> Result<Response, Error> {
        crate::raw::json(
            url,
            query_options,
            data,
            &self.calc_oauth("POST", url, &query_options),
        )
    }

    pub fn put(&self, url: &str, query_options: &Vec<(&str, &str)>) -> Result<Response, Error> {
        crate::raw::put(
            url,
            query_options,
            &self.calc_oauth("PUT", url, &query_options),
        )
    }

    pub fn delete(&self, url: &str, query_options: &Vec<(&str, &str)>) -> Result<Response, Error> {
        crate::raw::delete(
            url,
            query_options,
            &self.calc_oauth("DELETE", url, &query_options),
        )
    }

    pub fn multipart(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        data: crate::form::MultiPart,
    ) -> Result<Response, Error> {
        crate::raw::multipart(
            url,
            query_options,
            data,
            &self.calc_oauth("POST", url, &query_options),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{form, v1::Client};
    use std::io::{BufReader, Cursor, Read};

    #[test]
    fn test_api() {
        let client = Client::new_by_env().unwrap();

        // search
        let url = "https://api.twitter.com/1.1/search/tweets.json";
        let query_options = vec![("q", "*abc"), ("count", "2")];
        let res = client.get(url, &query_options).unwrap();
        println!("{:?}", res.into_json::<serde_json::Value>().unwrap());

        // home_timeline
        let url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let query_options = vec![("count", "2")];
        let res = client.get(url, &query_options).unwrap();
        println!("{:?}", res.into_json::<serde_json::Value>());

        // statuses/update
        let url = "https://api.twitter.com/1.1/statuses/update.json";
        let form_options = vec![
            ("status", "!\"'#$%&\\()+,/:;<=>?@[\\]^`{|}~;-._* 全部"),
            ("in_reply_to_status_id", "1178811297455935488"),
        ];
        let res = client.post(url, &vec![], &form_options).unwrap();
        println!("{:?}", res.into_json::<serde_json::Value>());

        // direct_messages new
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

        // media/upload
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
}
