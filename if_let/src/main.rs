// fn main() {
//     let some_u8_value = Some(0u8);

//     match some_u8_value {
//         Some(3) => println!("three!"),
//         _ => (),
//     }
// }


/////////////////////////////////////////////////////////////////

// example from above using "if let" syntax

// fn main() {
//     let some_u8_value = Some(0u8);

//     if let Some(3) = some_u8_value {
//         println!("three!");
//     }
// }

/////////////////////////////////////////////////////////////////

// example from above using "if let" syntax but in reverse order
// compiles, but does NOT work correctly!

// fn main() {
//     let some_u8_value = Some(0u8);

//     if let some_u8_value = Some(3) {
//         println!("three!");
//     }
// }

/////////////////////////////////////////////////////////////////

// coin example from match_patterns project

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

// fn main() {
//     let coin = Coin::Quarter(UsState::Alaska);
//     // let coin = Coin::Penny;
//     let mut count = 0;
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count += 1,
//     }
//     println!("count: {}", count);
// }


/////////////////////////////////////////////////////////////////

// coin example from match_patterns project using if let

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //snip
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    // let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);
}
