fn main() {
    let home: IpAdd = IpAdd {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback: IpAdd = IpAdd {
        kind: IpAddressKind::V6,
        address: String::from("::1")
    };

    println!("home: {:?}, other: {:?}", home, loopback);
}

#[derive(Debug)]
enum IpAddressKind {
    V4, V6,
}

#[derive(Debug)]
struct IpAdd {
    kind: IpAddressKind,
    address: String
}