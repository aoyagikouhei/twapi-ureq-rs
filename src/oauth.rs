use twapi_oauth::{calc_oauth_header};
use ureq::{Response};

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

pub fn request_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_callback: &str,
    x_auth_access_type: Option<&str>,
) -> Response {
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
) -> Option<Vec<(String, String)>> {
    let response = request_token_response(consumer_key, consumer_secret, oauth_callback, x_auth_access_type);
    parse_oauth_body(response)
}

pub fn access_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_token: &str,
    oauth_token_secret: &str,
    oauth_verifier: &str,
) -> Response {
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
) -> Option<Vec<(String, String)>> {
    let response = access_token_response(consumer_key, consumer_secret, oauth_token, oauth_token_secret, oauth_verifier);
    parse_oauth_body(response)
}

fn parse_oauth_body(response: Response) -> Option<Vec<(String, String)>> {
    if !response.ok() {
        return None;
    }
    match response.into_string() {
        Ok(body) => {
            Some(body.split("&").map(|it| {
                let mut pair = it.split("=");
                (pair.next().unwrap().to_string(), pair.next().unwrap().to_string())
            }).collect())
        },
        Err(_) => None,
    }
}