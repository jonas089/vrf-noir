# Verifiable randomness with zero knowledge

This library is a cryptographic sub-protocol that can be used as a verifiable random number generator (vrf). For many projects this zero knowledge implementation is overkill and only projects that require the obfuscation of certain input parameters to the verifiable pseudorandom number generator may benefit from it. When looking to utilize verifiable randomness on-chain, you're better off with an implementation like Chainlink's VRF. Chainlink's VRF simply signs a message appended by a nonce and therefore produces a random hash output. While this may seem similar to the content of the `noir` circuit designed for the scope of this VRF subprotocol, it works very differently. This VRF generates random numbers based on a set of *inputs*:

```
    x: the x-coordinate of a public key
    y: the y-coordinate of a public key
    nonce: a hashed seed message
    signature: a signature over a hashed seed message
```
The random number that is being generated is the only output of this protocol and is always *public*.
The `x`, `y` and `nonce` inputs can be either *public* or *private* (obfuscation of inputs). 
For most use cases this protocol is overkill and a simple signature over a nonce will be sufficient. 

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
    dotenv().ok();
    let bin: PathBuf = PathBuf::from(env::var("DEFAULT_NARGO_BINARY_PATH").expect("Failed to get DEFAULT_NARGO_BINARY_PATH from env!"));
    let circuit: PathBuf = PathBuf::from(env::var("DEFAULT_ABSOLUTE_CIRCUIT_PATH").expect("Failed to get DEFAULT_ABSOLUTE_CIRCUIT_PATH from env!"));

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
        println!("Verifiable random value: {:?}", &proof.get_random_number(false, true, true));
    
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

The `get_random_number` method takes 3 parameters. Each parameter tells whether an inputs is *true* => *public* or *false* => *private*. 
Which inputs are public and which inputs are *private* will depend on the inputs to the circuit in `main.nr`:

```rust
fn main(nonce: pub [u8;32], x: [u8;32], y: pub [u8;32], signature: [u8;64]) -> pub [u8;32] {
    ...
}
```
by default, `nonce` and `y` are *private* and `x` is *public*.

Inside the `get_random_number` function, a *Verifier* is constructed (from the actual `Verifier.toml` file generated by the `noir` prover):

```rust
pub struct Verifier{
    pub nonce: Option<Vec<String>>,
    pub output: Vec<String>,
    pub x: Option<Vec<String>>,
    pub y: Option<Vec<String>>
}
```

Should the public inputs be required, the `get_random_number` function could be modified to return the `Verifier` instance alongside the random number that it returns by default.