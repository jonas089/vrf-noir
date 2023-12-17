use std::{fs::create_dir, path::PathBuf};
use tempfile::tempdir;
use std::{fs, fs::File};
use std::{io::Write, io::Read};
use std::process::Command;
pub struct VerifiableRandomGenerator{
    pub bin: PathBuf,
    pub circuit: PathBuf
}

impl VerifiableRandomGenerator{
    pub fn generate(&self, nonce: Vec<u8>, x: Vec<u8>, y: Vec<u8>, signature: Vec<u8>){
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

        if prove.status.success(){
            println!("Proof was generated!");
        }
        else{
            eprintln!("Failed to generate proof!");
        }
    }
    pub fn verify(&self, proof: &str , verifier: &str){
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
            println!("Proof was verified!");
        }
        else{
            let error = String::from_utf8_lossy(&verify.stderr);
            eprintln!("Failed to verify proof: {:?}", &error);
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
    let proof = r#"226a56955bac7679dbdb071fa9b073d1747820ab778a6588f4e7f10fd2753e06215ce6b577c08ad91f35bfa4dce3edf64252ca566757cf5f8c25167054c3c7e90b6d66bddbff094d278d0f68e1da8e2b398ec14c38a5decd1c050f811f9e2c6923f9a135f1d1a7c8fa8e1d3d8956d1219fb5255d18b5b9dd66a1844a750f21d80f98fc19c6ff598e3a4cf44c177e55e96cc9b34d0a000883b30bacdbe7344ab91ba129425c595088adadeef35355c073a72a3a97ca5f09edbeb3be8aafc1af11235b169fe038c8548988203c62b39660b7eee02cb02385c2bbb3a12019edbbb81d8ba539b04d0bc39a9772b396e15acd474e19c5adcfb0078981c79552bdae371ae2a1e8f4dc22bc2e844c609502a55d9689ea3b991ea3b86e9869135114407a2b3e6620fa425f6a582dabb04913fadebf85ec9a1118f5674645f0dd00f1d431083619fdd204ec4dac1b8493522593eb62a71c225fdc4821fab673a80b7adac00577937812ddd92032702758de1a82e89c08d0aead98ac7fb51e619953ec9e9d0ab08e201a0cda4fdd9b2bab25ddc95017d7a82914f92ced2ae859355cadc4a725c8aeef71eff15c93fd574ede18e67d8d7fa9b0315363fff9f9b78c290ff59506c5fef280b9eb24ebbe13cd7c14d1ba442961c32a2531fd55d9e427a2581ed902d40dbff99c26a76b69c38e2a35f2b36265f5c982ff8951ab775c376619e21c2903443267bf74c5046573ffce9d607562d97839e01df82b2bf1e56284b824942e61bf4afd2d4489a29e863da0b02b63c62d70f33ea81a763a2480fc3f1c77542aaf3497a7cef25f3be6fffad0ddf4f45c0eca08d50047617d30b3e73e23940208d5bfa53354e0fc2c95fa28730710979b9961b090d76654d73962706133fd852ec97f47c0c95d506572de7c108ab2bf9c60941ef278d13b696895b75fc5e0022efe120a04c33a25ad5b053da8392e4dd9d17c7b5b5b50da1c79c1ab59252108264bd8f01316cd3219150e5ffe749eb476dfc73d5c77e5aed7eb11510e68d828241c3bb52fde7b4092353a7273bf0dbd0d0626061e30f31efdf5dfd1f98116362e2b942338d67c093755772515b8206b229ef7fd376eb4c85931e495100271cb1a1cf99b5f8bc54c271cc351cb6651418ca713a2a5b8ff3a47f36ed26bb83c1c2652a94795476fbe268d22c29caab34a5e749e230dd0bcf96978934e642bb8042ffd33faa6093a51b5105b74f0f33d7304e3c806c17143044bf6feb0cfe52e340e0621a99ba93eb3f16c9c72a5b6e85e1691d04f3e03c56e926000311176574606c412fb6b34d5b84e7c124dce8045e51b92e27e879b335475a195fca2b39262124f916323e72944d8687de088754f0d4df79aea2fe1092dace6304664263017038a02b7206ed23c84d3e2c6dc6454fc5e53167eb7d99cf625e5f3650718986125b17bb360099820fdb5d1874e1a7d82740b5540186e29b9c2c2a82d1e2df6f40104555f3f8cf722521a7fdcf154ba9578b64b7060a60af8cc762c882184af20269c8cbe3765589af258e641e63776f0601b4cac85464403175351dd3f17152c01670d5c599e52298e29ed609bf973e85fc2dcbc2cc4d2d8325aaa61efc465a0123be7572ed1d56d1746630661d8ad83e6838e7ac0c855c74f31fa2b3bf541a4179b461229771713eb51b58377c4132f1efceebc92ad16982e64c34a25bbb14926af10f53c166da50ce4fa22d2642566b1ca82a3bf89984daaf7d042efa1c93817f6c02d7ed24ea77061cc40816309eed4f97ed2d2ad12204f4f2620516209580d2a884c4479487a896e2d9843d6f16c056602c9e9deabe1f3d7a6e30dcd93c504bafb5fc2f49f21008828626d62c3113406cb63b35b9894a34abaee5138d14617bef9b282d6a7fda1d337ae8a6d23e9fce5e5b11e002d88016e7a95b3b73921078941ed8a062685a2b020ef184e621a8822ac529d95479ff12b875b32ff1eed040dfa2cb7407705d7b7a94a521b6a82bd827439e5732e73a2941ae8ed60eb7a058a9a95c04ec7692dd1251e156128359748ea3735e689121822b93f6956f313182c74e01b24f2b4bc2ffa5e105f09113e30a5cb25455d0c01431c538198fc940485d12312eec2ae5a3d6b61daca1bc60e58ca7aca735e8a68853a3434e38ff723117c7569db146576ff0e38c88cec42c4d7832dd8de0c0ef2754102bdd99f1e2774733dd649df1ec7fa39de05b49100191f8c35602613d3666db87358b8818a24e9baa1dc2849e89accd19257c529860d79124888aabf2384304119e33677df1616c2da903a24a3a685fead2b3a46d354a1f9695943746616589da0de14132a06378d9cefc3e2353e7fec04c2889be9eeb34ebbed92d15b86e52cd3433ad0db088c26c51501611cb52a743c6cf750db719b489023b312a6cd425e75fcc64d3d191f2debcd6c57d16de08ca7a03bdb619e313f6c8ae3cda5a76e485e8660a4c8129ef7153ecadfb292991986ffcc58be5e9b6f8d192206d3813c5579b410cf230664940be0b0950ad4c6a655d3c39dbc13a5d6e8f591aa886079b2deb54f1b2d2c63697c99d65496033d1dedbdb7161ca53d8a7cfb16b742ad4111f3770a4fec23ab73061982247d53a1033fcc6db11c9528735dab2ab72a45058aa99ab4eb7a2f883fe85134932330e24ebc4b1a7e38c226e6a3dfde860d8ce0be1eb7a882b004a11b2cb90b1af85dbc92c52cc01e656001f1a7af04c14f75015082a990eba50f2bf19ed31f063ad51665c8a64e5de14dcc4903794a670a47d01ab8d84a57231c1976d8116efdd4e2f8d0e40fd6e44db136fb5c86274d4e907f555af7d2b16805c73af078cd942f593eac2afaaf28a1831db8e078ea701cf10db9975258d1ef034ebbdeb9b1481aa92b05f4f4c083fb953838ad2c26990bfecd409b1b9f88741da3e9fb5084639e34956bebe7a470076bf48cd241926bf58ff5c01f59c867bf0b390059dc3dbec07e5e639bba1130cca3e0459d45d7f48c160f023b2d38049e"#;
    let verifier = r#"return = ["0x00000000000000000000000000000000000000000000000000000000000000b9", "0x00000000000000000000000000000000000000000000000000000000000000f8", "0x00000000000000000000000000000000000000000000000000000000000000af", "0x00000000000000000000000000000000000000000000000000000000000000b6", "0x000000000000000000000000000000000000000000000000000000000000005e", "0x00000000000000000000000000000000000000000000000000000000000000ab", "0x00000000000000000000000000000000000000000000000000000000000000f8", "0x0000000000000000000000000000000000000000000000000000000000000032", "0x00000000000000000000000000000000000000000000000000000000000000d8", "0x0000000000000000000000000000000000000000000000000000000000000002", "0x0000000000000000000000000000000000000000000000000000000000000030", "0x000000000000000000000000000000000000000000000000000000000000008d", "0x0000000000000000000000000000000000000000000000000000000000000033", "0x00000000000000000000000000000000000000000000000000000000000000ad", "0x00000000000000000000000000000000000000000000000000000000000000d1", "0x0000000000000000000000000000000000000000000000000000000000000034", "0x00000000000000000000000000000000000000000000000000000000000000d2", "0x0000000000000000000000000000000000000000000000000000000000000093", "0x00000000000000000000000000000000000000000000000000000000000000ac", "0x000000000000000000000000000000000000000000000000000000000000008c", "0x00000000000000000000000000000000000000000000000000000000000000b1", "0x00000000000000000000000000000000000000000000000000000000000000ee", "0x000000000000000000000000000000000000000000000000000000000000009a", "0x0000000000000000000000000000000000000000000000000000000000000007", "0x00000000000000000000000000000000000000000000000000000000000000a4", "0x00000000000000000000000000000000000000000000000000000000000000f8", "0x00000000000000000000000000000000000000000000000000000000000000ec", "0x000000000000000000000000000000000000000000000000000000000000000f", "0x00000000000000000000000000000000000000000000000000000000000000b1", "0x000000000000000000000000000000000000000000000000000000000000008b", "0x0000000000000000000000000000000000000000000000000000000000000068", "0x0000000000000000000000000000000000000000000000000000000000000070"]"#;
    dotenv().ok();
    let bin = PathBuf::from(env::var("DEFAULT_NARGO_BINARY_PATH").expect("Failed to get DEFAULT_NARGO_BINARY_PATH from env!"));
    let circuit = PathBuf::from(env::var("DEFAULT_ABSOLUTE_CIRCUIT_PATH").expect("Failed to get DEFAULT_ABSOLUTE_CIRCUIT_PATH from env!"));
    let vrf = VerifiableRandomGenerator{
        bin: bin,
        circuit: circuit
    };
    vrf.generate(nonce, x, y, signature);
    vrf.verify(proof, verifier);
}