// using normal variables

// fn main() {
//     let width = 30;
//     let height = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width, height)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


/////////////////////////////////////////////////////////////////

// using tuples

// fn main() {
//     let rect = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect)
//     );
// }

// fn area(rect: (u32, u32)) -> u32 {
//     rect.0 * rect.1
// }


/////////////////////////////////////////////////////////////////

// using structs

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect)
//     );
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

/////////////////////////////////////////////////////////////////

// using  and printing structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect is {:?}", rect);
    println!();

    println!("rect is {:#?}", rect); // # -> pretty-printing
    println!();

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}