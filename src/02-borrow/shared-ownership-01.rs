use std::rc::Rc;

// Rc, azaz Reference Counted.
// Az Rc egy olyan, klónozható típus, melynek minden klónja
// ugyanarra az értékre tartalmaz egy mutatót.
// Minden klónozás növel egy számlálót, míg minden megszűnés
// csökkenti azt.
// Ha a számláló nullára csökken, akkor a tartalmazott
// érték is megszűnik.
#[allow(dead_code)]
struct Greeting {
    hello: Rc<String>
}

#[allow(unused)]
fn main() {
    // 1 db.
    let hello = Rc::new(String::from("Hello, Rust Hungary!"));

    // 2 db.
    let greeting1 = Greeting {
        hello: Rc::clone(&hello)
    };
    
    // 3 db.
    let greeting2 = Greeting {
        hello: Rc::clone(&hello)
    };
    
    println!("{} darab van belőlem.", Rc::strong_count(&hello));
}
