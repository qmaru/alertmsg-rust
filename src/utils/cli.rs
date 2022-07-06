pub mod build {
    use clap::{App, AppSettings, Arg, SubCommand};

    const APP_NAME: &str = "alertmsg-rust";
    const APP_VERSION: &str = "1.0.0";
    const APP_ABOUT: &str = "Telegram & Dingtalk Message Sender";

    pub fn create() -> App<'static> {
        let app_cli = App::new(APP_NAME)
            .version(APP_VERSION)
            .about(APP_ABOUT)
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(
                SubCommand::with_name("telegram")
                    .about("Telegram Message")
                    .arg(
                        Arg::with_name("webhook_url")
                            .short('u')
                            .long("url")
                            .takes_value(true)
                            .required(true)
                            .help("Webhook URL"),
                    )
                    .arg(
                        Arg::with_name("message")
                            .short('m')
                            .long("message")
                            .takes_value(true)
                            .required(true)
                            .help("Message Text"),
                    )
                    .arg(
                        Arg::with_name("chat_id")
                            .short('i')
                            .long("id")
                            .allow_hyphen_values(true)
                            .takes_value(true)
                            .required(true)
                            .help("Chat ID"),
                    ),
            )
            .subcommand(
                SubCommand::with_name("dingtalk")
                    .about("Dingtalk Message")
                    .arg(
                        Arg::with_name("webhook_url")
                            .short('u')
                            .long("url")
                            .takes_value(true)
                            .required(true)
                            .help("Webhook URL"),
                    )
                    .arg(
                        Arg::with_name("message")
                            .short('m')
                            .long("message")
                            .takes_value(true)
                            .required(true)
                            .help("Message Text"),
                    )
                    .arg(
                        Arg::with_name("keyword")
                            .short('k')
                            .long("kw")
                            .takes_value(true)
                            .required(true)
                            .help("Keyword"),
                    )
                    .arg(
                        Arg::with_name("at")
                            .short('a')
                            .long("at")
                            .allow_hyphen_values(true)
                            .takes_value(true)
                            .help("At someone by user_id"),
                    ),
            );
        app_cli
    }
}
