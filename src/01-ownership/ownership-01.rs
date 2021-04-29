#[allow(unused)]
fn main() {
    // A Stringünk a first valakié lesz.
    let first = String::from("Hello, Rust Hungary!");

    // Odaadjunk másnak is, secondnek.
    let second = first;

    // Majd utána megpróbáljuk használni az eredeti valakinél.
    println!("{}", first);
}
