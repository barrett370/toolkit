use std::str::FromStr;

use ipnet::IpNet;
use toolkit::lib::foreach_input;

fn process(string: String) {
    let net = IpNet::from_str(&string).expect("input was not a valid IPv4/IPv6 CIDR");
    let subnets = net.subnets(net.max_prefix_len()).unwrap();
    let (_, min) = subnets.enumerate().min().unwrap();
    let (_, max) = subnets.enumerate().max().unwrap();
    println!("{}-{}", min.addr(), max.addr());
}

fn main() {
    foreach_input(process)
}
