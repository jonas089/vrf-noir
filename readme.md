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

