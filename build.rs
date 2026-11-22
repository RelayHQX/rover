fn main() {
    uniffi::generate_scaffolding("./src/rover.udl").expect("Building UniFFI scaffolding failed");
}
