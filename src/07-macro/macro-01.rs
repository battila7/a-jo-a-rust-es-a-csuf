// Makróval.
#[derive(Default)]
struct Greeting {
    hello: String
}

// Makró nélkül.
struct NoMacroGreeting {
    hello: String
}

impl Default for NoMacroGreeting {
    fn default() -> Self {
        Self {
            hello: String::from("")
        }
    }
}

fn main() {
    let greeting = Greeting::default();
    let no_macro_greeting = NoMacroGreeting::default();
    
    println!("hello = \"{}\"", greeting.hello);
    println!("hello = \"{}\"", no_macro_greeting.hello);
}
