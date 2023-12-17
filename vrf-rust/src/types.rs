use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Proof{
    pub proof: String,
    pub verifier: String
}

impl Proof{
    pub fn to_string(&mut self) -> String{
        serde_json::to_string(self).unwrap()
    }
    pub fn from_string(proof: String) -> Proof{
        serde_json::from_str(&proof).unwrap()
    }
    pub fn get_random_number(&self) -> u128{
        // collision is more likely than with a sha256 hash, but should still be very low.
        // a BigInt could be used to take the entire hash into consideration / ensure the randomness matches that of sha256.
        // sha256 can be replaced w. an even stronger hashing algorithm, but for most use cases this implementation should really be sufficient.
        let mut result: String = String::new();
        let verifier: Vec<String> = serde_json::from_str(&self.verifier[9..&self.verifier.len()-1]).unwrap();
        for num in verifier{
            result += &u128::from_str_radix(&num[2..], 16).unwrap().to_string();
        };
        result[..32].parse().unwrap()
    }
}