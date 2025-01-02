use mcping::get_status;
use serde_json::{ Value, json };
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

pub fn log(server: Value, ip: String) {
    if server["success"] == true {
        println!("------------------------------------------");
        println!("IP: {}\nVERSION: {}\nDESCRIPTION: {}\nPLAYERS: {}/{}\nLATENCY: {}ms", ip, server["version"], server["description"], server["players"], server["players_max"], server["latency"]);
    }
}

pub fn ping(server_address: &str) -> Value {
    let (sender, receiver) = mpsc::channel();
    let address: String = server_address.to_string();
    let _ = thread::spawn(move || {
        if let Err(_) = sender.send(get_status(&address, Duration::from_secs(2))) { // The timeout in the crate doesn't work, I made my own.
            // Channel has been closed, do nothing.
        }
    });
    let status_getter = receiver.recv_timeout(Duration::from_millis(5000));
    let result = match status_getter {
        Ok(Ok((latency, status))) => {
            json!({
                "success":      true,
                "latency":      latency,
                "version":      status.version.name,
                "description":  status.description.text(),
                "players":      status.players.online,
                "players_max":  status.players.max
            })
        }
        Ok(Err(e)) => json!({ "success": false, "error": e.to_string() }),
        Err(e) =>     json!({ "success": false, "error": e.to_string() }),
    };
    return result;
}