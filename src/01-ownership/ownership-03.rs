struct Gone;

impl Drop for Gone {
    fn drop(&mut self) {
        println!("Ennyi volt, mese volt.");
    }
}

#[allow(unused)]
fn main() {
    // Csak a móka kedvéért nyitunk egy új scope-ot.
    {
        // A Gone {} érték a first változóé lesz,
        // ahogy a first kimegy a scope-ból, a hozzá rendelt
        // érték is távozik, felszabadul.
        let first = Gone {};
    }
}
