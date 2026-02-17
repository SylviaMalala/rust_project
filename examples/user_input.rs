use std::io;

fn main() {
    println!("=== Rust User Input Demo ===");
    println!("What's your name?");
    
    // Create a mutable String to store input
    let mut name = String::new();
    
    // Read from standard input
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    // Trim whitespace and newline
    let name = name.trim();
    
    println!("Hello, {}! Welcome to Rust! 🎉", name);
    
    // Ask for their age
    println!("\nHow old are you?");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read line");
    
    // Parse the age to a number
    match age_input.trim().parse::<u32>() {
        Ok(age) => {
            println!("Great! You're {} years old.", age);
            
            if age >= 18 {
                println!("You're an adult! 👨");
            } else {
                println!("You're still young! 👶");
            }
        },
        Err(_) => {
            println!("That doesn't look like a valid age!");
        }
    }
}
