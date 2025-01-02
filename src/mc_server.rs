use mcping::get_status;
use serde_json::{ Value, json };
use std::time::Duration;

pub fn log(server: Value, ip: String) {
    if server["success"] == true {
        println!("------------------------------------------");
        println!("IP: {}\nVERSION: {}\nDESCRIPTION: {}\nPLAYERS: {}/{}\nLATENCY: {}ms", ip, server["version"], server["description"], server["players"], server["players_max"], server["latency"]);
    }
}

pub fn ping(server_address: &str) -> Value {
    let status_getter = get_status(server_address, Duration::from_secs(2));
    let (latency, status) = match status_getter {
        Ok(data) => data,
        Err(e) => return json!({ "success": false, "error": e.to_string() }),
    };
    return json!({
        "success":      true,
        "latency":      latency,
        "version":      status.version.name,
        "description":  status.description.text(),
        "players":      status.players.online,
        "players_max":  status.players.max
    });
}