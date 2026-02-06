// ============================================
// 🦀 LESSON 6: File I/O
// ============================================
// How to read and write files in Rust.
// Docs: https://doc.rust-lang.org/std/fs/

use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    println!("🦀 Lesson 6: File I/O\n");

    // ==========================================
    // PART 1: Writing Files
    // ==========================================
    
    let content = "Hello from Rust!\nThis file was created by code.";
    let filename = "lesson6_output.txt";

    // Write string to file (overwrites if exists)
    // Returns Result<(), io::Error>
    fs::write(filename, content)?; 
    println!("✅ Written to '{}'", filename);

    // ==========================================
    // PART 2: Reading Files (Text)
    // ==========================================
    
    // Read entire file as String
    let text_content = fs::read_to_string(filename)?;
    
    println!("\n--- File Content (Text) ---");
    println!("{}", text_content);
    
    // ==========================================
    // PART 3: Reading Files (Binary)
    // ==========================================
    
    // Read entire file as Bytes (Vec<u8>)
    // This is what MX8 will use for images/videos/npy
    let byte_content = fs::read(filename)?;
    
    println!("\n--- File Content (Bytes) ---");
    println!("Read {} bytes", byte_content.len());
    println!("First 10 bytes: {:?}", &byte_content[0..10]); // Slicing!

    // ==========================================
    // PART 4: Handling Errors
    // ==========================================
    
    println!("\n--- Error Handling ---");
    
    // Try to read a missing file
    match fs::read_to_string("missing_file.txt") {
        Ok(_) => println!("Found it!"),
        Err(e) => println!("Could not read file: {}", e),
    }

    // ==========================================
    // CLEANUP
    // ==========================================
    
    // Delete the file we created
    fs::remove_file(filename)?;
    println!("\n✅ Deleted '{}'", filename);

    Ok(())
}
