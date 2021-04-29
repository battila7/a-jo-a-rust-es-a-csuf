#[allow(unused)]
fn main() {
    // A Stringünk a hello valakié lesz.
    let hello = String::from("Hello, Rust Hungary!");

    // Újra Immutable módon adjuk kölcsön.
    append_to_greeting(&hello);
    
    // Majd utána megpróbáljuk használni az eredeti valakinél.
    println!("{}", hello);
}

fn append_to_greeting(s: &String) {
    // A push_str hozzá szeretne fűzni az eredeti Stringhez.
    // Azaz, módosítja azt.
    s.push_str(" Ez itt A jó a Rust és a csúf!");
}
