fn greeting() -> &String {
    // A "Hello, Rust Hungary" String tulajdonosa a hello változó.
    // Amikor a hello scope-ja lejár (a függvény visszatér), akkor
    // a hellohoz rendelt érték is megszűnik.
    let hello = String::from("Hello, Rust Hungary!");
    
    // Na, erre az értékre mutató referenciát
    // szeretnénk mi visszaadni, azaz ezt az
    // értéket kölcsönözni.
    &hello
}

#[allow(unused)]
fn main() {
    let hello = greeting();
    
    println!("{}", hello);
}
