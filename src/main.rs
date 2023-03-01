use std::{env, thread};
use std::io::{Write, Read};
use std::net::{ToSocketAddrs, SocketAddr, TcpStream};
use std::sync::mpsc;


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
            let mut buffer = Vec::new();
            stream.write(format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", domain).as_bytes())?;
            stream.read_to_end(&mut buffer)?;
            println!("{}", String::from_utf8_lossy(&buffer));
            return Ok(())
        }
    }

    println!("Couldn't connect to server...");
    Ok(())
}


fn concon(domain: &String) -> Result<(), std::io::Error> {
    let domain_with_port = format!("{domain}:80");

    let addrs = domain_with_port.to_socket_addrs()?;

    // Channel from connection_attempt threads to connected_client thread
    let (ca_to_cc_send, ca_to_cc_recv) = mpsc::channel();

    let connected_client = thread::spawn(move || {
        let mut recv_stream: TcpStream = ca_to_cc_recv.recv().unwrap();
        println!("Connected client on {}", recv_stream.peer_addr().unwrap());
        let mut buffer = Vec::new();
        recv_stream.write(format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", 1).as_bytes()).unwrap();
        recv_stream.read_to_end(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
    });

    let mut main_to_ca_senders = Vec::new();

    // Channels from main thread to connection_attempt threads
    /*
    addrs.len() number of channels
    each channel has cloned sender to connected_client
    each channel attempts to connect to addr supplied
    and send the resulting TcpStream to connected_client through the cloned sender
    this is done through a thread for each channel that listens for an addr sent later in a for loop
    */
    for i in 0..addrs.len() {
        let (main_to_ca_send, main_to_ca_recv) = mpsc::channel();
        let ccsender = ca_to_cc_send.clone();
        thread::spawn(move || {
            let recv_addr = main_to_ca_recv.recv().unwrap();
            println!("Attempting connection {} on {}", i, recv_addr);
            if let Ok(mut stream) = TcpStream::connect(recv_addr) {
                println!("{i} sending");
                ccsender.send(stream);
            } else {
                println!("Connection {i} failed");
            }
            println!("{i} done");
        });
        main_to_ca_senders.push(main_to_ca_send);
    }

    for (sender, addr) in main_to_ca_senders.into_iter().zip(addrs.into_iter()) {
        sender.send(addr);
    }

    connected_client.join();

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    dnslookup(&args).expect("Error: Invalid socket address");

    //for domain in args.iter() {
    //    seqcon(domain).expect("Error: Invalid socket address");
    //}

    for domain in args.iter() {
        concon(domain).expect("Error: Invalid socket address");
    }
}
