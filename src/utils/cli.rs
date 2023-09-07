use clap::{App, AppSettings, Arg, SubCommand};

pub struct CommandLine {
    name: String,
    version: String,
    about: String,
}

impl CommandLine {
    pub fn new(name: String, version: String, about: String) -> CommandLine {
        CommandLine {
            name,
            version,
            about,
        }
    }

    pub fn create<'a, 'b>(&'a self) -> App<'a, 'b> {
        App::new(&self.name)
            .version(&*self.version)
            .about(&*self.about)
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(
                SubCommand::with_name("telegram")
                    .about("Send Message By Telegram")
                    .arg(
                        Arg::with_name("message")
                            .short("m")
                            .long("message")
                            .takes_value(true)
                            .required(true)
                            .help("Telegram Message"),
                    )
                    .arg(
                        Arg::with_name("markdown")
                            .takes_value(false)
                            .long("markdown")
                            .help("Use Markdown"),
                    ),
            )
            .subcommand(
                SubCommand::with_name("dingtalk")
                    .about("Send Message By DingTalk")
                    .subcommand(SubCommand::with_name("users").about("list user"))
                    .arg(
                        Arg::with_name("message")
                            .short("m")
                            .long("message")
                            .takes_value(true)
                            .required(false)
                            .help("Dingtalk Message"),
                    )
                    .arg(
                        Arg::with_name("title")
                            .takes_value(true)
                            .short("t")
                            .long("title")
                            .help("Markdown title"),
                    )
                    .arg(
                        Arg::with_name("at")
                            .short("a")
                            .long("at")
                            .takes_value(true)
                            .required(false)
                            .help("@someone (Bob,Alice,...)"),
                    ),
            )
    }
}
