// v3 version of port sniffer
use bpaf::Bpaf;
use std::{
    io::{self, Write},
    net::{IpAddr, Ipv4Addr},
    sync::mpsc::{Sender, channel},
};
use tokio::net::TcpStream;
use tokio::task;

const MAX: u16 = 65535;

const IP_FALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]

pub struct Argument {
    #[bpaf(long, short, fallback(IP_FALLBACK))]
    /// the address that you want to sniff, must be a valid ipv4 address falls back to localhost(127.0.0.1);
    pub address: IpAddr,

    #[bpaf(
        long("start"),
        short('s'),
        fallback(1u16),
        guard(start_port_guard, "Must be a valid non-negative port")
    )]
    /// the start port for the sniffer(must be greater than 0);
    // bpaf provides a method to create a function to prevent something from happening;
    pub start_port: u16,

    #[bpaf(
        long("end"),
        short('e'),
        fallback(MAX),
        guard(end_port_guard, "Must be less than or equal to 65535")
    )]
    /// the end port for the sniffer(must be less than or equal 65535);
    pub end_port: u16,
}

fn start_port_guard(port: &u16) -> bool {
    *port > 0
}

fn end_port_guard(port: &u16) -> bool {
    *port <= MAX
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}: {}", addr, port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let opts: Argument = argument().run();

    let (tx, rx) = channel();

    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        task::spawn(async move {
            scan(tx, i, opts.address).await;
        });
    }

    drop(tx);

    let mut out = vec![];
    for r in rx {
        out.push(r);
    }

    println!("\n\nWaiting for all the threads to complete...");
    println!("\nScan complete!\n");
    out.sort();
    if out.is_empty() {
        println!("No open ports found");
    } else {
        println!("Open ports: ");
        for v in out {
            println!("{} is open", v);
        }
    }
}
