use std::cell::RefCell;
use std::rc::Rc;

// Szeretnénk egy olyan, módosítható valamit, aminek
// nem egészen tisztázott az élettartama, és megosztható
// több valaki között.
//
// A megosztást és módosítást már megoldottuk a RefCell-lel,
// míg a nem tisztázott élettartamot (megosztott oownershipet)
// Rc-vel. Egyszerűen csak össze kell raknunk a kettőt.
// 
// Két sorrend van Rc<RefCell> vagy RefCell<Rc>. Mivel
// módosítható értéket szeretnénk megosztani, és nem "megosztást módosítani",
// ezért az Rc<RefCell> lesz a nyerő.
#[allow(dead_code)]
struct Greeting {
    hello: Rc<RefCell<String>>
}

#[allow(unused)]
fn main() {
    let hello = Rc::new(RefCell::new(String::from("Hello, Rust Hungary!")));

    let greeting = Greeting {
        hello: Rc::clone(&hello)
    };

    greeting.hello.borrow_mut().push_str(" Tényleg ugyanazt módosítottuk!");
    
    println!("{}\n{}", hello.borrow(), greeting.hello.borrow());
}
