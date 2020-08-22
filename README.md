# Rust Network Tools

A small collection of example network tools based off of similar examples with [Scapy](https://scapy.net/) from this blog series: [Building Network Tools with Scapy](https://thepacketgeek.com/scapy/)

# Included Tools

- `ArpMonitor`
  - View sniffed ARP Requests & Replies on a network interface
- `ArpRequest`
  - Send an ARP request for a given IP and print out the replied MAC address


# Running the CLI

```rust
cargo run -- --help
network-tools 0.1.0

USAGE:
    network-tools [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interface <interface>    Interface name to bind to

SUBCOMMANDS:
    help               Prints this message or the help of the give
    list-interfaces    List available Interfaces
    monitor-arp        Monitor for ARP Request/Replies
    request-arp        Send an ARP request for a given IP Address
```

## Permissions
In order to create a `Channel` for Tx/Rx on an interface, you'll need sudo permissions:

```rust
cargo build
sudo ./target/debug/build monitor-arp -i eth0
```