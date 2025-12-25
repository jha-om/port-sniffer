// v2 version of port sniffer
use std::{
    env,
    io::{self, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{Sender, channel},
    thread::{self, JoinHandle},
    time::Duration,
};

const MAX: u16 = 65535;
const DEFAULT_THREADS: u16 = 4;
const TIMEOUT_MS: u64 = 200;

struct Argument {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Argument {
    fn new(args: &[String]) -> Result<Argument, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Argument {
                flag: String::from(""),
                ipaddr,
                threads: DEFAULT_THREADS,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") {
                if flag.len() == 2 {
                    println!(
                        "Usage: -j to select how many threads you want \n
                    -h or -help to show this help message"
                    );
                    return Err("help");
                } else {
                    return Err("too many arguments");
                }
            } else if flag.contains("-j") {
                if args.len() != 4 {
                    return Err("invalid syntax: expected -j <threads> <ipaddr>");
                }
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR, must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid thread number provided"),
                };
                return Ok(Argument {
                    flag,
                    ipaddr,
                    threads,
                });
            } else {
                return Err("invalid sytax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port;

    while port <= MAX {
        let socket = SocketAddr::new(addr, port);
        match TcpStream::connect_timeout(&socket, Duration::from_millis(TIMEOUT_MS)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if let Some(next_port) = port.checked_add(num_threads) {
            port = next_port;
        } else {
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let argument = Argument::new(&args).unwrap_or_else(|err| {
        if err.contains("-help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = argument.threads;
    let ipaddrs = argument.ipaddr;

    let (tx, rx) = channel();

    let handles: Vec<JoinHandle<()>> = (0..num_threads)
        .map(|i| {
            let tx = tx.clone();

            thread::spawn(move || {
                scan(tx, i, ipaddrs, num_threads);
            })
        })
        .collect();

    drop(tx);

    let mut out = vec![];
    for r in rx {
        out.push(r);
    }

    println!("\n\nWaiting for all the threads to complete...");
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Thread {} panicked: {:?}", i, e);
            }
        }
    }
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
