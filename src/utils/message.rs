pub trait SendMethod {
    fn send_text(&self) -> Result<serde_json::Value, reqwest::Error>;
    fn send_markdown(&self) -> Result<serde_json::Value, reqwest::Error>;
}

pub mod telegram {
    use super::SendMethod;
    use crate::utils::http::minireq;
    use serde_json::{json, Value};

    pub struct Message {
        pub webhook: String,
        pub message: String,
        pub chat_id: String,
    }

    fn make_post_url(url: &String) -> String {
        let post_url = format!("{}/sendMessage", url);
        post_url
    }

    fn make_body(chat_id: &String, text: &String, is_markdown: bool) -> Value {
        let mut post_body = json!({
            "chat_id": chat_id,
            "text": text,
        });

        if is_markdown {
            post_body["parse_mode"] = Value::String("MarkdownV2".to_string());
        }
        post_body
    }

    fn make_request(post_url: String, post_body: Value) -> Result<Value, reqwest::Error> {
        let post_res = minireq::post(&post_url, &post_body);
        match post_res {
            Ok(result) => {
                let ok = &result["ok"].as_bool();
                if let Some(true) = ok {
                    return Ok(result);
                }
                let description = &result["description"];
                panic!("{description}");
            }
            Err(err) => panic!("{err}"),
        }
    }

    impl SendMethod for Message {
        fn send_text(&self) -> Result<Value, reqwest::Error> {
            let post_url = make_post_url(&self.webhook);
            let post_body = make_body(&self.chat_id, &self.message, false);
            let post_result = make_request(post_url, post_body)?;
            Ok(post_result)
        }

        fn send_markdown(&self) -> Result<Value, reqwest::Error> {
            let post_url = make_post_url(&self.webhook);
            let post_body = make_body(&self.chat_id, &self.message, true);
            let post_result = make_request(post_url, post_body)?;
            Ok(post_result)
        }
    }
}

pub mod dingtalk {
    use super::SendMethod;
    use crate::utils::http::minireq;
    use serde_json::{json, Value};

    pub struct Message {
        pub webhook: String,
        pub keyword: String,
        pub message: String,
        pub markdown_title: String,
        pub at_ids: Vec<String>,
    }

    fn make_request(post_url: &String, post_body: Value) -> Result<Value, reqwest::Error> {
        let post_res = minireq::post(&post_url, &post_body);
        match post_res {
            Ok(result) => {
                let ok = &result["errmsg"].as_str();
                if let Some("ok") = ok {
                    return Ok(result);
                }
                let description = &result["errmsg"];
                panic!("{description}");
            }
            Err(err) => panic!("{err}"),
        }
    }

    impl SendMethod for Message {
        fn send_text(&self) -> Result<Value, reqwest::Error> {
            let post_message = format!("{}\n{}", &self.keyword, &self.message);
            let post_url = &self.webhook;
            let mut post_body = json!({
                "msgtype": "text",
                "text": {
                    "content": post_message
                },
            });

            let id_len = &self.at_ids.len();
            if *id_len > 0 {
                post_body["at"] = json!({
                    "atUserIds": &self.at_ids,
                    "isAtAll":   false,
                })
            }

            let post_res = make_request(post_url, post_body)?;
            Ok(post_res)
        }

        fn send_markdown(&self) -> Result<Value, reqwest::Error> {
            let post_marktitle = format!("{}\n{}", &self.markdown_title, &self.keyword);
            let post_url = &self.webhook;
            let mut post_body = json!({
                "msgtype": "markdown",
                "markdown": {
                    "title": post_marktitle,
                    "text": &self.message
                },
            });

            let id_len = &self.at_ids.len();
            if *id_len > 0 {
                post_body["at"] = json!({
                    "atUserIds": &self.at_ids,
                    "isAtAll":   false,
                })
            }

            let post_res = make_request(post_url, post_body)?;
            Ok(post_res)
        }
    }
}
