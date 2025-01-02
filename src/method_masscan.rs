#[cfg(not(windows))]
use std::{
    thread::{
        sleep,
        spawn
    },
    str,
    time::Duration,
    process::Command,
    fs::{
        write,
        read_to_string
    },
    path::Path,
};
#[cfg(not(windows))]
use serde_json::Value;
#[cfg(not(windows))]
#[path = "mc_server.rs"]
mod mc_server;
#[cfg(not(windows))]
use mc_server::{ ping, log };

#[cfg(not(windows))]
fn start_masscan(max_rate: u32) {
    let output = Command::new("masscan")
        .args(["-p25565", "0.0.0.0/0", "--max-rate", &max_rate.to_string(), "--excludefile", "masscan_exclude.conf", "-oL", "masscan_mcfinder.txt"])
        .output()
        .expect("Failed to execute user process");
    
    println!("{:?}", str::from_utf8(&output.stderr).unwrap());
}

#[cfg(not(windows))]
fn check(entry: &str) {
    // println!("Checking: {}", entry);
    let server: Value = ping(&entry);
    log(server, entry.to_string());
}

#[cfg(not(windows))]
pub fn run(scanrate: u32) {
    //---   Start Masscan   ---//
    const MASSCAN_EXCLUDE_FILE: &[u8; 11575] = include_bytes!("masscan_exclude.conf");
    write("masscan_exclude.conf", MASSCAN_EXCLUDE_FILE).expect("Unable to write file");
    spawn(move|| { start_masscan(scanrate); });
    sleep(Duration::from_millis(2000));

    //---   Check if Masscan is running   ---//
    if !Path::new("masscan_mcfinder.txt").exists() { return println!("Masscan failed to start properly.\nTry running this binary with sudo or as root."); };

    //---   Watch Masscan File   ---//
    let mut scanned: Vec<String> = vec![];
    loop {
        let file: String = read_to_string("masscan_mcfinder.txt").expect("Unable to read file");
        let lines = file.split_whitespace();
        let mut ips: Vec<String> = vec![];
        for entry in lines {
            //---   Parse down to IPs and remove duplicates   ---//
            if entry.split(".").count() < 3 { continue; }; // Only parse IPv4 addresses
            if scanned.contains(&entry.to_string()) { continue; };
            scanned.push(entry.to_string());
            
            //---   Add to check list   ---//
            ips.push(entry.to_string());
        }

        //---   Check items in list   ---//
        for ip in ips {
            spawn(move|| { check(&ip); });
        }

        sleep(Duration::from_millis(5000));
    }

}

#[cfg(windows)]
pub fn run(_scanrate: u32) {
    println!("Masscan mode is not supported on Windows.\nPlease use classic mode or refer to --help for more information.")
}
