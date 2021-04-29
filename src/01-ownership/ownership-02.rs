#[allow(unused)]
fn main() {
    // A Stringünk a first valakié lesz.
    let first = String::from("Hello, Rust Hungary!");

    // Bár nem gondolnánk, de itt valójában odaadjuk
    // a print_string s paraméterének.
    print_string(first);
    
    // Majd utána megpróbáljuk használni az eredeti valakinél.
    println!("{}", first);
}

fn print_string(s: String) {
    println!("{}", s);
}
