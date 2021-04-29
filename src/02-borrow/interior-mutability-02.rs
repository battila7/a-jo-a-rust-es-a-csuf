use std::cell::RefCell;

struct Immutable {
    hello: RefCell<String>
}

#[allow(unused)]
fn main() {
    let immutable = Immutable {
        hello: RefCell::new(String::from("Helló, Rust Hungary!"))
    };
    
    // Egy új, elszeparált scope-ban látványosabb
    // a végeredmény.
    {
        let mut hello_borrow_1 = immutable.hello.borrow_mut();
        let mut hello_borrow_2 = immutable.hello.borrow_mut();
    }
}
