// The main function is the entry point of every Rust program
fn main() {
    // println! is a macro (note the !) that prints to console
    println!("Hello, Rust World! 🦀");
    
    // Let's add some basic features
    let language = "Rust";
    let year = 2026;
    
    println!("I'm learning {} in {}!", language, year);
    
    // Demonstrating variables and types
    let is_fun = true;
    let difficulty: f64 = 7.5;
    
    println!("\nIs Rust fun? {}", is_fun);
    println!("Difficulty level (1-10): {}", difficulty);
    
    // Working with strings
    let mut greeting = String::from("Welcome to ");
    greeting.push_str("Rust programming!");
    println!("\n{}", greeting);
}
