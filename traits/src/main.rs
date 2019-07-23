// creating custom trait

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }


// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }


//////////////////////////////////////////////////////////////////////////

// creating custom trait with default method

// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     //comment/uncomment this to use default or custom method

//     // fn summarize(&self) -> String {
//     //     format!("{}, by {} ({})", self.headline, self.author, self.location)
//     // }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }


// fn main() {
//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from("The Pittsburgh Penguins once again are the best
//         hockey team in the NHL."),
//     };

//     println!("New article available! {}", article.summarize());
// }

//////////////////////////////////////////////////////////////////////////

// default implementations calling other methods in same trait

// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }


// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }


//////////////////////////////////////////////////////////////////////////

// traits as parameters

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }


// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     notify(tweet);
// }


//////////////////////////////////////////////////////////////////////////

// traits as parameters using trait bounds

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }


// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     notify(tweet);
// }


//////////////////////////////////////////////////////////////////////////

// multiple traits as parameters (may be equal, may be different)

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn notify(item1: &impl Summary, item2: &impl Summary) {
//     println!("Breaking news!");
//     println!("1: {}", item1.summarize());
//     println!("2: {}", item2.summarize());
//     println!();
// }


// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from("The Pittsburgh Penguins once again are the best
//         hockey team in the NHL."),
//     };

//     notify(&tweet, &article);
//     notify(&tweet, &tweet);
//     notify(&article, &article);
// }


//////////////////////////////////////////////////////////////////////////

// multiple traits as parameters using trait bounds (must be equal)

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn notify<T: Summary>(item1: &T, item2: &T) {
//     println!("Breaking news!");
//     println!("1: {}", item1.summarize());
//     println!("2: {}", item2.summarize());
//     println!();
// }


// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from("The Pittsburgh Penguins once again are the best
//         hockey team in the NHL."),
//     };

    
//     notify(&tweet, &tweet);
//     notify(&article, &article);
//     // notify(&tweet, &article); // this won't compile!
// }


//////////////////////////////////////////////////////////////////////////

/*
specifying multiple trait bounds with "+":

fn notify(item: impl Summary + Display) {

fn notify<T: Summary + Display>(item: T) {

*/


//////////////////////////////////////////////////////////////////////////

/*
clearer trait bounds with "where" clauses:

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
*/

//////////////////////////////////////////////////////////////////////////

// use trait for return type

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }


// fn main() {
//     let tweet = returns_summarizable();
//     println!("1 new tweet: {}", tweet.summarize());
// }

//////////////////////////////////////////////////////////////////////////

// implement standard traits on generic functions

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

//////////////////////////////////////////////////////////////////////////

// conditionally implement methods based on traits

use std::fmt::Display;
use std::collections::HashMap;

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let hash_pair:Pair<HashMap<i32,i32>> = Pair::new(HashMap::new(), HashMap::new());
    let int_pair = Pair::new(2,3);

    println!("hash_pair: {:?}", hash_pair);
    // hash_pair.cmp_display(); // PartialOrd is not implemented for HashMap -> does not compile

    println!("int_pair: {:?}", int_pair);
    int_pair.cmp_display();
}