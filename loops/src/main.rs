// fn main() {
//     loop{
//         println!("again!");
//     }
// }


// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("result = {}", result);
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}", number);
//         number -= 1;
//     }

//     println!("LIFTOFF!!");
// }

// fn main() {
//     let arr = [10,20,30,40,50];
//     let mut ind = 0;

//     println!("interating over array using while-loop");

//     while ind < 5 {
//         println!("{}", arr[ind]);

//         ind += 1;
//     }

//     println!("done");
// }

// fn main() {
//     let arr = [10,20,30,40,50];

//     println!("interating over array using for-loop");

//     for element in arr.iter() {
//         println!("{}", element);
//     }

//     println!("done");
// }

fn main() {

    println!("interating using for-loop and range");

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("LIFTOFF!!");
}
