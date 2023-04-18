use std::str::FromStr;

use ipnet::IpNet;
use toolkit::lib::foreach_input;

fn process(string: String) {
    let net = IpNet::from_str(&string).expect("input was not a valid IPv4/IPv6 CIDR");
    let subnets = net.subnets(net.max_prefix_len()).unwrap();
    for (_, subnet) in subnets.enumerate() {
        println!("{}", subnet.addr())
    }
}

fn main() {
    foreach_input(process);
}
