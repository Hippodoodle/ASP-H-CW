use std::env;
use std::io::Write;
use std::net::{ToSocketAddrs, SocketAddr, TcpStream};


fn dnslookup(domains: &Vec<String>) -> Result<(), std::io::Error> {
    for domain in domains.iter() {
        let domain_with_port = format!("{domain}:80");

        let addrs = domain_with_port.to_socket_addrs()?;

        for addr in addrs.into_iter() {
            match addr {
                SocketAddr::V4(addr) => {
                    println!("{domain} IPv4 {}", addr.ip())
                }
                SocketAddr::V6(addr) => {
                    println!("{domain} IPv6 {}", addr.ip())
                }
            }
        }
    }

    Ok(())
}


fn seqcon(domain: &String) -> Result<(), std::io::Error> {
    let domain_with_port = format!("{domain}:80");

    let addrs = domain_with_port.to_socket_addrs()?;

    for addr in addrs.into_iter() {
        if let Ok(mut stream) = TcpStream::connect(addr) {
            println!("Connected on {}", addr.ip());
            stream.write(format!("GET / HTTP/1.1\r\nHost: {domain}\r\n\r\n").as_bytes())?;
            return Ok(())
        }
    }

    println!("Couldn't connect to server...");
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    dnslookup(&args).expect("Error: Invalid socket address");

    for domain in args.iter() {
        seqcon(domain).expect("Error: Invalid socket address");
    }
}
