// ============================================
// 🦀 WHY DO WE NEED OPTION?
// ============================================
// Real-world scenarios where things might be missing.

struct UserProfile {
    username: String,
    // Scenario 1: Optional Input
    // Not everyone wants to provide their age or phone number.
    // Use Option<T> to represent "maybe they gave it, maybe they didn't".
    age: Option<i32>, 
}

fn main() {
    println!("=== Scenario 1: Optional Data ===");
    
    let alice = UserProfile {
        username: String::from("Alice"),
        age: Some(30), // Alice provided her age
    };

    let bob = UserProfile {
        username: String::from("Bob"),
        age: None, // Bob prefers not to say
    };

    print_age(&alice);
    print_age(&bob);


    println!("\n=== Scenario 2: Finding things in a List ===");
    // Imagine a game inventory or a database search.
    // If you search for something, it might NOT be there.
    
    let inventory = vec!["Sword", "Shield", "Potion"];
    
    // .get(index) returns an Option because indices can be out of bounds!
    let item_at_index_1 = inventory.get(1); // Some("Shield")
    let item_at_index_99 = inventory.get(99); // None (Safe! No crash!)

    println!("Item 1: {:?}", item_at_index_1);
    println!("Item 99: {:?}", item_at_index_99);


    println!("\n=== Scenario 3: Empty Containers ===");
    // Removing items from a list might fail if the list is empty.
    
    let mut deck_of_cards = vec!["Ace of Spades"];
    
    // First draw
    let card1 = deck_of_cards.pop(); // Some("Ace of Spades")
    println!("Drew: {:?}", card1);
    
    // Second draw (Deck is empty!)
    let card2 = deck_of_cards.pop(); // None
    println!("Drew: {:?}", card2);
}

fn print_age(user: &UserProfile) {
    match user.age {
        Some(age) => println!("{} is {} years old.", user.username, age),
        None => println!("{}'s age is a mystery.", user.username),
    }
}
