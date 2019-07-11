// order while initializing is not important (key value pairs)

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("someemail@provider.com"),
//         username: String::from("MyUsername"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = build_user(String::from("mnoseda@bluewin.ch"), String::from("Mario Noseda"));

//     let user3 = build_user_shorthand(String::from("celina.steffani@bluewin.ch"), String::from("Celina Steffani"));

//     println!("User1:");
//     println!("{}", user1.username);
//     println!("{}", user1.email);
//     println!();

//     println!("User2:");
//     println!("{}", user2.username);
//     println!("{}", user2.email);
//     println!();

//     println!("User3:");
//     println!("{}", user3.username);
//     println!("{}", user3.email);
//     println!();
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn build_user_shorthand(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }


///////////////////////////////////////////////////////////////////////////////////

// using struct to initialize left over fields

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someemail@provider.com"),
        username: String::from("MyUsername"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("mnoseda@bluewin.ch"),
        username: String::from("Mario Noseda"),
        ..user1
    };

    println!("User1:");
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    println!();

    println!("User2:");
    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.active);
    println!("{}", user2.sign_in_count);
    println!();

}
