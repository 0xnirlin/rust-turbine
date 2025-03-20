// let's go with the tutorial
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// enums with args
enum IpAddr2 {
    V4(String),
    V6(String),
    V7(u8, u8, u8, u8),
}

// enums with structs
enum IpAddrStruct {
    V4(IpAddr),
    V6(IpAddr).
}



fn main() {
    let home = IpAddr { 
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let my_home = IpAddr2::V4(String::from("127.0.0.1"));
    let loop_back = IpAddr2::V6(String::from("::1"));
    let my_v6 = IpAddr2::V7(10, 12, 12, 14);

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let struct_v4 = IpAddrStruct::V4(home);
    
    // To access properties of a struct inside an enum variant, we need to use pattern matching
    // This is because enums can contain different types of data in each variant
    // The match expression lets us destructure the enum and access the inner data
    let v4 = match struct_v4 {
        // When struct_v4 is the V4 variant, extract the IpAddr struct inside it as 'addr'
        IpAddrStruct::V4(addr) => {
            // Now we can access the struct fields using dot notation
            println!("IPv4 address: {}, kind: {:?}", addr.address, addr.kind);
            addr // Return the IpAddr struct from this match arm
        },
        // We must handle all possible enum variants in a match expression
        IpAddrStruct::V6(addr) => {
            println!("IPv6 address: {}, kind: {:?}", addr.address, addr.kind);
            addr
        },
    };


    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing to {:?}", ip_kind);
}
