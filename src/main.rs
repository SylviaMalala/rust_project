use std::io::{self, Write};

fn main() {
    println!("🦀 Rust Mini Task Manager");
    println!("========================\n");
    
    // Using a vector to store tasks (dynamic array)
    let mut tasks: Vec<String> = Vec::new();
    
    loop {
        println!("\nOptions:");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Remove task");
        println!("4. Exit");
        print!("\nChoose an option: ");
        
        // Flush stdout to ensure prompt appears
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                tasks.push(task.trim().to_string());
                println!("✅ Task added!");
            },
            "2" => {
                if tasks.is_empty() {
                    println!("📋 No tasks yet!");
                } else {
                    println!("\n📋 Your Tasks:");
                    for (index, task) in tasks.iter().enumerate() {
                        println!("  {}. {}", index + 1, task);
                    }
                }
            },
            "3" => {
                if tasks.is_empty() {
                    println!("❌ No tasks to remove!");
                } else {
                    print!("Enter task number to remove: ");
                    io::stdout().flush().unwrap();
                    let mut num = String::new();
                    io::stdin().read_line(&mut num).unwrap();
                    
                    if let Ok(index) = num.trim().parse::<usize>() {
                        if index > 0 && index <= tasks.len() {
                            tasks.remove(index - 1);
                            println!("✅ Task removed!");
                        } else {
                            println!("❌ Invalid task number!");
                        }
                    }
                }
            },
            "4" => {
                println!("👋 Goodbye!");
                break;
            },
            _ => println!("❌ Invalid option! Please choose 1-4."),
        }
    }
}
