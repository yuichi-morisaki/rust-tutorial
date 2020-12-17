fn main() {
    {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        fn route(ip_kind: IpAddrKind) {
            println!("{:?}", ip_kind);
        }

        route(four);
        route(six);

        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        fn route2(ip: IpAddr) {
            println!("{:?}", ip);
        }

        route2(home);
        route2(loopback);
    }

    {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let _home = IpAddr::V4(String::from("127.0.0.1"));
        let _loopback = IpAddr::V6(String::from("::1"));
    }
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }
    {
        struct Ipv4Addr {}
        struct Ipv6Addr {}
        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }
}
