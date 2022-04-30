use std::str::FromStr;
use ipnet::IpNet;

fn main() {
    let cidr = std::env::args().nth(1).expect("argument 1 must be a valid IPv4/IPv6 CIDR");
    let net = IpNet::from_str(&cidr).expect("argument 1 was not a valid IPv4/IPv6 CIDR");
    let subnets = net.subnets(net.max_prefix_len()).unwrap();
    let (_, min) = subnets.enumerate().min().unwrap();
    let (_, max) = subnets.enumerate().max().unwrap();
    println!("{}-{}", min.addr(), max.addr());
}
