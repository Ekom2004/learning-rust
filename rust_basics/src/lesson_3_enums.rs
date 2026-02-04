// ============================================
// 🦀 LESSON 3: Enums & Pattern Matching
// ============================================
// Enums let you define a type that can be ONE of several variants
// Pattern matching lets you handle each variant differently

// Define enums OUTSIDE of main (so helper functions can use them)
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

.

fn main() {
    println!("🦀 Lesson 3: Enums & Pattern Matching\n");

    // ==========================================
    // PART 1: Basic Enums
    // ==========================================
    // An enum is a type that can be one of several variants
    
    let player_direction = Direction::Up;

    // Match on an enum (MUST handle all cases!)
    match player_direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }

    // ==========================================
    // PART 2: Enums with Data
    // ==========================================
    // Enums can hold data inside each variant
    
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello!"));
    let msg4 = Message::ChangeColor(255, 0, 128);

    // Handle each variant
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);

    // ==========================================
    // PART 3: Option<T> - Rust's Null Replacement
    // ==========================================
    // Rust doesn't have null! Instead, use Option<T>
    // Option is either:  Some(value)  or  None
    
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    // You MUST handle both cases
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number!"),
    }

    // Real example: finding in a list
    let numbers = vec![1, 2, 3, 4, 5];
    let found = numbers.iter().find(|&&x| x == 3);

    match found {
        Some(n) => println!("Found: {}", n),
        None => println!("Not found"),
    }

    // ==========================================
    // PART 4: if let (Shortcut for Single Match)
    // ==========================================
    // When you only care about ONE variant, use if let
    
    let my_option = Some(42);

    // Instead of full match, use if let:
    if let Some(n) = my_option {
        println!("if let got: {}", n);
    }

    // With else:
    if let Some(n) = no_number {
        println!("Got: {}", n);
    } else {
        println!("Got nothing!");
    }

    // ==========================================
    // PART 5: Result<T, E> - Handling Errors
    // ==========================================
    // Result is either:  Ok(value)  or  Err(error)
    
    let good_result: Result<i32, String> = Ok(100);
    let bad_result: Result<i32, String> = Err(String::from("Something went wrong"));

    match good_result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    },

    match bad_result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Real example: parsing a string to number
    let parsed = "42".parse::<i32>();
    match parsed {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    let bad_parse = "not a number".parse::<i32>();
    match bad_parse {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    // ==========================================
    // PART 6: Match with Guards
    // ==========================================
    let number = 7;

    match number {
        n if n < 0 => println!("Negative"),
        0 => println!("Zero"),
        n if n < 10 => println!("Single digit: {}", n),
        n => println!("Big number: {}", n),
    }

    // ==========================================
    // PART 7: Destructuring in Match
    // ==========================================
    let point = (3, 4);

    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at {}", x),
        (0, y) => println!("On y-axis at {}", y),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    println!("\n🎉 Lesson 3 Complete!");
    println!("Key takeaways:");
    println!("  - Enums = one of several variants");
    println!("  - match = handle all variants");
    println!("  - Option = Some(value) or None");
    println!("  - Result = Ok(value) or Err(error)");
}

// ==========================================
// HELPER FUNCTION
// ==========================================
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit message received");
        }
        Message::Move { x, y } => {
            println!("Move to x={}, y={}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}
