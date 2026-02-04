// ============================================
// 🦀 LESSON 2: Ownership - The Heart of Rust
// ============================================
// To run this lesson, rename this file to main.rs
// (and rename the current main.rs to lesson_1.rs)
// Or copy contents to main.rs

#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    println!("🦀 Lesson 2: Ownership\n");
    
    // ==========================================
    // THE 3 RULES OF OWNERSHIP
    // ==========================================
    // 1. Each value has exactly ONE owner
    // 2. When the owner goes out of scope, the value is dropped
    // 3. There can only be ONE owner at a time
    
    // ==========================================
    // PART 1: Move Semantics
    // ==========================================
    
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2
    
    // println!("{}", s1);  // ❌ ERROR! s1 is no longer valid
    println!("s2 = {}", s2);  // ✅ s2 owns the string now
    
    // WHY? Rust avoids "double free" bugs by making s1 invalid
    // This is different from most languages!
    
    // ==========================================
    // PART 2: Clone (Deep Copy)
    // ==========================================
    
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Explicit deep copy
    
    println!("s1 = {}, s2 = {}", s1, s2);  // ✅ Both valid!
    
    // Note: clone() can be expensive for large data
    
    // ==========================================
    // PART 3: Copy Types (Stack Only)
    // ==========================================
    
    // Simple types that live on the stack implement Copy
    let x = 5;
    let y = x;  // x is COPIED, not moved
    
    println!("x = {}, y = {}", x, y);  // ✅ Both valid!
    
    // Copy types: integers, floats, bools, chars, tuples of Copy types
    // NOT Copy: String, Vec, any heap-allocated type
    
    // ==========================================
    // PART 4: Functions and Ownership
    // ==========================================
    
    let s = String::from("hello");
    takes_ownership(s);  // s is MOVED into the function
    
    // println!("{}", s);  // ❌ ERROR! s was moved
    
    let x = 5;
    makes_copy(x);  // x is COPIED into the function
    println!("x is still valid: {}", x);  // ✅ x is still valid
    
    // ==========================================
    // PART 5: Returning Ownership
    // ==========================================
    
    let s1 = gives_ownership();  // Function returns ownership to s1
    println!("Got ownership: {}", s1);
    
    let s2 = String::from("hello");
    takes_and_gives_back(&s2);  // just borrowing
    // s2 is still valid
    println!("Gave and got back: {}", s2);
    
    // ==========================================
    // PART 6: References & Borrowing
    // ==========================================
    
    println!("\n--- References & Borrowing ---\n");
    
    let s1 = String::from("hello");
    
    // & creates a REFERENCE (borrow without taking ownership)
    let len = calculate_length(&s1);  // We "borrow" s1
    
    println!("The length of '{}' is {}", s1, len);  // ✅ s1 still valid!
    
    // ==========================================
    // PART 7: Mutable References
    // ==========================================
    
    let mut s = String::from("hello");
    
    change(&mut s);  // Mutable borrow
    println!("After change: {}", s);
    
    // ⚠️ RULE: You can have EITHER:
    //    - ONE mutable reference, OR
    //    - ANY number of immutable references
    //    - But NOT both at the same time!
    
    // This prevents data races at compile time!
    
    // ==========================================
    // PART 8: The Rules Visualized
    // ==========================================
    
    let mut s = String::from("hello");
    
    let r1 = &s;     // ✅ First immutable borrow
    let r2 = &s;     // ✅ Second immutable borrow (allowed)
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // ✅ Mutable borrow (after immutable borrows are done)
    println!("{}", r3);
    
    // This would FAIL:
    // let r1 = &s;
    // let r2 = &mut s;  // ❌ Can't borrow as mutable while immutable exists
    // println!("{}, {}", r1, r2);
    
    println!("\n🎉 Lesson 2 Complete!");
    println!("This is the MOST IMPORTANT concept in Rust!");
    println!("Practice until you can predict what compiles and what doesn't.");
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

fn takes_ownership(s: String) {
    println!("I own this now: {}", s);
}  // s goes out of scope and is dropped

fn makes_copy(x: i32) {
    println!("I got a copy: {}", x);
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s  // Ownership is returned
}

fn takes_and_gives_back(s: String) -> String {
    s  // Return the same string, transferring ownership back
}

fn calculate_length(s: &String) -> usize {  // s is a REFERENCE
    s.len()
}  // s goes out of scope, but since it doesn't own the data, nothing is dropped

fn change(s: &mut String) {  // MUTABLE reference
    s.push_str(", world!");
}
