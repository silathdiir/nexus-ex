# nexus-zkvm Documentation

## Summary

- The VM code is open source and has an opening TG group (I reviewed the previous discussions and got some answers).
- Based on Rust RISC target: `rustup target add riscv32i-unknown-none-elf`.
- Has two Nova implementations of the prover: sequential (IVC) and parallel (PCD) (has an example in the benchmark section below). It has CLI code (tools/src/command/) for both prover and verifier, in general it could load any rv32i-elf file by file path (even compiled from C). It should support recursion, I found code of recursive-snark in both IVC and PCD.
- Could configure `k` number of VM instructions in each proving step. Increasing it reduces the number of proving steps (may reduce the proving time, check benchmark below), but increases the final proof size.
- Has a reduced instruction set of RISC, will support extension circuits (as keccak) via a concept of VM precompiles, otherwise it's slow to prove keccak in VM (lots of memory lookup constaints: read, write and update). From what I see in the current code, it's under developing as this [Precompile issue for Poseidon](https://github.com/nexus-xyz/nexus-zkvm/issues/109). I think it may be Nova custom circuit, there's a [Doc issue](https://github.com/nexus-xyz/nexus-zkvm/issues/112) about how to use it directly (not resolved, just looking into some code in progress).
![nvm-coprocessors](https://github.com/nexus-xyz/nexus-zkvm/blob/main/docs/public/images/nvm-coprocessors.svg)

## Examples

I add the current examples to [nexus-ex](https://github.com/silathdiir/nexus-ex) repo. They're all app code for now, none is the custom circuit (will try to add one for testing).

### SQL Parser

The code is [parse_sql.rs](https://github.com/silathdiir/nexus-ex/blob/main/src/parse_sql.rs):
```
#[nexus_rt::main]
fn main() {
    let sql = "SELECT a, b FROM table_1";

    let dialect = GenericDialect {};

    // Debugged in it, and found that it's failed to parse SQL tokens.
    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("AST: {:?}", ast);
}
```

It's a simple example to parse a SQL string, but it's failed with an error:
```
make run bin=parse_sql

Error: misaligned memory access 43454c45
```
The code could work in a normal Rust project, I try to debug and found it may be caused by Rust macro for parsing SQL tokens, submit an [issue-142](https://github.com/nexus-xyz/nexus-zkvm/issues/142).

### Multiple Crates

The code is [utils.rs](https://github.com/silathdiir/nexus-ex/blob/main/src/utils.rs):
```
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
```

I test multiple Rust crates in this code, they must be build with `nostd` (default-features = false):
```
[dependencies]
anyhow = { version = "1.0", default-features = false }
hex = { version = "0.4", default-features = false, features = ["alloc"] }
nexus-rt = { git = "https://github.com/nexus-xyz/nexus-zkvm.git" }
rand = { version = "0.8", default-features = false, features = ["alloc", "small_rng"] }
# Cannot build
# rlp = { version = "0.5", default-features = false }
serde = { version = "1.0", default-features = false }
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
sha3 = { version = "0.10", default-features = false }
```

The `rlp` crate cannot be build, since its dependency `bytes` cannot be build on RISC target.
The `rand` crate is highly depended on OS, it could only use `SmallRng`:
```
let test_seed = 100;
let mut small_rng = SmallRng::seed_from_u64(test_seed);
```

This example could run, prove and verify successfully:
```
make run bin=utils
make prove bin=utils
make verify bin=utils
```

### Keccak

The code is [keccak.rs](https://github.com/silathdiir/nexus-ex/blob/main/src/keccak.rs), I copy it from the [nexus-zkvm example](https://github.com/nexus-xyz/nexus-zkvm/blob/main/examples/src/bin/keccak.rs).

It's not the `precompile` version as this comment:
```
// This example shows how to compute keccak hashes in software. In
// practice, using a keccak "pre-compile" will be more efficient.
```

This example could run, prove and verify successfully. I will use it to do benchmark below:
```
make run bin=keccak
make prove bin=keccak
make verify bin=keccak
```

## Benchmark (VM)

I tried to run on an AWS, but it’s stuck sometimes. So I benchmark as below on my local Mac, and collect the below infos.

### Sequential Prover

Prove the keccak app (Proved 4095 step(s) in 93min22s; 11.70 instructions / second):
```
cargo nexus prove --bin keccak

  Setting up public parameters for IVC ... 14.4s
    Finished in 14.4s
Executing program...
0510000000000c85000bbfd5e133f14cb355c3fd8d99367964f
Executed 65520 instructions in 3.497743709s. 25814920 bytes used by trace.
  Loading public parameters ... 3.9s
 Finished in 3.9s
  Computing step 0 ... 795ms
  Computing step 1 ... 914ms
  Computing step 2 ... 1.1s
...
  Computing step 4090 ... 1.4s
  Computing step 4091 ... 1.4s
  Computing step 4092 ... 1.4s
  Computing step 4093 ... 1.4s
  Computing step 4094 ... 1.4s
     Proved 4095 step(s) in 93min22s; 11.70 instructions / second
  Saving proof ... 53ms
Finished in 53ms
```

Verify the generated proof:
```
cargo nexus verify

  Loading public parameters ... 3.9s
 Finished in 3.9s
  Verifying proof ... 1.3s
   Finished in 1.3s
```

### Parallel Prover

#### With default k (number of VM instructions in each proving step)

```
cargo nexus prove --bin keccak --impl=par
```

```
cargo nexus verify --impl=par
```

#### With k = 128

```
cargo nexus prove --bin keccak --impl=par -k=128
```

```
cargo nexus verify --impl=par -k=128
```

#### With k = 8

```
cargo nexus prove --bin keccak --impl=par -k=8
```

```
cargo nexus verify --impl=par -k=8
```

## Benchmark (Nova API)

https://github.com/silathdiir/nexus-nova-ex
benches [Microsoft Nova](https://github.com/microsoft/Nova), [nexus-zkvm](https://github.com/nexus-xyz/nexus-zkvm) Nova and Super Nova with a test Poseidon circuit:
```bash
# Microsoft Nova
ms-nova-11/Prove        time:   [51.772 ms 52.178 ms 52.798 ms]
ms-nova-11/Verify       time:   [45.647 ms 45.808 ms 46.082 ms]

# Nexus Nova
nexus-nova-11/Prove     time:   [309.96 ms 311.28 ms 312.65 ms]
nexus-nova-11/Verify    time:   [259.73 ms 262.18 ms 264.83 ms]

# Nexus Super Nova, no much different with Nexus Nova (for uniform circuit)
nexus-supernova-11/Prove
                        time:   [332.21 ms 345.33 ms 360.69 ms]
nexus-supernova-11/Verify
                        time:   [279.89 ms 286.91 ms 294.71 ms]
```

For now the Nexus Nova and Super Nova are slower than Microsoft Nova, similar as the benchmark results in https://github.com/nexus-xyz/nexus-zkvm/tree/main/nova-benches.

## TODO

- This is the [paper of CCS circuits](https://eprint.iacr.org/2023/552), which is used to implement its VM Precompile. The code of [CCS PR](https://github.com/nexus-xyz/nexus-zkvm/pull/52) has been merged, so it should support its VM Precompile. Check if any code available.

- The [Proof compression](https://docs.nexus.xyz/Specs/nexus-prover#proof-compression) mentions that it uses a variant of PSE halo2 based on KZG PCS to implement the first part of compression, and will use Groth16 to implement the second part. Need to check that part of code.

