// 'a egy lifetime paraméter, ami azt jelenti, hogy
// a hello borrow legalább addig kell éljen, mint
// a tartalmazó struct.
struct Greeting<'a> {
    hello: &'a str
}

#[allow(unused)]
fn main() {
    let mut greeting = Greeting {
        hello: &"Hello, Rust Hungary!"
    };
    
    // Figyelem:
    // A haliho nevű változó csak ebben a scope-ban él,
    // míg a greeting nevű változó az egész main függvényben.
    {
        let haliho = String::from("Hili, hali, halihó!");
        
        greeting.hello = &haliho.as_str();
    }
    
    println!("{}", greeting.hello);
}
