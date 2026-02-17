fn main() {
    println!("🦀 Rust Data Types Demo\n");
    
    // Integer types
    println!("=== Integer Types ===");
    let age: u8 = 25;           // unsigned 8-bit (0 to 255)
    let score: i32 = -100;      // signed 32-bit
    println!("Age: {}, Score: {}", age, score);
    
    // Floating point
    println!("\n=== Floating Point ===");
    let pi: f64 = 3.14159;
    let temperature: f32 = 23.5;
    println!("Pi: {}, Temperature: {}°C", pi, temperature);
    
    // Boolean
    println!("\n=== Boolean ===");
    let is_rust_awesome = true;
    let is_easy = false;
    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Is it easy? {}", is_easy);
    
    // Character
    println!("\n=== Character ===");
    let letter = 'R';
    let emoji = '🦀';
    println!("Letter: {}, Emoji: {}", letter, emoji);
    
    // String types
    println!("\n=== String Types ===");
    let string_literal = "Hello";           // &str (string slice)
    let mut string_type = String::from("World"); // String (heap-allocated)
    string_type.push_str("!");
    println!("{} {}", string_literal, string_type);
    
    // Tuples
    println!("\n=== Tuples ===");
    let person: (&str, u8, bool) = ("Alice", 30, true);
    println!("Name: {}, Age: {}, Active: {}", person.0, person.1, person.2);
    
    // Arrays (fixed size)
    println!("\n=== Arrays ===");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First number: {}", numbers[0]);
    println!("Array length: {}", numbers.len());
    
    // Vectors (dynamic size)
    println!("\n=== Vectors ===");
    let mut fruits = vec!["Apple", "Banana", "Orange"];
    fruits.push("Mango");
    println!("Fruits: {:?}", fruits);
    println!("Number of fruits: {}", fruits.len());
}
