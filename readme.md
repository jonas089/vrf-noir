# Verifiable randomness with zero knowledge

This library enables the generation and verification of random parameters based on a constrained input.
For a pos consensus network the input may be constrained to a timestamp in a certain range `t0 < t < t1` appended by a random `nonce`.
The verifier contains the unique random output and can be used, together with the proof, to run the verification against the `Noir` circuit.

# Test setup
Edit the `.env` file to point to the global path of the `circuit` directory on your machine and the absolute path to the nargo binary. Use either `nargo-darwin` or `nargo-linux`, depending on your system architecture.

To enter the nix development shell for `vrf-rust`:
```
nix develop
```

To run the tests and see the output:
```
cargo test -- --nocapture

```

# Usage with ecdsa-circuit-input-lib

My [ecdsa-circuit-input-lib](https://github.com/jonas089/ecdsa-circuit-input-lib) provides functionality to generate elliptic curve keypairs, sign messages and export the public and private key in a format that is accepted by the randomness generator/ circuit.

From the `example` crate:

```rust
use vrf_rust::nargo::VerifiableRandomGenerator;
use ecdsa_circuit_input_lib::{keys::ecdsa::EcdsaKeyManager, core::signatures::{InputGenerator, Inputs}, db::StoreManager};
use std::path::PathBuf;
use serde_json;
fn main(){
    ... // load key from .db
    let key_manager: EcdsaKeyManager = EcdsaKeyManager{
        slice: key_serialized
    };
    // generate circuit inputs
    let input_generator = InputGenerator{
        sk: key_manager.deserialize(),
        message: hashed_nonce
    };
    let inputs = input_generator.generate();
    // initialize the random generator from a noir binary and specify the circuit location
    let random_generator = VerifiableRandomGenerator{
        bin: PathBuf::from("/users/chef/Desktop/vrf-noir/vrf-rust/bin/nargo-darwin"),
        circuit: PathBuf::from("/users/chef/Desktop/vrf-noir/circuit")
    };
    // generate a proof and obtain the verifiable random value
    let proof: vrf_rust::types::Proof = random_generator.generate(inputs.message, inputs.x, inputs.y, inputs.signature);
    // output the random value
    println!("Verifiable random value: {:?}", &proof.verifier);
    // verify the integrity of the generation of the random parameter:
    let is_valid: bool = random_generator.verify(&proof.proof, &proof.verifier);
    if is_valid == true{
        println!("The random value was verified successfully!")
    }
    else{
        println!("The random value could not be verified!")
    }
}
```

When generating inputs for the randomness generator, the `message` that's to be signed corresponds to the seed of the randomness operation. This could, for example, be a `timestamp` with or without a `nonce`. The range of valid inputs depends on the needs of the system and invalid inputs can be rejected by the system when verifying the proof for the random parameter.


