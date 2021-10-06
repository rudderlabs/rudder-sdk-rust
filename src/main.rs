use rudderanalytics::client::Client;
use rudderanalytics::http::HttpClient;
use rudderanalytics::message::Message;
use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use std::io;

fn main() -> Result<(), Error> {
    // println!("Printing debug info: step-1");

    let matches = App::new("Rudderanalytics")
        .version("0.1")
        .about("Sends analytics events to Rudderstack")
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
                .help("Scheme and host to send to")
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

    let client = HttpClient::new(
        reqwest::Client::new(),
        matches.value_of("data-plane-url").unwrap().to_owned(),
    );

    let message = match matches.subcommand_name() {
        Some("identify") => Message::Identify(serde_json::from_reader(io::stdin())?),
        Some("track") => Message::Track(serde_json::from_reader(io::stdin())?),
        Some("page") => Message::Page(serde_json::from_reader(io::stdin())?),
        Some("screen") => Message::Screen(serde_json::from_reader(io::stdin())?),
        Some("group") => Message::Group(serde_json::from_reader(io::stdin())?),
        Some("alias") => Message::Alias(serde_json::from_reader(io::stdin())?),
        Some(_) => panic!("unknown message type"),
        None => panic!("subcommand is required"),
    };
    // println!("Printing debug info: step-1 {}",message);

    client.send(matches.value_of("write-key").unwrap(), &message)?;
    Ok(())
}
