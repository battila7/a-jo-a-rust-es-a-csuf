#[allow(unused)]
fn main() {
    let mut hello = String::from("Hello, Rust Hungary!");
    
    // Először Immutable módon kölcsönzünk.
    let hello_im = &hello;
    
    // Ezt követően pedig szeretnénk Mutable módon
    // kölcsönadni az értéket az append_greeting-nek.
    // A hello_im kölcsön természetesen továbbra is "él".
    append_to_greeting(&mut hello);
    
    println!("{}", hello_im);
}

fn append_to_greeting(s: &mut String) {
    s.push_str(" Ez itt A jó a Rust és a csúf!");
}
