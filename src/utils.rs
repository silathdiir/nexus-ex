#![no_std]
#![no_main]

use anyhow::anyhow;
use nexus_rt::{println, Write};
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use serde_json::Value;
use sha3::{Digest, Keccak256};

#[nexus_rt::main]
fn main() {
    // Test anyhow.
    println!("Anyhow error: {:?}", anyhow!("Test error"));

    // Test hex.
    let s = hex::encode("Hello world!");
    assert_eq!(s, "48656c6c6f20776f726c6421");
    println!("Hex: {s}");

    // Test rand.
    let test_seed = 100;
    let mut small_rng = SmallRng::seed_from_u64(test_seed);
    let rand_num = small_rng.next_u64();
    println!("Rand U64: {rand_num}");

    // Test sha3.
    let mut hasher = Keccak256::new();
    hasher.update([1, 2, 3, 4]);
    let output = hasher.finalize().to_vec();
    println!("Sha3 output: {output:?}");

    // Test serde_json.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let value: Value = serde_json::from_str(data).unwrap();
    println!("Serde json: {value}");
}
