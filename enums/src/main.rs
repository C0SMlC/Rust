enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//Option

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // println!("{:?} {}", home.kind, home.address);
    // println!("{:?} {}", loopback.kind, loopback.address);

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_numberm = None;

    println!("{:?} {:?}", some_number, some_string);
}
