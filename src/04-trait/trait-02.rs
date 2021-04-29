trait Default {
    // Van default() a standard könyvtárban, amit ráadásul
    // implementál az u64, ezért nevet kell változtatnunk.
    fn not_std_default() -> Self;
}

// Az u64 típus egy beépített típus, nem a sajátunk.
// Azaz, másnak a kódjához adhatunk hozzá új viselkedést!
impl Default for u64 {
    fn not_std_default() -> Self {
        69
    }
}

fn main() {
    println!("{}", u64::not_std_default());
}
