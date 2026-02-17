# Getting Started with Rust - A Beginner's Guide
## Prompt-Powered Kickstart: Building a Beginner's Toolkit for Rust Programming Language

---

## 1. Title & Objective

**Technology:** Rust Programming Language  
**Why I chose it:** Rust is a modern systems programming language that emphasizes memory safety, concurrency, and performance. It's increasingly popular for building reliable and efficient software, from web servers to embedded systems. I chose Rust because it represents the future of systems programming and offers unique learning opportunities around ownership and borrowing concepts.

**End Goal:** Create a simple command-line application that demonstrates Rust's core features including:
- Hello World program
- Basic data types and variables
- User input handling
- A simple task manager CLI tool

---

## 2. Quick Summary of the Technology

**What is Rust?**  
Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency. It was originally created by Mozilla Research and first released in 2010. Rust prevents segfaults and guarantees thread safety through its ownership system at compile time.

**Where is it used?**
- Systems programming (operating systems, device drivers)
- Web backends (using frameworks like Actix, Rocket)
- WebAssembly applications
- Command-line tools
- Embedded systems
- Blockchain and cryptocurrency systems

**Real-world example:**  
Discord uses Rust for their performance-critical services, particularly for handling millions of concurrent users. They migrated from Go to Rust and saw significant performance improvements in their Read States service, reducing latency spikes from seconds to sub-millisecond levels.

---

## 3. System Requirements

### Operating Systems
- ✅ Linux (Ubuntu, Fedora, etc.)
- ✅ macOS
- ✅ Windows 10/11

### Tools Required
1. **Text Editor/IDE:**
   - VS Code (recommended) with Rust Analyzer extension
   - IntelliJ IDEA with Rust plugin
   - Vim/Neovim with rust.vim

2. **Rust Toolchain:**
   - Rustup (Rust installer and version manager)
   - Cargo (package manager and build tool)
   - rustc (Rust compiler)

3. **Additional Tools:**
   - Git (for version control)
   - Terminal/Command Prompt

### Minimum Hardware
- 2 GB RAM (4 GB recommended)
- 1 GB free disk space for toolchain
- Any modern processor (x86_64, ARM64)

---

## 4. Installation & Setup Instructions

### Step 1: Install Rust (Linux/macOS)

Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions. When prompted, choose option 1 (default installation).

**Expected Output:**
```
info: downloading installer
info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
...
Rust is installed now. Great!
```

### Step 2: Configure Your Environment

Add Rust to your PATH by running:

```bash
source $HOME/.cargo/env
```

Or restart your terminal.

### Step 3: Verify Installation

Check that everything is installed correctly:

```bash
rustc --version
cargo --version
rustup --version
```

**Expected Output:**
```
rustc 1.76.0 (07dca489a 2024-02-04)
cargo 1.76.0 (c84b36747 2024-01-18)
rustup 1.26.0 (5af9b9484 2023-04-05)
```

### Step 4: Install VS Code Extensions (Optional but Recommended)

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search and install:
   - **rust-analyzer** (Rust language support)
   - **CodeLLDB** (Debugger)
   - **crates** (Cargo.toml dependency management)

### Step 5: Create Your First Project

```bash
# Navigate to your workspace
cd ~/phase_four/rust_project

# Initialize a new Rust project
cargo init --name rust_toolkit

# This creates:
# - Cargo.toml (project configuration)
# - src/main.rs (main source file)
# - .gitignore
```

**What Cargo Created:**

```
rust_project/
├── Cargo.toml       # Project manifest
├── src/
│   └── main.rs      # Your Rust code goes here
└── .gitignore       # Git ignore file
```

---

## 5. Minimal Working Example

### Example 1: Hello World (src/main.rs)

```rust
// The main function is the entry point of every Rust program
fn main() {
    // println! is a macro (note the ! ) that prints to console
    println!("Hello, Rust World! 🦀");
    
    // Let's add some basic features
    let language = "Rust";
    let year = 2024;
    
    println!("I'm learning {} in {}!", language, year);
}
```

**How to Run:**
```bash
cargo run
```

**Expected Output:**
```
   Compiling rust_toolkit v0.1.0 (/home/sylvia-malala/phase_four/rust_project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/rust_toolkit`
Hello, Rust World! 🦀
I'm learning Rust in 2024!
```

### Example 2: Interactive User Input (src/examples/user_input.rs)

```rust
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
}
```

### Example 3: Simple Task Manager CLI

**File: src/main.rs (Enhanced Version)**

```rust
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
```

**How to Run:**
```bash
cargo run
```

**Sample Interaction:**
```
🦀 Rust Mini Task Manager
========================

Options:
1. Add task
2. View tasks
3. Remove task
4. Exit

Choose an option: 1
Enter task description: Learn Rust ownership
✅ Task added!

Options:
1. Add task
2. View tasks
3. Remove task
4. Exit

Choose an option: 2

📋 Your Tasks:
  1. Learn Rust ownership
```

---

## 6. AI Prompt Journal

### Prompt 1: Initial Learning
**Prompt Used:**  
"Explain Rust programming language in simple terms. What makes it different from other languages like C++ or Python? Give me a beginner-friendly overview."

**AI Response Summary:**  
The AI explained that Rust is a systems programming language focused on safety and performance. It highlighted the ownership system as Rust's unique feature that prevents memory bugs at compile time without needing a garbage collector.

**Link to Curriculum:**  
[Introduction to Rust - ai.moringaschool.com](https://ai.moringaschool.com)

**Evaluation:**  
⭐⭐⭐⭐⭐ Extremely helpful! The explanation helped me understand why Rust is valuable and what problems it solves. The comparison with other languages gave me context.

---

### Prompt 2: Installation Process
**Prompt Used:**  
"Give me step-by-step instructions to install Rust on Linux Ubuntu. Include verification steps."

**AI Response Summary:**  
The AI provided the curl command for rustup installation and explained how to verify the installation using `rustc --version` and `cargo --version` commands.

**Link to Curriculum:**  
[Rust Setup Guide - ai.moringaschool.com](https://ai.moringaschool.com)

**Evaluation:**  
⭐⭐⭐⭐⭐ Perfect! The installation went smoothly following the AI's instructions. The verification steps helped confirm everything was working.

---

### Prompt 3: First Program
**Prompt Used:**  
"How do I create my first Rust program? Explain the Hello World example with comments explaining each line."

**AI Response Summary:**  
The AI showed how to use `cargo new` to create a project and explained the basic structure of a Rust program including the `main` function and `println!` macro.

**Link to Curriculum:**  
[Your First Rust Program - ai.moringaschool.com](https://ai.moringaschool.com)

**Evaluation:**  
⭐⭐⭐⭐ Very helpful! I learned about Cargo's project structure and understood the difference between functions and macros (the `!` symbol).

---

### Prompt 4: User Input Handling
**Prompt Used:**  
"How do I read user input in Rust? Show me a simple example that asks for the user's name and greets them."

**AI Response Summary:**  
The AI demonstrated using `std::io` module and the `read_line` method. It explained the concept of mutable variables and error handling with `expect()`.

**Link to Curriculum:**  
[Rust I/O Operations - ai.moringaschool.com](https://ai.moringaschool.com)

**Evaluation:**  
⭐⭐⭐⭐ Good explanation, though I initially struggled with the borrowing concept (`&mut`). After re-reading, it made sense.

---

### Prompt 5: Building a CLI Application
**Prompt Used:**  
"Help me build a simple command-line task manager in Rust. It should allow adding tasks, viewing tasks, and removing tasks. Keep it beginner-friendly."

**AI Response Summary:**  
The AI provided a complete example using Vec (vector), loops, match expressions, and basic I/O. It explained each component's purpose.

**Link to Curriculum:**  
[Building CLI Apps with Rust - ai.moringaschool.com](https://ai.moringaschool.com)

**Evaluation:**  
⭐⭐⭐⭐⭐ Excellent! This brought together multiple concepts. The match expression for menu handling was elegant. I learned about vectors and iteration.

---

### Prompt 6: Error Handling
**Prompt Used:**  
"I'm getting a 'cannot borrow as mutable' error in Rust. What does this mean and how do I fix it?"

**AI Response Summary:**  
The AI explained Rust's borrowing rules and showed how to properly declare mutable variables using `let mut`. It clarified the ownership system basics.

**Link to Curriculum:**  
[Rust Ownership and Borrowing - ai.moringaschool.com](https://ai.moringaschool.com)

**Evaluation:**  
⭐⭐⭐⭐⭐ Critical learning moment! Understanding borrowing is key to Rust. The AI's explanation with examples made it click.

---

### Prompt 7: Code Organization
**Prompt Used:**  
"How should I organize my Rust project? Where do different files go and how do I use modules?"

**AI Response Summary:**  
The AI explained Cargo's standard project structure, the `src/` directory, and how to create additional modules and files.

**Link to Curriculum:**  
[Rust Project Structure - ai.moringaschool.com](https://ai.moringaschool.com)

**Evaluation:**  
⭐⭐⭐⭐ Helpful for understanding professional project organization. Made my code cleaner.

---

## 7. Common Issues & Fixes

### Issue 1: "rustc: command not found" after installation

**Problem:**  
After installing Rust with rustup, the terminal doesn't recognize `rustc` or `cargo` commands.

**Solution:**
```bash
# Add Cargo's bin directory to PATH
source $HOME/.cargo/env

# Or add this to ~/.bashrc or ~/.zshrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**References:**
- [Rustup Documentation](https://rust-lang.github.io/rustup/)
- [Stack Overflow: Rust not in PATH](https://stackoverflow.com/questions/tagged/rust+path)

---

### Issue 2: "cannot borrow `x` as mutable, as it is not declared as mutable"

**Problem:**
```rust
let name = String::new();
io::stdin().read_line(&mut name) // Error!
```

**Error Message:**
```
error[E0596]: cannot borrow `name` as mutable, as it is not declared as mutable
```

**Solution:**  
Declare the variable as mutable:
```rust
let mut name = String::new();  // Add 'mut' keyword
io::stdin().read_line(&mut name)  // Now it works!
```

**Why this happens:**  
Rust variables are immutable by default. To modify a variable, you must explicitly mark it as mutable with `mut`.

---

### Issue 3: Compilation is slow on first build

**Problem:**  
Running `cargo build` or `cargo run` takes a long time (1-2 minutes) on the first execution.

**Solution:**  
This is normal! Cargo is compiling all dependencies and creating the build cache. Subsequent builds will be much faster thanks to incremental compilation.

**Tips to speed up:**
```bash
# Use release mode only when needed
cargo run          # Development mode (default, faster compile)
cargo run --release  # Optimized mode (slower compile, faster execution)
```

---

### Issue 4: "unused variable" warnings

**Problem:**
```rust
let x = 5;
// Compiler warning: unused variable: `x`
```

**Solution Option 1:** Use the variable
```rust
let x = 5;
println!("x is {}", x);
```

**Solution Option 2:** Prefix with underscore if intentionally unused
```rust
let _x = 5;  // No warning
```

---

### Issue 5: Understanding ownership errors

**Problem:**
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // Error: value borrowed here after move
```

**Solution:**  
Either clone the string or use references:

```rust
// Option 1: Clone
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}", s1);  // Works!

// Option 2: Use references
let s1 = String::from("hello");
let s2 = &s1;
println!("{}", s1);  // Works!
```

**Reference:**
- [The Rust Book - Understanding Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

### Issue 6: "linker 'cc' not found" error on Linux

**Problem:**  
Cargo fails with a linker error during compilation.

**Solution:**  
Install build essentials:

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential

# Fedora
sudo dnf install gcc

# Arch
sudo pacman -S base-devel
```

---

## 8. References

### Official Documentation
- 📘 [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Comprehensive guide (highly recommended!)
- 📖 [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn through runnable examples
- 📚 [Standard Library Documentation](https://doc.rust-lang.org/std/) - API reference
- 🛠️ [Cargo Book](https://doc.rust-lang.org/cargo/) - Package manager guide

### Video Tutorials
- 🎥 [Rust Programming Course for Beginners](https://www.youtube.com/watch?v=MsocPEZBd-M) - freeCodeCamp (13 hours)
- 🎥 [Rust Crash Course](https://www.youtube.com/watch?v=zF34dRivLOw) - Traversy Media (1 hour)
- 🎥 [Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty) - YouTube channel with great tutorials

### Interactive Learning
- 💻 [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get you used to Rust
- 🏃 [Exercism Rust Track](https://exercism.org/tracks/rust) - Coding exercises with mentorship
- 🎮 [Rust Playground](https://play.rust-lang.org/) - Try Rust in your browser

### Community Resources
- 💬 [Rust Users Forum](https://users.rust-lang.org/) - Ask questions
- 🦀 [r/rust](https://www.reddit.com/r/rust/) - Reddit community
- 💡 [This Week in Rust](https://this-week-in-rust.org/) - Weekly newsletter
- 📱 [Rust Discord](https://discord.gg/rust-lang) - Real-time chat

### Helpful Blog Posts
- 📝 [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- 📝 [Learning Rust the Dangerous Way](http://cliffle.com/p/dangerust/)
- 📝 [Rust for JavaScript Developers](https://www.rustfromjs.com/)

### Tools & Extensions
- 🔧 [rust-analyzer](https://rust-analyzer.github.io/) - IDE support
- 🎨 [Clippy](https://github.com/rust-lang/rust-clippy) - Linting tool
- 📊 [rustfmt](https://github.com/rust-lang/rustfmt) - Code formatter

---

## 9. Next Steps

After completing this toolkit, here are recommended next steps:

1. **Master the Fundamentals:**
   - Complete [Rustlings](https://github.com/rust-lang/rustlings) exercises
   - Read chapters 1-10 of The Rust Book

2. **Build Small Projects:**
   - Command-line file searcher
   - Simple HTTP server
   - Text-based game

3. **Explore Advanced Topics:**
   - Lifetimes and advanced ownership
   - Traits and generics
   - Async/await programming
   - Macro systems

4. **Join the Community:**
   - Contribute to open-source Rust projects
   - Attend local Rust meetups
   - Share your projects on r/rust

---

## 10. Conclusion

Rust is a challenging but rewarding language to learn. Its emphasis on safety and performance makes it ideal for building robust applications. The ownership system, while initially confusing, becomes a powerful tool for writing correct code.

**Key Takeaways:**
- ✅ Rust prevents memory bugs at compile time
- ✅ The ownership system is unique and powerful
- ✅ Compiler errors are detailed and helpful
- ✅ Strong community and excellent documentation
- ✅ Great for systems programming and beyond

**My Learning Journey:**  
Using AI prompts significantly accelerated my learning process. Instead of getting stuck on errors for hours, I could quickly get explanations and solutions. The AI helped me understand concepts like ownership and borrowing much faster than traditional resources alone.

---

**Project Completion Date:** February 17, 2026  
**Created by:** Moringa School AI Capstone Student  
**Technology:** Rust Programming Language 🦀  
**Status:** ✅ Complete and tested
