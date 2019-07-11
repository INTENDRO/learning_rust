// using the index to signal the end of the first word. error prone! 
// (string can be deleted but index would still be valid)

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word will get the value 5

//     println!("index: {}", word);

//     s.clear(); // this empties the String, making it equal to ""

//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }


// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

/////////////////////////////////////////////////////////////////////////////////

// using string slices to return first word

// fn main() {
//     let s = String::from("helloworld!");
    
//     let word = first_word(&s);

//     println!("first word: {}", word);
// }


// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }



/////////////////////////////////////////////////////////////////////////////////

// using string slices to return first word (already using slice as parameter)

// fn main() {
//     let my_string = String::from("hello world");

//     // first_word works on slices of `String`s
//     let word = first_word(&my_string[..]);
//     println!("first word: {}", word);

//     let my_string_literal = "hello world";

//     // first_word works on slices of string literals
//     let word = first_word(&my_string_literal[..]);
//     println!("first word: {}", word);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
//     println!("first word: {}", word);

// }


// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }


/////////////////////////////////////////////////////////////////////////////////

// other slices


fn main() {
    let a = [10,20,30,40,50];
    let sla = &a[1..3];

    println!("a: {:?}",a);
    println!("sla: {:?}", sla);
}