use rudderanalytics::methods::Methods;
use rudderanalytics::rudder::Rudderelement;
/// use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;



fn main() -> Result<(), Error>{
    // println!("Printing debug info: step-1");

    // let matches = App::new("Rudderanalytics")
    //     .version("0.1")
    //     .about("Sends analytics events to Rudderstack")
    //     .setting(AppSettings::ColoredHelp)
    //     .arg(
    //         Arg::with_name("write-key")
    //             .help("Write key to send message with")
    //             .default_value("1xXRrCtFweOR54PIyMRCLQfFjBN")
    //             .short("w")
    //             .long("write-key")
    //     )
    //     .arg(
    //         Arg::with_name("data-plane-url")
    //             .help("Scheme and host to send to")
    //             .default_value("http://localhost:8080")
    //             .short("d")
    //             .long("data-plane-url")
    //     )
    //     .subcommand(SubCommand::with_name("identify").about("Send an identify event"))
    //     .subcommand(SubCommand::with_name("track").about("Send a track event"))
    //     .subcommand(SubCommand::with_name("page").about("Send a page event"))
    //     .subcommand(SubCommand::with_name("screen").about("Send a screen event"))
    //     .subcommand(SubCommand::with_name("group").about("Send a group event"))
    //     .subcommand(SubCommand::with_name("alias").about("Send an alias event"))
    //     .get_matches();
    
    let write_key = String::from("WRITE-KEY");
    let data_plane_url = String::from("DATA-PLANE-URL");
    
    let rudderanalytics = Rudderelement::load(write_key,data_plane_url);
    let msg = rudderanalytics::message::Identify {user_id:Some("foo".to_string()),..Default::default()};
    rudderanalytics.identify(&msg)

    // let client = HttpClient::new(
    // reqwest::Client::new(),
    // data_plane_url
    // );
    
    // let _identify = |_msg:rudderanalytics::message::Identify| -> Result<(), Error>{
    //     let message = Message::Identify(_msg);        
    //     client.send(&write_key, &message)?;
    //     Ok(())
    // };

    
    
    // _identify(rudderanalytics::message::Identify {user_id:Some("foo".to_string()),..Default::default()})

    // let message = match api_type {
    //     Some("identify") => Message::Identify(rudderanalytics::message::Identify {user_id:Some("foo".to_string()),..Default::default()}),
    //     Some("track") => Message::Track(serde_json::from_reader(io::stdin())?),
    //     Some("page") => Message::Page(serde_json::from_reader(io::stdin())?),
    //     Some("screen") => Message::Screen(serde_json::from_reader(io::stdin())?),
    //     Some("group") => Message::Group(serde_json::from_reader(io::stdin())?),
    //     Some("alias") => Message::Alias(serde_json::from_reader(io::stdin())?),
    //     Some(_) => panic!("unknown message type"),
    //     None => panic!("subcommand is required"),
    // };
    // println!("Printing debug info: step-1 {:#?}",message);

    // client.send(&write_key, &message)?;
    // Ok(())
}
