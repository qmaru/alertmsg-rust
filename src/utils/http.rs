pub mod minireq {
    use reqwest::blocking::Client;
    use reqwest::header::{HeaderMap, HeaderValue};
    use serde_json::Value;

    fn default_header() -> HeaderMap {
        let header_value = "VicMessage-rust";
        let mut h = HeaderMap::new();
        h.insert("User-Agent", HeaderValue::from_static(header_value));
        h
    }

    fn client() -> reqwest::blocking::Client {
        let client = Client::builder().default_headers(default_header()).build();

        match client {
            Ok(client) => client,
            Err(err) => panic!("HTTP Client Error: {err}"),
        }
    }

    pub fn post(url: &String, body: &Value) -> Result<Value, reqwest::Error> {
        let client = client();
        let response = client.post(url).json(&body).send()?;
        let body = response.json::<Value>()?;
        Ok(body)
    }
}
