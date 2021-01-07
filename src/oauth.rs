use std::collections::HashMap;
use twapi_oauth::calc_oauth_header;
use ureq::{Error, Response};

pub fn get_bearer_token_response(
    consumer_key: &str,
    consumer_secret: &str,
) -> Result<Response, Error> {
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
    match get_bearer_token_response(consumer_key, consumer_secret) {
        Ok(response) => match response.into_json::<serde_json::Value>() {
            Ok(json) => match json["access_token"].as_str() {
                Some(access_token) => Some(access_token.to_string()),
                None => None,
            },
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub fn request_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_callback: &str,
    x_auth_access_type: Option<&str>,
) -> Result<Response, Error> {
    let uri = "https://api.twitter.com/oauth/request_token";
    let mut header_options = vec![("oauth_callback", oauth_callback)];
    if let Some(x_auth_access_type) = x_auth_access_type {
        header_options.push(("x_auth_access_type", x_auth_access_type));
    }
    let signed = calc_oauth_header(
        &format!("{}&", consumer_secret),
        consumer_key,
        &header_options,
        "POST",
        uri,
        &vec![],
    );
    ureq::post(uri)
        .set("Authorization", &format!("OAuth {}", signed))
        .call()
}

pub fn request_token(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_callback: &str,
    x_auth_access_type: Option<&str>,
) -> Result<HashMap<String, String>, Error> {
    let response = request_token_response(
        consumer_key,
        consumer_secret,
        oauth_callback,
        x_auth_access_type,
    )?;
    Ok(parse_oauth_body(response))
}

pub fn access_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_token: &str,
    oauth_token_secret: &str,
    oauth_verifier: &str,
) -> Result<Response, Error> {
    let uri = "https://api.twitter.com/oauth/access_token";
    let signed = calc_oauth_header(
        &format!("{}&{}", consumer_secret, oauth_token_secret),
        consumer_key,
        &vec![
            ("oauth_token", oauth_token),
            ("oauth_verifier", oauth_verifier),
        ],
        "POST",
        uri,
        &vec![],
    );
    ureq::post(uri)
        .set("Authorization", &format!("OAuth {}", signed))
        .call()
}

pub fn access_token(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_token: &str,
    oauth_token_secret: &str,
    oauth_verifier: &str,
) -> Result<HashMap<String, String>, Error> {
    let response = access_token_response(
        consumer_key,
        consumer_secret,
        oauth_token,
        oauth_token_secret,
        oauth_verifier,
    )?;
    Ok(parse_oauth_body(response))
}

fn parse_oauth_body(response: Response) -> HashMap<String, String> {
    let mut result = HashMap::new();
    match response.into_string() {
        Ok(body) => {
            for item in body.split("&") {
                let mut pair = item.split("=");
                result.insert(
                    pair.next().unwrap().to_string(),
                    pair.next().unwrap().to_string(),
                );
            }
        }
        Err(_) => {}
    }
    result
}
