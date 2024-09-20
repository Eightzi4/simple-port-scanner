use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc, io::{stdin, Read}, env::args, process::exit};
use tokio::{sync::Mutex, net::TcpStream, time::{timeout, Duration}};
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};

fn parse_args() -> Result<(u16, u16), String> {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => Ok((1, 65535)),
        3 => {
            let start_port = args[1].parse::<u16>().map_err(|_| "Invalid starting port")?;
            let end_port = args[2].parse::<u16>().map_err(|_| "Invalid ending port")?;
            if start_port <= end_port {
                Ok((start_port, end_port))
            } else {
                Err("Starting port must be less than or equal to ending port".to_string())
            }
        },
        _ => Err("Usage: simple-port-scanner [start_port end_port]".to_string()),
    }
}

#[tokio::main]
async fn main() {
    let (start_port, end_port) = match parse_args() {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Usage: simple-port-scanner [start_port end_port]");
            eprintln!("If no ports are specified, all ports (1-65535) will be scanned.");
            exit(1);
        }
    };

    let start = std::time::Instant::now();
    let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let ports = end_port - start_port + 1;

    println!("Scanning ports {} to {} on {}...", start_port, end_port, addr);

    let pb = ProgressBar::new(ports as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let pb = Arc::new(Mutex::new(pb));

    let open_ports = Arc::new(Mutex::new(Vec::new()));

    stream::iter(start_port..=end_port)
        .map(|port| {
            let pb = Arc::clone(&pb);
            let open_ports = Arc::clone(&open_ports);
            async move {
                let sock_addr = SocketAddr::new(addr, port);
                if let Ok(Ok(_)) = timeout(Duration::from_millis(1), TcpStream::connect(&sock_addr)).await {
                    open_ports.lock().await.push(port);
                }
                pb.lock().await.inc(1);
            }
        })
        .buffer_unordered(1024)
        .collect::<Vec<()>>()
        .await;

    pb.lock().await.finish_with_message("Scan complete");

    let open_ports = open_ports.lock().await;
    if open_ports.is_empty() {
        println!("No open ports found");
    } else {
        println!("Open ports:");
        for port in open_ports.iter() {
            println!("{}", port);
        }
    }

    println!("Scanned {} ports in {:?}", ports, start.elapsed());
    println!("Press enter to exit");
    stdin().read(&mut [0]).unwrap();
}