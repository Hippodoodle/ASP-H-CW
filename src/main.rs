use std::env;
use std::net::ToSocketAddrs;


fn dnslookup(domains: Vec<String>) {
    for domain in domains {
        let domain_with_port = format!("{domain}:80");
        match domain_with_port.to_socket_addrs() {
            Ok(addrs) => {
                for addr in addrs {
                    match addr {
                        std::net::SocketAddr::V4(addr) => {
                            println!("{} IPv4 {}", domain, addr.ip())
                        }
                        std::net::SocketAddr::V6(addr) => {
                            println!("{} IPv6 {}", domain, addr.ip())
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {} {}", e, domain)
            }
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    dnslookup(args);
}
