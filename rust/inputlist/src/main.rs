use std::io;

fn main() {
    let mut inputs: Vec<String> = Vec::new(); // store multiple inputs

    loop {
        println!("Enter how youfeel now (or type 'exit' to quit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim(); // remove newline
        if input == "exit" {
            break;
        }

        inputs.push(input.to_string()); // store in vector
        println!("Current inputs: {:?}", inputs);
    }

    println!("You entered: {:?}", inputs);
}