// create empty hashmap and add values

// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
// }

//////////////////////////////////////////////////

// constructing hashmap with vector of key-value tuples

// use std::collections::HashMap;

// fn main() {
//     let teams = vec![String::from("Blue"), String::from("Yellow")];
//     let initial_scores = vec![10, 50];
//     let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

//     println!("Blue: {:?}", scores.get(&String::from("Blue")));
//     println!("Yellow: {:?}", scores.get(&String::from("Yellow")));
//     println!("Red: {:?}", scores.get(&String::from("Red")));

// }

//////////////////////////////////////////////////

// values without copy trait get moved into the hashmap

// use std::collections::HashMap;

// fn main() {
//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");
//     let mut map = HashMap::new();

//     map.insert(field_name, field_value); // field_name and field_value get moved

//     println!("field_name: {:?}", field_name); // data has been moved and cannot be used anymore. hashmap is the new owner
// }

//////////////////////////////////////////////////

// using the get method to retrieve data from the hashmap. return an Option<T>

// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);

//     println!("{:?}", score);
// }


//////////////////////////////////////////////////

// iterating over hashmap using for loop

// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// }

//////////////////////////////////////////////////

// overwriting an existing value

// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 50);

//     println!("{:?}", scores);
// }

//////////////////////////////////////////////////

// insert if key has no value

// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);

//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);

//     println!("{:?}", scores);
// }

//////////////////////////////////////////////////

// updating based on old value

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
