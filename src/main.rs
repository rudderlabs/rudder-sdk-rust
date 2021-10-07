use rudderanalytics::client::Client;
use rudderanalytics::http::HttpClient;
use rudderanalytics::message::Message;
use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use std::io;

fn main() -> Result<(), Error>{
    println!("Printing debug info: step-1");

    let matches = App::new("Rudderanalytics")
        .version("0.1")
        .about("Sends analytics events to Rudderstack")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("write-key")
                .help("Write key to send message with")
                .default_value("1xXRrCtFweOR54PIyMRCLQfFjBN")
                .short("w")
                .long("write-key")
        )
        .arg(
            Arg::with_name("data-plane-url")
                .help("Scheme and host to send to")
                .default_value("http://localhost:8080")
                .short("d")
                .long("data-plane-url")
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

    let client = HttpClient::new(
    reqwest::Client::new(),
    data_plane_url
    );

    let message = match matches.subcommand_name() {
        Some("identify") => Message::Identify(rudderanalytics::message::Identify {user_id:Some("asdf".to_string()),..Default::default()}),
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

    println!("Printing debug info: step-2 {:#?}",message);

    client.send(&write_key, &message)?;
    Ok(())
}
