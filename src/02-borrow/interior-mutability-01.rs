use std::cell::RefCell;

// A struct nem egyszerűen egy String típusú mezőt, hanem egy
// RefCell<String> típusú mezőt tartalmaz.
// A RefCell fogja elrejteni a "módosíthatóságot", ő akkor is
// módosítható, ha az egész structra csak Immutable referenciánk van.
struct Immutable {
    hello: RefCell<String>
}

#[allow(unused)]
fn main() {
    let immutable = Immutable {
        hello: RefCell::new(String::from("Helló, Rust Hungary!"))
    };
    
    // Vegyük észre, hogy Immutable borrow történik!
    append_to_greeting(&immutable);
    
    println!("{}", immutable.hello.borrow());
}

fn append_to_greeting(immutable: &Immutable) {
    // És az Immutable borrow ellenére, módosítani tudjuk
    // a structban tárolt Stringet.
    // Haha, ócska borrow checker, ezt nem teszed zsebre!
    let mut hello = immutable.hello.borrow_mut();
    hello.push_str(" Ez itt A jó a Rust és a csúf!");
}
