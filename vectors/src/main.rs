// writing to vectors

// fn main() {
//     let v1: Vec<i32> = Vec::new();
//     let v2 = vec![1,2,3];
//     let mut vm = Vec::new();

//     vm.push(5);
//     vm.push(6);
//     vm.push(7);
//     vm.push(8);
// }

/////////////////////////////////////////

// reading from vectors

// fn main() {
//     let v = vec![1,2,3,4,5];

//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);
    
//     match v.get(2) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element!"),
//     }
// }

/////////////////////////////////////////

// accessing data outside boundaries

// fn main() {
//     let v = vec![1,2,3,4,5];

//     let does_not_exist = &v[100]; // this will panic the program
//     let does_not_exist = v.get(100); // this will return a None variant of Option<T>
// }

/////////////////////////////////////////

// mixing immutable and mutable references

// fn main() {
//     let v = vec![1,2,3,4,5];

//     let first = &v[0]; // immutable reference to the first element
    
//     v.push(6);
// }

/////////////////////////////////////////

// iterating over a vector

// fn main() {
//     let v = vec![100, 32, 57];

//     for i in &v {
//         println!("{}", i);
//     }
// }

/////////////////////////////////////////

// iterating over a mutable vector

// fn main() {
//     let mut v = vec![100, 32, 57];

//     for i in &mut v {
//         *i += 50;
//     }

//     for i in &mut v {
//         println!("{}", i);
//     }
// }


/////////////////////////////////////////

// elements with different types

// fn main() {
//     let v = vec![100, "hi", 57.1];

// }

/////////////////////////////////////////

// using an enum to store elements with different types

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(12.34),
        ];
}


