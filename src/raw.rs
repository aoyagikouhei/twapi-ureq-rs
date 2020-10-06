use ureq::{Request, Response};

use twapi_oauth::encode;

fn make_query(list: &Vec<(&str, &str)>, separator: &str) -> String {
    let mut result = String::from("");
    for item in list {
        if "" != result {
            result.push_str(separator);
        }
        result.push_str(&format!("{}={}", item.0, encode(item.1)));
    }
    result
}

pub(crate) fn get(url: &str, query_options: &Vec<(&str, &str)>, authorization: &str) -> Response {
    let url = if query_options.len() > 0 {
        format!("{}?{}", url, make_query(query_options, "&"))
    } else {
        url.to_owned()
    };
    let mut request = ureq::get(&url).set("Authorization", authorization).build();
    request.call()
}

pub(crate) fn post(
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

pub(crate) fn json(
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

pub(crate) fn put(url: &str, query_options: &Vec<(&str, &str)>, authorization: &str) -> Response {
    let mut request = ureq::put(url).set("Authorization", authorization).build();
    apply_query_options(&mut request, query_options);
    request.call()
}

pub(crate) fn delete(url: &str, query_options: &Vec<(&str, &str)>, authorization: &str) -> Response {
    let mut request = ureq::delete(url)
        .set("Authorization", authorization)
        .build();
    apply_query_options(&mut request, query_options);
    request.call()
}

pub(crate) fn multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    mut data: crate::form::MultiPart,
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