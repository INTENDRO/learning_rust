
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     println!("{}", value_in_cents(Coin::Penny));
// }

/////////////////////////////////////////////////////////

// patterns that bind to values

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     //snip
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}", state);
//             25
//         },
//     }
// }

// fn main() {
//     println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
// }


///////////////////////////////////////////////////////////////

// matching with Option<T>

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i+1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);


//     println!("five: {:?}", five);
//     println!("six: {:?}", six);
//     println!("none: {:?}", none);
// }

///////////////////////////////////////////////////////////////

// matches are exhaustive (error if not all cases are handled)

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i+1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);


//     println!("five: {:?}", five);
//     println!("six: {:?}", six);
//     println!("none: {:?}", none);
// }

///////////////////////////////////////////////////////////////

// machting using a placeholder

fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}