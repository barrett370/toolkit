use ipnet::Ipv4Subnets;
use toolkit::lib::foreach_input;

fn process(string: String) {
        let mut addrs = string.split('-');
        let start = addrs.next().unwrap();
        let end = addrs.next().unwrap();

        let subnets = Ipv4Subnets::new(
            start.parse().unwrap(),
            end.parse().unwrap(),
            1,
        );

        for subnet in subnets {
            println!("{}", subnet)
        }
    }


fn main() {
    foreach_input(process);
}
