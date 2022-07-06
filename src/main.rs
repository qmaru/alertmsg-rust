use alertmsg_rust::utils::cli::build;
use alertmsg_rust::utils::message::{dingtalk, telegram, SendMethod};

fn main() {
    let app = build::create();
    let args = app.get_matches();

    match args.subcommand() {
        None => println!(""),
        Some(matches) => {
            let name = matches.0;
            let args = matches.1;
            if name == "telegram" {
                let webhook = args.value_of("webhook_url");
                let message = args.value_of("message");
                let chat_id = args.value_of("chat_id");

                let mut arg_w: &str = "";
                let mut arg_m: &str = "";
                let mut arg_i: &str = "";

                if let Some(webhook) = webhook {
                    arg_w = webhook;
                };

                if let Some(message) = message {
                    arg_m = message;
                };

                if let Some(chat_id) = chat_id {
                    arg_i = chat_id;
                };

                let t = telegram::Message {
                    webhook: arg_w.to_string(),
                    chat_id: arg_i.to_string(),
                    message: arg_m.to_string(),
                };
                t.send_text().unwrap();
            } else if name == "dingtalk" {
                let webhook = args.value_of("webhook_url");
                let message = args.value_of("message");
                let keyword = args.value_of("keyword");
                let at_id = args.value_of("at");

                let mut arg_w: &str = "";
                let mut arg_m: &str = "";
                let mut arg_k: &str = "";
                let mut arg_a: &str = "";

                if let Some(webhook) = webhook {
                    arg_w = webhook;
                };

                if let Some(message) = message {
                    arg_m = message;
                };

                if let Some(keyword) = keyword {
                    arg_k = keyword;
                };

                if let Some(at_id) = at_id {
                    arg_a = at_id;
                };

                let d = dingtalk::Message {
                    webhook: arg_w.to_string(),
                    keyword: arg_k.to_string(),
                    message: arg_m.to_string(),
                    at_ids: vec![arg_a.to_string()],
                    markdown_title: String::from(""),
                };
                d.send_text().unwrap();
            }
        }
    }
}
