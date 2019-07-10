// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, nothing happens.

//////////////////////////////////////////////////////////

// attempting to modify a borrowed value

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

//////////////////////////////////////////////////////////

// using a mutable reference


// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);

//     println!("{}", s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }



//////////////////////////////////////////////////////////

// multiple mutable references

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }


//////////////////////////////////////////////////////////

// multiple mutable references with different scopes

// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;

//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;
// }

//////////////////////////////////////////////////////////

// combining mutable and immutable references

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);
// }

//////////////////////////////////////////////////////////

// creating mutable ref after immutable refs went out of scope (ok)

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // r1 and r2 are no longer used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }

//////////////////////////////////////////////////////////

// creating dangling pointer

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

//////////////////////////////////////////////////////////

// solution to code above


fn main() {
    let reference_to_nothing = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

