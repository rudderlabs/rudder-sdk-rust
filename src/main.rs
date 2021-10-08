use rudderanalytics::client::RudderAnalytics;
use rudderanalytics::message::Message;
use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use std::io;

fn main() -> Result<(), Error>{
    println!("Printing debug info: step-1");

    let matches = App::new("Rudderanalytics")
        .version("0.1")
        .about("Sends analytics events to RudderStack")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("write-key")
                .help("Write key to send message with")
                .takes_value(true)
                .short("w")
                .long("write-key")
                .required(true),
        )
        .arg(
            Arg::with_name("data-plane-url")
                .help("Base url to send to your data")
                .takes_value(true)
                .short("d")
                .long("data-plane-url")
                .required(true),
        )
        .subcommand(SubCommand::with_name("identify").about("Send an identify event"))
        .subcommand(SubCommand::with_name("track").about("Send a track event"))
        .subcommand(SubCommand::with_name("page").about("Send a page event"))
        .subcommand(SubCommand::with_name("screen").about("Send a screen event"))
        .subcommand(SubCommand::with_name("group").about("Send a group event"))
        .subcommand(SubCommand::with_name("alias").about("Send an alias event"))
        .get_matches();

    let write_key = matches.value_of("write-key").unwrap().to_owned();
    let data_plane_url = matches.value_of("data-plane-url").unwrap().to_owned();

    println!("Supplied CLI args:-");
    println!("write-key: {}", write_key);
    println!("data-plane-url: {}", data_plane_url);
    
    let rudderanalytics = RudderAnalytics::load(write_key,data_plane_url);

    let message = match matches.subcommand_name() {
        Some("identify") => Message::Identify(rudderanalytics::message::Identify {user_id:Some("foo".to_string()),..Default::default()}),
        Some("track") => {
            let mut cmd_ln_inp = String::new();
            io::stdin().read_line(&mut cmd_ln_inp).expect("Failed To read Input");
            let clean_str = cmd_ln_inp.trim();
            println!("Input JSON string: {}", clean_str);
            Message::Track(serde_json::from_str(clean_str)?)
        },
        Some("page") => Message::Page(serde_json::from_reader(io::stdin())?),
        Some("screen") => Message::Screen(serde_json::from_reader(io::stdin())?),
        Some("group") => Message::Group(serde_json::from_reader(io::stdin())?),
        Some("alias") => Message::Alias(serde_json::from_reader(io::stdin())?),
        Some(_) => panic!("unknown message type"),
        None => panic!("subcommand is required"),
    };

    rudderanalytics.send(&message)
}
