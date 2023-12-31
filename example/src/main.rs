use vrf_rust::nargo::VerifiableRandomGenerator;
use ecdsa_circuit_input_lib::{keys::ecdsa::EcdsaKeyManager, core::signatures::{InputGenerator, Inputs}, db::StoreManager};
use std::path::PathBuf;
use dotenv::dotenv;
use std::env;
fn main(){
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
}