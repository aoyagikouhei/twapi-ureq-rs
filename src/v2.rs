use twapi_oauth::{oauth2_authorization_header};
use ureq::{Response};

pub fn get(url: &str, query_options: &Vec<(&str, &str)>, bearer_token: &str) -> Response {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::get(url, query_options, &authorization)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::env;

    #[test]
    fn test_api() {
        let consumer_key = env::var("CONSUMER_KEY").unwrap();
        let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
        let bearer_token = oauth::get_bearer_token(&consumer_key, &consumer_secret).unwrap();

        // search
        let res = v2::get(
            "https://api.twitter.com/1.1/search/tweets.json",
            &vec![("q", "東京&埼玉"), ("count", "2")],
            &bearer_token,
        );
        println!("{:?}", res.into_json());
    }
}