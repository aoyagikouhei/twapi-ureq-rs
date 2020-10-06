use twapi_oauth::{oauth2_authorization_header};
use ureq::{Response};

pub fn get(url: &str, query_options: &Vec<(&str, &str)>, bearer_token: &str) -> Response {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::get(url, query_options, &authorization)
}

pub fn post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Response {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::post(url, query_options, form_options, &authorization)
}

pub fn json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: serde_json::Value,
    bearer_token: &str,
) -> Response {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::json(url, query_options, data, &authorization)
}

pub fn put(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Response {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::put(url, query_options, &authorization)
}

pub fn delete(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Response {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::delete(url, query_options, &authorization)
}

pub fn multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: crate::form::MultiPart,
    bearer_token: &str,
) -> Response {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::multipart(url, query_options, data, &authorization)
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
