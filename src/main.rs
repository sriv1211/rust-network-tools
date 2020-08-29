use pnet::datalink::{self, NetworkInterface};
use pnet::packet::arp::ArpOperations;
use structopt::StructOpt;

use network_tools::{arp, routes};

#[derive(Debug, StructOpt)]
struct Args {
    /// Network tool to run
    #[structopt(subcommand)]
    command: Command,
    /// Interface name to bind to
    #[structopt(short, long, global = true)]
    interface: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
    /// List available Interfaces
    Interfaces,
    /// List available Routes (or route for given destination IpAddr)
    Route { addr: Option<std::net::IpAddr> },
    /// Monitor for ARP Request/Replies
    MonitorArp {
        #[structopt(short, long, default_value = "10")]
        /// Monitor until up to this many ARP packets seen
        count: usize,
    },
    /// Send an ARP request for a given IP Address
    RequestArp { address: std::net::Ipv4Addr },
}

fn main() {
    let args = Args::from_args();
    let interfaces: Vec<NetworkInterface> = datalink::interfaces().into_iter().collect();

    match args.command {
        Command::Interfaces => {
            let mut width = 0usize;
            for iface in &interfaces {
                width = std::cmp::max(width, iface.name.len());
            }
            for iface in &interfaces {
                let addrs: Vec<_> = iface.ips.iter().map(|ip| format!("{}", ip.ip())).collect();
                println!("{:w$} [{}]", iface.name, addrs.join(", "), w = width + 2);
            }
        }
        Command::Route { addr } => {
            let routes = if let Some(iface_name) = &args.interface {
                let interface = get_interface(interfaces, Some(iface_name));
                routes::Routes::with_interfaces(vec![interface]).unwrap()
            } else {
                routes::Routes::new().unwrap()
            };
            if let Some(dest) = addr {
                if let Some(next_hop) = routes.lookup_gateway(dest) {
                    println!(
                        "{} via {} [{}]",
                        dest,
                        next_hop.gateway,
                        next_hop
                            .mac
                            .map(|mac| mac.to_string())
                            .unwrap_or("--".to_owned())
                    );
                }
            } else {
                for route in routes.routes() {
                    println!(
                        "{}/{} via {} [{}]",
                        route.0,
                        route.1,
                        route.2,
                        route
                            .3
                            .map(|mac| mac.to_string())
                            .unwrap_or("--".to_owned())
                    );
                }
            }
        }
        Command::MonitorArp { count } => {
            let interface = get_interface(interfaces, args.interface.as_ref());
            let mut monitor = arp::ArpMonitor::new(&interface).unwrap();
            let mut limit = count;
            loop {
                for arp in &mut monitor {
                    match arp.get_operation() {
                        ArpOperations::Request => eprintln!(
                            "Request: {} is asking about {}",
                            arp.get_sender_proto_addr(),
                            arp.get_target_proto_addr()
                        ),
                        ArpOperations::Reply => eprintln!(
                            "*Reply: {} has address {}",
                            arp.get_sender_hw_addr(),
                            arp.get_sender_proto_addr()
                        ),
                        _ => return,
                    }

                    limit -= 1;
                }
                if limit == 0 {
                    break;
                }
            }
        }
        Command::RequestArp { address } => {
            let interface = get_interface(interfaces, args.interface.as_ref());
            let requester = arp::ArpRequest::new(&interface, address);
            let hw_addr = requester.request().unwrap();
            eprintln!("{} has MAC Address {}", address, hw_addr);
        }
    }
}

fn get_interface<'a>(
    interfaces: Vec<NetworkInterface>,
    iface_name: Option<&String>,
) -> NetworkInterface {
    if let Some(iface_name) = iface_name {
        interfaces
            .into_iter()
            .filter(|iface| &iface.name == iface_name)
            .next()
            .unwrap_or_else(|| {
                eprintln!("'{}' is not a valid interface", iface_name,);
                std::process::exit(1);
            })
    } else {
        interfaces
            .first()
            .map(|i| {
                eprintln!("Using interface: {}", i.name);
                i.clone()
            })
            .expect("No interfaces found")
    }
}
