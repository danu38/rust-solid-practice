use std::io;

fn main() {
    println!("Please enter your name:");

    let mut name = String::new(); // create a mutable string to store input
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line"); // read input from user

    println!("Hello, {}!", name.trim()); // trim removes newline
}