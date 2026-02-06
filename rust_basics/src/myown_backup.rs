use std::io;  // Import for reading input

// ============================================
// STRUCT = HAS all these fields (name AND age AND...)
// A student HAS all of these at once
// ============================================

// so options is to make sure we handle cases where the user might not enter a value
// and if they they dont enter the valuye the programm wont crash
struct Student {
    name: Option<String>,
    age: Option<u8>,
    is_learning: bool,
    is_commuter: bool,
}

// ============================================sting too


// ENUM = IS one of these options (Active OR Inactive OR Pending)
// A status can only BE one of these, not all
// ============================================
enum Status {
    Active,
    Inactive,
    Pending,
}

fn main(){


    // ============================================
    // RESULT = Ok(value) OR Err(error)
    // Used for operations that might fail
    // ============================================
    let my_goodday: Result<String, String> = Ok(String::from("having a good day"));
    let my_badday: Result<String, String> = Err(String::from("having a bad day"));

    match my_goodday {
        Ok(goodday) => println!("{}", goodday),
        Err(badday) => println!("{}", badday),
    }

    match my_badday {
        Ok(goodday) => println!("{}", goodday),
        Err(badday) => println!("Error: {}", badday),
    }

    let mut os_version: Option<String> = Some(String::from("mac os 19"));

    // so some and none are its like value and null
    match os_version{
        Some(os_version) => println!("os version is {}", os_version),
        None => println!("no os version found"),
    }

    println!("hello world");

    let x = 5;
    let y = 8;
    println!("x= {} y= {}", x, y);

    let name: &str = "Ekom";
    let age: i32 = 21;
    let is_learning = true;
    let emoji: char = '🦀';

    println!("name: {}, age: {}, is_learning: {}, emoji: {}", name, age, is_learning, emoji);

    let result = add(5, 3);
    println!("5 + 3 = {}", result);

    // ============================================
    // Taking user input
    // ============================================
    println!("\nWhat's your favorite language?");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    
    let answer = input.trim();
    println!("Nice! {} is a great choice!", answer);

    // ============================================
    // mut makes the variable mutable 
    // String::from makes it growable
    // ============================================
    let mut fullname = String::from("ekom");
    fullname.push_str("otu");
    println!("Full name: {}", fullname);

    // if/else statement for conditions 
    let num = 5;
    if num < 0 {
        println!("number is negative");
    } else if num == 0 {
        println!("number is zero");
    } else {
        println!("number is positive");
    }

    // ============================================
    // OWNERSHIP AND CLONE
    // Clone copies the variable to another variable
    // Without clone, it MOVES (original becomes invalid)
    // ============================================
    let a1 = String::from("hello");
    let a2 = a1.clone();  // Copy it
    println!("a1= {}, a2= {}", a1, a2);  // Both valid!

    // ============================================
    // BORROWING
    // Use & to lend the variable without giving ownership
    // ============================================
    let a3 = String::from("day of doom");
    print_borrowed(&a3);  // Lend with &
    println!("Still mine: {}", a3);  // Still valid!

    // ============================================
    // MUTABLE BORROWING
    // &mut lets the function MODIFY our data
    // ============================================
    let mut a5 = String::from("doom");
    mutable_borrow(&mut a5);
    println!("Modified: {}", a5);  // Now "doom is coming"

    // ============================================
    // STRUCT EXAMPLE
    // Struct = HAS all these fields (AND)
    // A student HAS name AND age AND is_learning, all at once
    // ============================================
    let student1 = Student {
        name: Some(String::from("Ekom")),
        age: Some(21),
        is_learning: true,
        is_commuter: false,
    };
    // Access ALL fields anytime with dot notation
    // Use {:?} for Option types, or unwrap them
    println!("Student: {:?}, age {:?}, learning: {}", student1.name, student1.age, student1.is_learning);

    // ============================================
    // ENUM EXAMPLE
    // Enum = IS one of these options (OR)
    // A status can only be Active OR Inactive OR Pending
    // ============================================
    let current_status = Status::Active;
    
    // Use match to check which variant it is
    match current_status {
        Status::Active => println!("Status: Active!"),
        Status::Inactive => println!("Status: Inactive"),
        Status::Pending => println!("Status: Pending..."),
    }
}

// Function that BORROWS (doesn't take ownership)
fn print_borrowed(s: &String) {
    println!("Borrowed: {}", s);
}

// Function that adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Mutable borrowing allows a function to modify a value without taking ownership
fn mutable_borrow(s: &mut String) {
    s.push_str(" is coming");
}
