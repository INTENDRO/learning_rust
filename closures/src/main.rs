// from function to closure

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

//////////////////////////////////////////////////////

// example program. needs to call an expensive function
// either never, once or twice. program uses a closure
// and a casher struct for memoization

// use std::thread;
// use std::time::Duration;

// struct Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             },
//         }
//     }
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let mut expensive_result = Cacher::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });

//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_result.value(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_result.value(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_result.value(intensity)
//             );
//         }
//     }
// }

// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;

//     generate_workout(
//         simulated_user_specified_value,
//         simulated_random_number
//     );
// }

//////////////////////////////////////////////////////

// closures capture the environment that they have
// been called from (variable x can be accessed)

// fn main() {
//     let x = 4;

//     let equal_to_x = |z| z == x;

//     let y = 4;

//     assert!(equal_to_x(y));
// }

//////////////////////////////////////////////////////

// functions do not capture the environment that they
// have been called from (variable x cannot be accessed)

fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}