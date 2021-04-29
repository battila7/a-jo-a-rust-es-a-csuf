trait Default {
    // A Self itt egy ismeretlen típus:
    // mindig az a típus, aki majd implementálja a trait-et.
    fn default() -> Self;
}

struct IpAddress {
    addr: String
}

impl Default for IpAddress {
    // A Self típus itt viszont már konkrét:
    // maga az IpAddress típus, hiszen ő implementálja
    // épp a Default traitet.
    fn default() -> Self {
        Self {
            addr: String::from("127.0.0.1")
        }
    }
}

fn main() {
    // A Javahoz képest ez 3021.
    let ip = IpAddress::default();
    
    println!("{}", ip.addr);
}
