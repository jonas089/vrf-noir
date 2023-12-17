use std::{fs::create_dir, path::PathBuf};
use tempfile::tempdir;
use std::{fs, fs::File};
use std::{io::Write, io::Read};
use std::process::Command;
use crate::types::Proof;

pub struct VerifiableRandomGenerator{
    pub bin: PathBuf,
    pub circuit: PathBuf
}

impl VerifiableRandomGenerator{
    pub fn generate(&self, nonce: Vec<u8>, x: Vec<u8>, y: Vec<u8>, signature: Vec<u8>) -> Proof{
        let temp_dir = tempdir().unwrap();
        let temp_dir = temp_dir.path().to_path_buf();
        let temp_src = temp_dir.join("src");
        create_dir(&temp_src).unwrap();
        // copy the entire circuit source
        for script in fs::read_dir(&self.circuit.join("src")).unwrap(){
            let script_unwrapped = script.unwrap();
            let script_path = &script_unwrapped.path();
            let destination_path = &temp_src.join(&script_unwrapped.file_name());
            match fs::copy(&script_path, &destination_path){
                Err(msg) => panic!("Failed to copy script! \n Code: {:?}", msg),
                Ok(_) => {}
            };
        };
        // copy the Nargo.toml (circuit config file)
        let temp_nargo_toml = &temp_dir.join("Nargo.toml");
        match fs::copy(&self.circuit.join("Nargo.toml"), temp_nargo_toml){
            Err(msg) => panic!("Failed to copy Nargo.toml! \n Code: {:?}", msg),
            Ok(_) => {}
        }
        // create the prover file from params
        let mut prover = match File::create(&temp_dir.join("Prover.toml")) {
            Err(msg) => panic!("{:?}", msg),
            Ok(file) => file,
        };
        // write the params line-by-line
        writeln!(prover, "nonce = {:?}", nonce).unwrap();
        writeln!(prover, "x = {:?}", x).unwrap();
        writeln!(prover, "y = {:?}", y).unwrap();
        writeln!(prover, "signature = {:?}", signature).unwrap();
        // generate the proof -> this will create the vrf.proof file in proofs/
        let prove = Command::new(&self.bin)
        .arg("prove")
        .arg("--workspace")
        .current_dir(&temp_dir.to_str().unwrap())
        .output()
        .unwrap();

        let verifier: String = std::fs::read_to_string(&temp_dir.join("Verifier.toml")).unwrap();
        let proof: String = std::fs::read_to_string(&temp_dir.join("proofs").join("vrf.proof")).unwrap();
        if prove.status.success(){
            println!("Proof was generated!");
        }
        else{
            eprintln!("Failed to generate proof!");
        }
        Proof{
            proof: proof,
            verifier: verifier
        }
    }
    pub fn verify(&self, proof: &str , verifier: &str) -> bool{
        let temp_dir = tempdir().unwrap();
        let temp_dir = temp_dir.path().to_path_buf();
        let temp_src = temp_dir.join("src");
        create_dir(&temp_src).unwrap();
        // copy the entire circuit source
        for script in fs::read_dir(&self.circuit.join("src")).unwrap(){
            let script_unwrapped = script.unwrap();
            let script_path = &script_unwrapped.path();
            let destination_path = &temp_src.join(&script_unwrapped.file_name());
            match fs::copy(&script_path, &destination_path){
                Err(msg) => panic!("Failed to copy script! \n Code: {:?}", msg),
                Ok(_) => {}
            };
        };
        // copy the Nargo.toml (circuit config file)
        let temp_nargo_toml = &temp_dir.join("Nargo.toml");
        match fs::copy(&self.circuit.join("Nargo.toml"), temp_nargo_toml){
            Err(msg) => panic!("Failed to copy Nargo.toml! \n Code: {:?}", msg),
            Ok(_) => {}
        }
        // write the proof and run the verify function
        let temp_proofs = temp_dir.join("proofs");
        create_dir(&temp_proofs).expect("Failed to create temp/proofs!");
        let mut proof_file = match File::create(temp_proofs.join("vrf.proof")) {
            Err(msg) => panic!("{:?}", msg),
            Ok(file) => file,
        };
        proof_file.write_all(&proof.as_bytes()).expect("Failed to write proof!");
        // empty verifier
        let mut verifier_file = match File::create(&temp_dir.join("Verifier.toml")) {
            Err(msg) => panic!("{:?}", msg),
            Ok(file) => file,
        };
        verifier_file.write_all(&verifier.as_bytes()).expect("Failed to write verifier!");
        // verify the proof
        let verify = Command::new(&self.bin)
        .arg("verify")
        .arg("--workspace")
        .current_dir(&temp_dir.to_str().unwrap())
        .output()
        .unwrap();

        if verify.status.success(){
            true
        }
        else{
            let error = String::from_utf8_lossy(&verify.stderr);
            false
        }  
    }
}

#[test]
fn test_generator_with_verification(){
    use dotenv::dotenv;
    use std::env;
    use std::path::PathBuf;

    let nonce: Vec<u8> = vec![192, 246, 228, 6, 21, 169, 84, 198, 106, 131, 91, 200, 25, 216, 99, 78, 85, 119, 218, 18, 83, 37, 190, 122, 23, 73, 170, 28, 245, 215, 101, 184];
    let x: Vec<u8> = vec![42, 20, 27, 7, 166, 238, 115, 118, 231, 70, 250, 155, 101, 211, 192, 140, 19, 27, 144, 177, 226, 5, 17, 160, 24, 56, 8, 156, 29, 165, 234, 121];
    let y: Vec<u8> = vec![83, 7, 164, 2, 157, 234, 12, 147, 193, 122, 238, 77, 240, 96, 153, 248, 232, 84, 4, 37, 135, 204, 5, 238, 210, 29, 134, 226, 211, 89, 183, 124];
    let signature: Vec<u8> = vec![218, 19, 71, 182, 206, 141, 138, 47, 62, 31, 216, 31, 145, 217, 24, 9, 87, 19, 41, 243, 185, 138, 4, 102, 50, 39, 153, 89, 173, 157, 229, 185, 97, 227, 171, 66, 192, 26, 236, 25, 34, 198, 97, 233, 152, 243, 74, 250, 133, 239, 114, 146, 239, 214, 240, 26, 206, 195, 205, 137, 135, 236, 31, 65];
    dotenv().ok();
    let bin: PathBuf = PathBuf::from(env::var("DEFAULT_NARGO_BINARY_PATH").expect("Failed to get DEFAULT_NARGO_BINARY_PATH from env!"));
    let circuit: PathBuf = PathBuf::from(env::var("DEFAULT_ABSOLUTE_CIRCUIT_PATH").expect("Failed to get DEFAULT_ABSOLUTE_CIRCUIT_PATH from env!"));
    let vrf: VerifiableRandomGenerator = VerifiableRandomGenerator{
        bin: bin,
        circuit: circuit
    };
    let proof: Proof = vrf.generate(nonce, x, y, signature);
    assert!(vrf.verify(&proof.proof, &proof.verifier) == true);
}