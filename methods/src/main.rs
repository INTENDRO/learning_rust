// using example from "rectangles" project and implementing function as method

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect is {:#?}", rect); // # -> pretty-printing
//     println!();

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect.area()
//     );
// }

///////////////////////////////////////////////////////////////////////

// implementing methods with multiple parameters

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };

//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }


///////////////////////////////////////////////////////////////////////

// associated function (function without self)

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::square(3);
    let rect2 = Rectangle::square(8);

    println!("rect1 is {:#?}", rect1); // # -> pretty-printing
    println!("rect2 is {:#?}", rect2); // # -> pretty-printing
    println!();

    println!("rect1 area: {}", rect1.area());
    println!("rect1 area: {}", rect2.area());
    println!();

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
}