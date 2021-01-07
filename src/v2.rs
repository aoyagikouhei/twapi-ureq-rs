use twapi_oauth::oauth2_authorization_header;
use ureq::{Error, Response};

pub struct Client {
    bearer_token: String,
}

impl Client {
    pub fn new(consumer_key: &str, consumer_secret: &str) -> Option<Self> {
        crate::oauth::get_bearer_token(&consumer_key, &consumer_secret)
            .map(|bearer_token| Self { bearer_token })
    }

    pub fn new_by_env() -> Result<Option<Self>, std::env::VarError> {
        Ok(Self::new(
            &std::env::var("CONSUMER_KEY")?,
            &std::env::var("CONSUMER_SECRET")?,
        ))
    }

    fn make_header(&self) -> String {
        oauth2_authorization_header(&self.bearer_token)
    }

    pub fn get(&self, url: &str, query_options: &Vec<(&str, &str)>) -> Result<Response, Error> {
        crate::raw::get(url, query_options, &self.make_header())
    }

    pub fn post(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        form_options: &Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        crate::raw::post(url, query_options, form_options, &self.make_header())
    }

    pub fn json(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        data: serde_json::Value,
    ) -> Result<Response, Error> {
        crate::raw::json(url, query_options, data, &self.make_header())
    }

    pub fn put(&self, url: &str, query_options: &Vec<(&str, &str)>) -> Result<Response, Error> {
        crate::raw::put(url, query_options, &self.make_header())
    }

    pub fn delete(&self, url: &str, query_options: &Vec<(&str, &str)>) -> Result<Response, Error> {
        crate::raw::delete(url, query_options, &self.make_header())
    }

    pub fn multipart(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        data: crate::form::MultiPart,
    ) -> Result<Response, Error> {
        crate::raw::multipart(url, query_options, data, &self.make_header())
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::Client;

    #[test]
    fn test_api() {
        let client = Client::new_by_env().unwrap().unwrap();

        // search
        let res = client
            .get(
                "https://api.twitter.com/1.1/search/tweets.json",
                &vec![("q", "東京&埼玉"), ("count", "2")],
            )
            .unwrap();
        println!("{:?}", res.into_json::<serde_json::Value>());
    }
}
