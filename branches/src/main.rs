fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }

    let new_var = if number != 0 {
        3
    } else {
        2
    };

    println!("new_var is {}", new_var);
}
