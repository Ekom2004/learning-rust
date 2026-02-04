// ============================================
// 🦀 LESSON 4: The Option Enum
// ============================================
// "The Billion Dollar Mistake" fix.
// Rust doesn't have `null`. instead, it has `Option`.

// It's defined in the standard library like this:
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    println!("🦀 Lesson 4: Option<T>\n");

    // ==========================================
    // PART 1: Creating Options
    // ==========================================
    
    // Some(value) means "I have a value"
    let number: Option<i32> = Some(10);
    
    // None means "I have nothing" (like null)
    let nothing: Option<i32> = None;

    // Note: We MUST tell Rust the type for None, 
    // because it can't guess what "kind" of nothing it is.
    
    println!("number is {:?}", number);
    println!("nothing is {:?}", nothing);

    // ==========================================
    // PART 2: Unwrapping (Getting the value out)
    // ==========================================
    // You cannot use `Option<i32>` as an `i32` directly.
    // You have to "unwrap" it first.

    // 1. SAFE WAY: match (Handles both cases)
    match number {
        Some(n) => println!("The number is: {}", n + 5), // We can do math on `n`
        None => println!("There was no number!"),
    }

    // 2. SAFE WAY: if let (Short for match)
    if let Some(n) = number {
        println!("We saw a number: {}", n);
    }

    // 3. DANGEROUS WAY: unwrap()
    // CRASHES if the value is None! Only use if you are 100% sure.
    let val = number.unwrap(); 
    println!("Unwrapped value: {}", val);

    // This would CRASH:
    // let crash = nothing.unwrap(); 

    // 4. SAFER UNWRAP: unwrap_or(default)
    // If None, give me a default value instead of crashing.
    let safe_val = nothing.unwrap_or(0); 
    println!("Unwrapped or default: {}", safe_val); // Prints 0

    // ==========================================
    // PART 3: Why is this useful?
    // ==========================================
    
    let developers = vec!["Alice", "Bob", "Charlie"];
    
    // .get() returns an Option because the index might be out of bounds!
    let first = developers.get(0);  // Some("Alice")
    let tenth = developers.get(10); // None

    println!("First dev: {:?}", first);
    println!("Tenth dev: {:?}", tenth);
    
    handle_developer(first);
    handle_developer(tenth);
}

fn handle_developer(dev: Option<&&str>) {
    match dev {
        Some(name) => println!("Found developer: {}", name),
        None => println!("No developer found at that index."),
    }
}
