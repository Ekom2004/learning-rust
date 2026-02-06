// ============================================
// 🦀 LESSON 7: Traits (Interfaces)
// ============================================
// What is a Trait?
// A Trait is a "Contract". It defines BEHAVIOR that different types can share.
// Think of it like a Java Interface.
//
// Why use them?
// So you can write ONE function that works on MANY types (Polymorphism).

// 1. Define the Trait (The Contract)
pub trait Summary {
    // Anyone implementing Summary MUST create this function
    fn summarize(&self) -> String;

    // Default implementation (optional to override)
    fn announce(&self) {
        println!("(Default Announcement): {}", self.summarize());
    }
}

// 2. Define some Structs
struct Tweet {
    username: String,
    content: String,
}

struct NewsArticle {
    headline: String,
    author: String,
}

// 3. Implement the Trait for the Structs
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

// 4. Use the Trait as a Parameter (Polymorphism!)
// This function accepts ANYTHING that implements Summary
fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("ekom"),
        content: String::from("Rust traits are cool!"),
    };

    let article = NewsArticle {
        headline: String::from("Rust takes over the world"),
        author: String::from("The Times"),
    };

    // Both work passing to notify()!
    notify(&tweet);
    notify(&article);

    // Calling the default method
    println!("\n--- Default Implementation ---");
    tweet.announce();
    article.announce();
}

// ============================================
// CHALLENGE:
// 1. Create a `struct Email { from: String, subject: String }`
// 2. Implement `Summary` for `Email`
// 3. Pass it to `notify()`
// ============================================
