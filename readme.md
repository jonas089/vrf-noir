# Verifiable randomness with zero knowledge

This library enables the generation and verification of random parameters based on a constrained input.
For a pos consensus network the input may be constrained to a timestamp in a certain range `t0 < t < t1` appended by a random `nonce`.
The verifier contains the unique random output and can be used, together with the proof, to run the verification against the `Noir` circuit.

# Test setup
Edit the `.env` file to point to the global path of the `circuit` directory on your machine and the relative path to the nargo binary. Use either `nargo-darwin` or `nargo-linux`, depending on your system architecture.

To run the tests and see the output:
```
cargo test -- --nocapture

```

# Usage with ecdsa-circuit-input-lib

My [ecdsa-circuit-input-lib](https://github.com/jonas089/ecdsa-circuit-input-lib) provides functionality to generate elliptic curve keypairs, sign messages and export the public and private key in a format that is accepted by the randomness generator/ circuit.

Find examples to generate and store a `keypair` [here](https://github.com/jonas089/ecdsa-circuit-input-lib/blob/master/src/lib.rs).

```rust
#[test]
fn generate_signature_circuit_inputs(){
    use ecdsa_circuit_input_lib::keys::ecdsa::EcdsaKeyManager;
    use ecdsa_circuit_input_lib::core::signatures::InputGenerator;
    use k256::{
        ecdsa::{SigningKey}, FieldBytes
    };
    // initialize keystore
    let store_manager: StoreManager = StoreManager{
        path: PathBuf::from("./keys.db")
    };
    // get key
    let key: db::Response = store_manager
        .get_key_by_uid("SOME_KEY_UID_0".to_string())
        .expect("[Error] Failed to get the key!");
    // deserialize SigningKey from Response object
    let key_slice: Vec<u8> = key.deserialize();
    let key_manager: EcdsaKeyManager = EcdsaKeyManager{
        slice: key_slice
    };
    // signing key ready for use with input generator
    let deserialized_signing_key = key_manager.deserialize();
    let message: Vec<u8> = vec![0;32];
    // initialize the input generator
    let input_generator = InputGenerator{
        sk: deserialized_signing_key,
        message: message
    };
    let inputs = input_generator.generate();
    println!("Circuit Inputs: {:?}", inputs);
}
```

When generating inputs for the randomness generator, the `message` that's to be signed corresponds to the seed of the randomness operation. This could, for example, be a `timestamp` with or without a `nonce`. The range of valid inputs depends on the needs of the system and invalid inputs can be rejected by the system when verifying the proof for the random parameter.


