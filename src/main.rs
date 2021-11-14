fn main() {
    match cicada_core::start() {
        Err(error) => panic!("Shutdown failed: {}", error),
        _ => println!("\nBye!")
    }
}
