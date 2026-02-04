// ============================================
// 🦀 LESSON 5: Result & Error Propagation
// ============================================
// Result is Rust's way of handling operations that might fail.
// The `?` operator makes error handling clean and concise.

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("🦀 Lesson 5: Result<T, E> & the ? operator\n");

    // ==========================================
    // PART 1: Result Basics (Review)
    // ==========================================
    
    // Result has two variants:
    // - Ok(value)  = success, here's the value
    // - Err(error) = failure, here's what went wrong
    
    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err(String::from("something went wrong"));

    // Handle with match
    match success {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match failure {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // ==========================================
    // PART 2: Real-World Example - Division
    // ==========================================
    
    // Division can fail (divide by zero), so we return Result
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Cannot divide by zero!"))
        } else {
            Ok(a / b)
        }
    }

    println!("\n--- Division Examples ---");
    
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // ==========================================
    // PART 3: The ? Operator (The Magic)
    // ==========================================
    
    // The ? operator does two things:
    // 1. If Ok(value) -> unwrap and continue
    // 2. If Err(e) -> return early with that error
    
    // WITHOUT ? (verbose)
    fn read_file_verbose(path: &str) -> Result<String, io::Error> {
        let file_result = File::open(path);
        
        let mut file = match file_result {
            Ok(f) => f,
            Err(e) => return Err(e),  // Return early on error
        };
        
        let mut contents = String::new();
        
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(e) => Err(e),
        }
    }

    // WITH ? (clean!)
    fn read_file_clean(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;  // ? = return Err if failed
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;  // ? = return Err if failed
        Ok(contents)
    }

    // Even cleaner (one-liner style)
    fn read_file_oneliner(path: &str) -> Result<String, io::Error> {
        std::fs::read_to_string(path)
    }

    println!("\n--- File Reading Examples ---");
    
    // Try reading a file that doesn't exist
    match read_file_clean("nonexistent.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Could not read file: {}", e),
    }

    // ==========================================
    // PART 4: Chaining with ?
    // ==========================================
    
    // The power of ? is chaining multiple fallible operations
    fn process_config() -> Result<String, io::Error> {
        // Each ? could fail, but code stays clean
        let config = std::fs::read_to_string("config.txt")?;
        let backup = std::fs::read_to_string("backup.txt")?;
        Ok(format!("Config: {}, Backup: {}", config, backup))
    }

    println!("\n--- Chained Operations ---");
    match process_config() {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Config processing failed: {}", e),
    }

    // ==========================================
    // PART 5: unwrap() and expect() (Dangerous!)
    // ==========================================
    
    // unwrap() - Get the value OR PANIC (crash)
    // Only use when you're 100% SURE it won't fail
    let sure_thing: Result<i32, &str> = Ok(42);
    let value = sure_thing.unwrap();  // Fine, we know it's Ok
    println!("\nUnwrapped: {}", value);

    // expect() - Like unwrap(), but with a custom panic message
    let another: Result<i32, &str> = Ok(100);
    let value2 = another.expect("This should never fail!");
    println!("Expected: {}", value2);

    // DON'T DO THIS (would crash):
    // let bad: Result<i32, &str> = Err("oops");
    // let crash = bad.unwrap();  // PANIC!

    // ==========================================
    // PART 6: unwrap_or() and unwrap_or_else()
    // ==========================================
    
    // unwrap_or() - Get the value OR use a default
    let maybe_number: Result<i32, &str> = Err("failed");
    let number = maybe_number.unwrap_or(0);  // Returns 0 since it's Err
    println!("\nunwrap_or: {}", number);

    // unwrap_or_else() - Get the value OR compute a default
    let another_maybe: Result<i32, &str> = Err("failed");
    let computed = another_maybe.unwrap_or_else(|e| {
        println!("Error occurred: {}, using fallback", e);
        -1
    });
    println!("unwrap_or_else: {}", computed);

    // ==========================================
    // PART 7: Your Turn!
    // ==========================================
    
    println!("\n--- Exercise Results ---");
    
    // Exercise 1: Call this function and handle both cases
    match calculate_percentage(50, 100) {
        Ok(pct) => println!("Percentage: {}%", pct),
        Err(e) => println!("Error: {}", e),
    }

    // Exercise 2: Call with zero total
    match calculate_percentage(50, 0) {
        Ok(pct) => println!("Percentage: {}%", pct),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n🎉 Lesson 5 Complete!");
    println!("Key takeaways:");
    println!("  1. Result = Ok(value) or Err(error)");
    println!("  2. ? operator = early return on Err, unwrap on Ok");
    println!("  3. Use match for explicit handling");
    println!("  4. Avoid unwrap() in production code");
}

// ==========================================
// EXERCISE FUNCTION
// ==========================================

// Calculate percentage, but return error if total is 0
fn calculate_percentage(part: i32, total: i32) -> Result<i32, String> {
    if total == 0 {
        Err(String::from("Cannot calculate percentage: total is zero"))
    } else {
        Ok((part * 100) / total)
    }
}

// ==========================================
// BONUS: Using ? in main()
// ==========================================

// Normally main() returns (), but you can make it return Result!
// This allows using ? in main:
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let contents = std::fs::read_to_string("file.txt")?;
//     println!("{}", contents);
//     Ok(())
// }
