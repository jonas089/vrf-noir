# Verifiable randomness with zero knowledge

This library is a cryptographic sub-protocol that can be used as a verifiable random number generator (vrf).
For a pos consensus network the input may be constrained to a timestamp in a certain range `t0 < t < t1` appended by a `nonce`. While the range of possible inputs is known to the network in advance, the random number output is not. This property is based on the assumption that the node's private key is not leaked to the network, as the private key is used to sign the public input inside the `noir` circuit.
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

# Build dependencies
Note that for 64-bit Debian / Ubuntu the `noir-linux` binary has an additional dependency:
```
sudo apt update
sudo apt-get install libc++1
```

For MacOS darwin systems, use the `noir-darwin` binary.

# Usage with ecdsa-circuit-input-lib

My [ecdsa-circuit-input-lib](https://github.com/jonas089/ecdsa-circuit-input-lib) provides functionality to generate elliptic curve keypairs, sign messages and export the public and private key in a format that is accepted by the randomness generator/ circuit.

From the `example` crate:

```rust
    ...
    // any valid seed that is used to generate the random value
    for i in 0..10{
        let hashed_nonce: Vec<u8> = vec![i;32];
        println!("Current nonce: {:?}", &hashed_nonce);
        let key_manger: EcdsaKeyManager = EcdsaKeyManager{
            slice: vec![]
        };
        let key_serialized: Vec<u8> = key_manger.new();
        let key_manager: EcdsaKeyManager = EcdsaKeyManager{
            slice: key_serialized
        };
        // generate circuit inputs
        let input_generator: InputGenerator = InputGenerator{
            sk: key_manager.deserialize(),
            message: hashed_nonce
        };
        let inputs: Inputs = input_generator.generate();
        // initialize the random generator from a noir binary and specify the circuit location
        let random_generator: VerifiableRandomGenerator = VerifiableRandomGenerator{
            bin: PathBuf::from(&bin),
            circuit: PathBuf::from(&circuit)
        };
        // generate a proof and obtain the verifiable random value
        let proof: vrf_rust::types::Proof = random_generator.generate(inputs.message, inputs.x, inputs.y, inputs.signature);
        // output the random value
        println!("Verifiable random value: {:?}", &proof.get_random_number());
    
        // verify the integrity of the generation of the random parameter:
        let is_valid: bool = random_generator.verify(&proof.proof, &proof.verifier);
        if is_valid == true{
            println!("The random value was verified successfully!")
        }
        else{
            println!("The random value could not be verified!")
        }
    }
    ...
```

When generating inputs for the randomness generator, the `message` that's to be signed corresponds to the seed of the randomness operation. This could, for example, be a `timestamp` with or without a `nonce`. The range of valid inputs depends on the needs of the system and invalid inputs can be rejected by the system when verifying the proof for the random parameter.

For a POS consensus protocol, it would, for example, be a reasonable decision to restrict the input to the timestamp + block hash of the proposed block, to ensure that every node can only produce one deterministic pseudorandom number.

