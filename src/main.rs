mod config_file;
mod surfline_requests;
mod spots;
mod tides;

use crate::config_file::{ UserConfig, read_config_file };
use crate::surfline_requests::{ get_session_token, get_tide_data };

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    spot: String
}


fn main() {
    let args = Args::parse();
    println!("Looking up surf report for {}", args.spot);

    // Read argfile
    let config: UserConfig = match read_config_file("./config/config.txt") {
        None => panic!("Missing or invalid config file"),
        Some(user) => user
    };

    let session = match get_session_token(&config) {
        Ok(session_token) => session_token,
        Err(error) => panic!("Error generating session token: {:?}", error)
    };

    let spot  = match spots::get_spot(&args.spot) {
        Some(res) => res,
        None => panic!("Inavlid!")
    };

    println!("Found SPOT: {:?}", spot);
    get_tide_data(&session, &spot.id);
}
