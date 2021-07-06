fn main() {
    let x = Ipv4Addr();
    let y = IpAddr::V4(x);

    y.print_type();

    y.print_type_4();

    let y = IpAddr::V6(Ipv6Addr());
    y.print_type_4(); //won't print anything
}

#[derive(Debug)]
struct Ipv4Addr();

impl Ipv4Addr {
    fn print_type(&self) {
        println!("call from Ipv4Addr");
    }
}

#[derive(Debug)]
struct Ipv6Addr();

impl Ipv6Addr {
    fn print_type(&self) {
        println!("call from Ipv6Addr");
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    fn print_type(&self) {
        match self {
            IpAddr::V4(x) => {
                x.print_type();
                println!("type is V4")
            }
            IpAddr::V6(x) => {
                x.print_type();
                println!("type is V6")
            }
        }
    }

    //example with if let
    fn print_type_4(&self) {
        if let IpAddr::V4(x) = self {
            x.print_type();
        }
    }
}
