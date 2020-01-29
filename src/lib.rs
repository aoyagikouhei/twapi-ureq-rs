use bytebuffer::ByteBuffer;
use rand::seq::SliceRandom;
use std::io::Cursor;
use twapi_oauth::{oauth1_authorization_header, oauth2_authorization_header};
use ureq::{Request, Response};

pub fn get(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Response {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "GET",
        url,
        &query_options,
    );
    raw_get(url, query_options, &authorization)
}

pub fn get_v2(url: &str, query_options: &Vec<(&str, &str)>, bearer_token: &str) -> Response {
    let authorization = oauth2_authorization_header(bearer_token);
    raw_get(url, query_options, &authorization)
}

fn raw_get(url: &str, query_options: &Vec<(&str, &str)>, authorization: &str) -> Response {
    let mut request = ureq::get(url).set("Authorization", authorization).build();
    apply_query_options(&mut request, query_options);
    request.call()
}

pub fn post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Response {
    let mut merged_options = query_options.clone();
    for option in form_options {
        merged_options.push(*option);
    }
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "POST",
        url,
        &merged_options,
    );
    raw_post(url, query_options, form_options, &authorization)
}

fn raw_post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Response {
    let mut request = ureq::post(url)
        .set("Authorization", authorization)
        .set(
            "Content-Type",
            "application/x-www-form-urlencoded;charset=UTF-8",
        )
        .build();
    apply_query_options(&mut request, query_options);
    request.send_string(&make_body(form_options))
}

pub fn json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: serde_json::Value,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Response {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "POST",
        url,
        &query_options,
    );
    raw_json(url, query_options, data, &authorization)
}

fn raw_json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: serde_json::Value,
    authorization: &str,
) -> Response {
    let mut request = ureq::post(url)
        .set("Authorization", authorization)
        .set("Content-Type", "application/json")
        .build();
    apply_query_options(&mut request, query_options);
    request.send_json(data)
}

pub fn put(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Response {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "PUT",
        url,
        &query_options,
    );
    raw_put(url, query_options, &authorization)
}

fn raw_put(url: &str, query_options: &Vec<(&str, &str)>, authorization: &str) -> Response {
    let mut request = ureq::put(url).set("Authorization", authorization).build();
    apply_query_options(&mut request, query_options);
    request.call()
}

pub fn delete(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Response {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "DELETE",
        url,
        &query_options,
    );
    raw_delete(url, query_options, &authorization)
}

fn raw_delete(url: &str, query_options: &Vec<(&str, &str)>, authorization: &str) -> Response {
    let mut request = ureq::delete(url)
        .set("Authorization", authorization)
        .build();
    apply_query_options(&mut request, query_options);
    request.call()
}

pub fn multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: MultiPart,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Response {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "POST",
        url,
        &query_options,
    );
    raw_multipart(url, query_options, data, &authorization)
}

fn raw_multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    mut data: MultiPart,
    authorization: &str,
) -> Response {
    let mut request = ureq::post(url)
        .set("Authorization", authorization)
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", data.boundary),
        )
        .build();
    apply_query_options(&mut request, query_options);
    request.send_bytes(&data.to_bytes())
}

fn make_body(form_options: &Vec<(&str, &str)>) -> String {
    match serde_urlencoded::to_string(form_options) {
        Ok(body) => body
            .replace('+', "%20")
            .replace('*', "%2A")
            .replace("%7E", "~")
            .into(),
        Err(_) => String::from(""),
    }
}

fn apply_query_options(request: &mut Request, query_options: &Vec<(&str, &str)>) {
    for query_option in query_options {
        request.query(query_option.0, query_option.1);
    }
}

pub fn get_bearer_token_response(consumer_key: &str, consumer_secret: &str) -> Response {
    let key = base64::encode(&format!("{}:{}", consumer_key, consumer_secret));
    ureq::post("https://api.twitter.com/oauth2/token")
        .set(
            "Content-Type",
            "application/x-www-form-urlencoded;charset=UTF-8",
        )
        .set("Authorization", &format!("Basic {}", key))
        .send_string("grant_type=client_credentials")
}

pub fn get_bearer_token(consumer_key: &str, consumer_secret: &str) -> Option<String> {
    let response = get_bearer_token_response(consumer_key, consumer_secret);
    match response.into_json() {
        Ok(json) => match json["access_token"].as_str() {
            Some(access_token) => Some(access_token.to_string()),
            None => None,
        },
        Err(_) => None,
    }
}

pub struct MultiPart {
    pub boundary: String,
    buffer: ByteBuffer,
    before_flag: bool,
}

impl MultiPart {
    pub fn new() -> Self {
        let mut rng = &mut rand::thread_rng();
        let postfix = String::from_utf8(
            "-_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                .as_bytes()
                .choose_multiple(&mut rng, 30)
                .cloned()
                .collect(),
        )
        .unwrap();
        Self {
            boundary: format!("---------------------------{}", postfix),
            buffer: ByteBuffer::new(),
            before_flag: true,
        }
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        if self.before_flag {
            self.write_boundary(true);
            self.before_flag = false;
        }
        self.buffer.to_bytes()
    }

    pub fn add_string(&mut self, name: &str, data: &str) {
        self.write_boundary(false);
        self.write_header(name);
        self.buffer.write_bytes(&format!("{}\r\n", data).as_bytes());
    }

    pub fn add_cursor(&mut self, name: &str, cursor: Cursor<Vec<u8>>) {
        self.write_boundary(false);
        self.write_header(name);
        self.buffer.write_bytes(cursor.get_ref());
        self.buffer.write_bytes("\r\n".as_bytes());
    }

    fn write_header(&mut self, name: &str) {
        self.buffer.write_bytes(
            &format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", name).as_bytes(),
        );
    }

    fn write_boundary(&mut self, last_flag: bool) {
        self.buffer.write_bytes("--".as_bytes());
        self.buffer.write_bytes(&self.boundary.as_bytes());
        if last_flag {
            self.buffer.write_bytes("--".as_bytes());
        } else {
            self.buffer.write_bytes("\r\n".as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::env;
    use std::io::{BufReader, Cursor, Read};

    #[test]
    fn test_api() {
        let consumer_key = env::var("CONSUMER_KEY").unwrap();
        let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
        let access_key = env::var("ACCESS_KEY").unwrap();
        let access_secret = env::var("ACCESS_SECRET").unwrap();
        let bearer_token = get_bearer_token(&consumer_key, &consumer_secret).unwrap();

        // search
        let res = get_v2(
            "https://api.twitter.com/1.1/search/tweets.json",
            &vec![("q", "東京&埼玉"), ("count", "2")],
            &bearer_token,
        );
        println!("{:?}", res.into_json());

        // home_timeline
        let url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let query_options = vec![("count", "2")];
        let res = get(
            url,
            &query_options,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        );
        println!("{:?}", res.into_json());

        // statuses/update
        let url = "https://api.twitter.com/1.1/statuses/update.json";
        let form_options = vec![
            ("status", "!\"'#$%&\\()+,/:;<=>?@[\\]^`{|}~;-._* 全部"),
            ("in_reply_to_status_id", "1178811297455935488"),
        ];
        let res = post(
            url,
            &vec![],
            &form_options,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        );
        println!("{:?}", res.into_json());

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
        let res = json(
            url,
            &vec![],
            data,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        );
        println!("{:?}", res.into_json());

        // media/upload
        let metadata = std::fs::metadata("test.jpg").unwrap();
        let file_size = metadata.len();
        let f = std::fs::File::open("test.jpg").unwrap();
        let mut cursor = Cursor::new(vec![0; file_size as usize]);
        let mut reader = BufReader::new(f);
        reader.read(cursor.get_mut()).unwrap();

        let mut data = MultiPart::new();
        data.add_cursor("media", cursor);

        let url = "https://upload.twitter.com/1.1/media/upload.json";
        let res = multipart(
            url,
            &vec![],
            data,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        );
        println!("{:?}", res.into_json());
    }
}
