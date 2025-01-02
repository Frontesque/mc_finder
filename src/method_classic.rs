use std::time::Duration;
use serde_json::Value;
use std::net::{ TcpStream, SocketAddr, IpAddr, Ipv4Addr };
use rand::Rng;
use std::thread;

#[path = "mc_server.rs"]
mod mc_server;
use mc_server::{ ping, log };

pub fn scanip(addr: SocketAddr) -> bool {
    match TcpStream::connect_timeout(&addr, Duration::from_millis(50)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn gen_ipv4() -> SocketAddr {
    let mut rng = rand::thread_rng(); // Open RNG Thread
    let raw_ip = Ipv4Addr::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)); // Generate Random IP
    let ip = IpAddr::V4(raw_ip); // Convert to a rust "IPV4" datatype
    let socket = SocketAddr::new(ip, 25565); // Convert to a rust socket datatype
    return socket;
}

fn scan() {
    loop {
        let ipv4 = gen_ipv4();
        let is_port_open = scanip(ipv4);
        if is_port_open == true {
            // println!("Found   [{}]", ipv4);
            let ipv4_string: String = ipv4.to_string();
            let ipv4_str: &str = &ipv4_string[..];
            let server: Value = ping(&ipv4_str);
            log(server, ipv4_string);
        }
    }
}

pub fn run(threads: u16) {
    for i in 0..threads {
        println!("Started thread: {}", i);
        thread::spawn(move|| { scan(); });
    }
    scan();
}
