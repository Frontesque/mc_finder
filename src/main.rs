use std::env;
mod method_classic;
mod method_masscan;
mod mc_server;
use serde_json::Value;

fn help() {
    const HELP_STRING: &str = "------------------------------------------
classic <number>
    Run the program in classic mode (randomly generated IP addresses) with x number of threads.
    NOTICE: THIS METHOD IS INEFFICIENT. IT IS INCLUDED FOR PLATFORM COMPATABILITY AND SO THE BINARY CAN BE RUN AS STANDALONE.

masscan <packets_per_second>
    Run the program in masscan mode
    Masscan mode only works on linux and requires masscan to be installed.
    For <packets_per_second>, rate of 1000 is reccommended to start with. It may be increased if your router permits it.

check <ipv4 address>
    Ping a single ip address for a minecraft server running on port 25565

help
    Shows this message.
------------------------------------------";
    println!("{} {} \n{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), HELP_STRING);
}

fn main() {
    let mut args: Vec<String> = env::args().collect(); // Gather args
    if args.len() < 2 { args.push("invalid".to_string()); } // Ensure an arg always exists to prevent errors

    let command: String = args[1].to_lowercase();
    match command.as_str() {
        "classic" => {
            if args.len() < 3 { args.push("invalid".to_string()); }
            let threads_str: String = args[2].clone();
            let threads_int: u16 = threads_str.parse::<u16>().unwrap_or(0);
            if threads_int == 0 { return println!("Please provide a thread count.\nSee --help for more information") };
            method_classic::run(threads_int);
        },

        "masscan" => {
            if args.len() < 3 { args.push("invalid".to_string()); }
            let pps_str: String = args[2].clone();
            let pps_int: u32 = pps_str.parse::<u32>().unwrap_or(0);
            if pps_int == 0 { return println!("Please provide a packet rate.\nSee --help for more information") };
            method_masscan::run(pps_int);
        },

        "check" => {
            if args.len() < 3 { args.push("invalid".to_string()); }
            let ip_str: String = args[2].clone();
            let server_ping: Value = mc_server::ping(&ip_str);
            if server_ping["success"] == true {
                mc_server::log(server_ping, ip_str);
            } else {
                println!("{}", server_ping["error"]);
            }
        },

        "help" => help(),
        "--help" => help(),
        _ => return eprintln!("Usage: {} <method>\nSee --help for more information", args[0]),
    }
}