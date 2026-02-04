// ============================================
// 🦀 LESSON 1: Rust Basics - Your First Steps
// ============================================
// Run this file with: cargo run
// After each section, uncomment the code and run it!

fn main() {
    println!("🦀 Welcome to Rust!");
    
    // ==========================================
    // PART 1: Variables and let bindings
    // ==========================================
    
    // In Rust, variables are IMMUTABLE by default
    let name = "Rust Learner";
    println!("Hello, {}!", name);
    
    // This would FAIL (try uncommenting it!):
    // name = "New Name";  // ❌ Error: cannot assign twice to immutable variable
    
    // To make a variable mutable, use `mut`
    let mut age = 25;
    println!("Age: {}", age);
    
    age = 26;  // ✅ This works because `age` is mutable
    println!("Age after birthday: {}", age);
    
    // ==========================================
    // PART 2: Basic Types
    // ==========================================
    
    // Integers
    let x: i32 = 42;        // 32-bit signed integer
    let y: u64 = 100;       // 64-bit unsigned integer
    
    // Floats
    let pi: f64 = 3.14159;
    
    // Booleans
    let is_learning: bool = true;
    
    // Characters (single quotes, supports unicode!)
    let emoji: char = '🦀';
    
    println!("x={}, y={}, pi={}, learning={}, emoji={}", x, y, pi, is_learning, emoji);
    
    // ==========================================
    // PART 3: String vs &str
    // ==========================================
    
    // &str - string "slice", borrowed, immutable, usually hardcoded
    let greeting: &str = "Hello";
    
    // String - owned, growable, heap-allocated
    let mut message = String::from("Hello");
    message.push_str(", World!");  // Can modify because it's owned AND mut
    
    println!("{} vs {}", greeting, message);
    
    // ==========================================
    // PART 4: Functions
    // ==========================================
    
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let doubled = double(21);
    println!("21 doubled = {}", doubled);
    
    greet("Rust Student");
    
    // ==========================================
    // PART 5: Control Flow
    // ==========================================
    
    // if / else
    let number = 7;
    if number < 5 {
        println!("less than 5");
    } else if number == 5 {
        println!("equals 5");
    } else {
        println!("greater than 5");
    }
    
    // if is an EXPRESSION (returns a value!)
    let description = if number > 5 { "big" } else { "small" };
    println!("{} is {}", number, description);
    
    // for loop
    println!("\nCounting with for:");
    for i in 1..=5 {  // 1..=5 means 1, 2, 3, 4, 5 (inclusive)
        println!("  {}", i);
    }
    
    // loop with break
    println!("\nLoop until break:");
    let mut counter = 0;
    let final_count = loop {
        counter += 1;
        if counter == 3 {
            break counter * 10;  // break can return a value!
        }
    };
    println!("Final count: {}", final_count);
    
    // ==========================================
    // PART 6: Tuples and Arrays
    // ==========================================
    
    // Tuple - fixed size, mixed types
    let person: (&str, i32, bool) = ("Alice", 30, true);
    println!("Name: {}, Age: {}, Active: {}", person.0, person.1, person.2);
    
    // Destructuring
    let (name, age, active) = person;
    println!("Destructured: {} is {} years old", name, age);
    
    // Array - fixed size, same type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First element: {}", numbers[0]);
    println!("Array length: {}", numbers.len());
    
    // ==========================================
    // 🎉 YOU MADE IT!
    // ==========================================
    println!("\n🎉 Lesson 1 Complete!");
    println!("Next: Run `cargo run` to see this all work");
    println!("Then: Look at lesson_2_ownership.rs");
}

// ==========================================
// FUNCTIONS (defined outside main)
// ==========================================

// Parameters need type annotations
// Return type comes after ->
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = this is the return value (expression)
}

// You can also use explicit return
fn double(x: i32) -> i32 {
    return x * 2;  // Works, but implicit return is more idiomatic
}

// Functions with no return value (returns unit type `()`)
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
