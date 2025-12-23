use std::{
    env,
    io::{self, Write},
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{Sender, channel},
    thread,
};

const MAX: u16 = 65535;

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
        let flag = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Argument {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            if flag.contains("-h") || flag.contains("-help") && flag.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want \n
                    -h or -help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
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

fn scan(tx: Sender<u16>, start_port: u16, ipaddr: IpAddr, num_threads: u16) {
    let mut port = start_port;

    loop {
        match TcpStream::connect((ipaddr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
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

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, ipaddrs, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for r in rx {
        out.push(r);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
