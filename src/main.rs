use sha2::{Sha256, Digest};

fn main() {
    // Input string to hash
    let input = "Hello, Cryptography!";

    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Write input data
    hasher.update(input);

    // Read hash digest and consume hasher
    let result = hasher.finalize();

    println!("Hash of '{}': {:?}", input, result);
}
