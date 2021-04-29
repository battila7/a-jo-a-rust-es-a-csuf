#[allow(unused)]
fn main() {
    // A Stringünk a hello valakié lesz.
    // Explicit jeleznünk kell, hogy tervezzük Mutable módon
    // kölcsönadni.
    // Nem a változó lesz mutable, hanem maga az érték.
    let mut hello = String::from("Hello, Rust Hungary!");

    // Ezúttal Mutable módon adjuk kölcsön, amit a &mut jelez.
    append_to_greeting(&mut hello);
    
    // Majd utána megpróbáljuk használni az eredeti valakinél.
    println!("{}", hello);
}

// A szignatúrának világos része, hogy ez a függvény Mutable
// referenciát vár.
fn append_to_greeting(s: &mut String) {
    s.push_str(" Ez itt A jó a Rust és a csúf!");
}
