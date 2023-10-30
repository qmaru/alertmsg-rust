use super::http::fast_post;
use serde_json::{json, Value};

pub trait BotMethod {
    fn send_message(&self, message: Value);
}

pub struct Telegram {
    pub webhook: String,
}
pub struct Dingtalk {
    pub webhook: String,
    pub keyword: String,
}

impl BotMethod for Telegram {
    fn send_message(&self, message: Value) {
        let url = format!("{}/sendMessage", self.webhook);
        let result = fast_post(&url, message);
        match result {
            Ok(resp) => {
                let okcode = resp.get("ok");
                if let Some(okcode) = okcode {
                    let ok = okcode.as_bool();
                    if let Some(ok) = ok {
                        if ok {
                            println!("发送成功");
                        } else {
                            println!("发送失败: {}", resp);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}

impl Telegram {
    pub fn make_message(&self, message: String, chat_id: String, is_markdown: bool) -> Value {
        let mut text = json!({
            "chat_id": chat_id,
            "text": message,
        });

        if is_markdown {
            text["parse_mode"] = Value::String("MarkdownV2".to_string());
        }
        text
    }
}

impl BotMethod for Dingtalk {
    fn send_message(&self, message: Value) {
        let result = fast_post(&self.webhook, message);

        match result {
            Ok(resp) => {
                let errcode = resp.get("errcode");
                if let Some(errcode) = errcode {
                    let code = errcode.as_i64();
                    if let Some(code) = code {
                        if code == 0 {
                            println!("发送成功");
                        } else {
                            let errmsg = resp.get("errmsg");
                            if let Some(errmsg) = errmsg {
                                let msg = errmsg.as_str();
                                if let Some(msg) = msg {
                                    println!("发送失败: {code} / {msg}");
                                }
                            }
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("{}", error)
            }
        }
    }
}

impl Dingtalk {
    pub fn make_text_message(&self, message: String, at_ids: Vec<String>) -> Value {
        let mut message = message;
        if self.keyword != "" {
            message = format!("{}\n{}", self.keyword, message);
        }
        let mut text = json!({
            "msgtype": "text",
            "text": {
                "content": message
            },
        });

        if at_ids.len() != 0 {
            text["at"] = json!({
                "atUserIds": at_ids,
                "isAtAll":   false,
            })
        }
        text
    }

    pub fn make_markdown_message(
        &self,
        markdown_title: String,
        message: String,
        at_ids: Vec<String>,
    ) -> Value {
        let mut markdown_title = markdown_title;
        if self.keyword != "" {
            markdown_title = format!("{}\n{}", markdown_title, self.keyword,);
        }
        let mut markdown = json!({
            "msgtype": "markdown",
            "markdown": {
                "title": markdown_title,
                "text": message
            },
        });

        if at_ids.len() != 0 {
            markdown["at"] = json!({
                "atUserIds": at_ids,
                "isAtAll":   false,
            })
        }
        markdown
    }
}
