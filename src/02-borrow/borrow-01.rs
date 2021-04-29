#[allow(unused)]
fn main() {
    // A Stringünk a hello valakié lesz.
    let hello = String::from("Hello, Rust Hungary!");

    // Ezúttal csak kölcsönadjuk, méghozzá Immutable módon.
    // Ezt jelzi a & szimbólum.
    // Szakszavakkal: nem érték, hanem referencia szerint adjuk át.
    print_string(&hello);
    
    // Majd utána megpróbáljuk használni az eredeti valakinél.
    println!("{}", hello);
}

fn print_string(s: &String) {
    println!("{}", s);
}
