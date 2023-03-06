use std::{env, thread};
use std::io::{Write, Read};
use std::net::{ToSocketAddrs, TcpStream};
use std::sync::mpsc;


fn concon(domain: &String) -> Result<(), std::io::Error> {
    let domain_with_port = format!("{domain}:80");

    let addrs = domain_with_port.to_socket_addrs()?;

    // Channel from connection_attempt threads to connected_client thread
    let (ca_to_cc_send, ca_to_cc_recv) = mpsc::channel();

    let connected_client = thread::spawn(move || {
        let mut recv_stream: TcpStream = ca_to_cc_recv.recv().unwrap();
        drop(ca_to_cc_recv);
        println!("Connected client on {}", recv_stream.peer_addr().unwrap());
        let mut buffer = String::new();
        recv_stream.write(format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", 1).as_bytes()).unwrap();
        recv_stream.read_to_string(&mut buffer).unwrap();
        println!("{buffer}");
    });

    let mut main_to_ca_senders = vec![];

    // Channels from main thread to connection_attempt threads
    for i in 0..addrs.len() {
        let (main_to_ca_send, main_to_ca_recv) = mpsc::channel();
        let ccsender = ca_to_cc_send.clone();
        thread::spawn(move || {
            let recv_addr = main_to_ca_recv.recv().unwrap();
            println!("Attempting connection {i} on {recv_addr}");
            if let Ok(stream) = TcpStream::connect(recv_addr) {
                ccsender.send(stream).unwrap();
            }
            println!("{i} done");
        });
        main_to_ca_senders.push(main_to_ca_send);
    }

    for (sender, addr) in main_to_ca_senders.into_iter().zip(addrs.into_iter()) {
        sender.send(addr).expect("Couldn't send to connection attempt thread");
    }

    connected_client.join().unwrap();

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for domain in args.iter() {
        concon(domain).expect("Error: Invalid socket address");
    }
}
