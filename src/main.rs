use std::env;
use std::net::ToSocketAddrs;


fn dnslookup(domains: Vec<String>) {

    for mut domain in domains {
        domain.push_str(":80");
        match domain.to_socket_addrs() {
            Ok(addrs) => {
                for addr in addrs {
                    println!("{} {}", domain, addr)
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
