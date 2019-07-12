// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     println!("{:?}", four);
// }

///////////////////////////////////////////////////////

// using enums and structs to store ip addresses

// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

//     println!("home: {:#?}", home);
//     println!();
//     println!("loopback: {:#?}", loopback);
// }


///////////////////////////////////////////////////////

// store data in the enum itself

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }


// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));

//     println!("home: {:#?}", home);
//     println!();
//     println!("loopback: {:#?}", loopback);
// }

///////////////////////////////////////////////////////

// implement methods on enum

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn call(&self) {
        println!("Call method: {:#?}", self);
    }
}


fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:#?}", home);
    println!();
    println!("loopback: {:#?}", loopback);
    println!();

    home.call();
}

