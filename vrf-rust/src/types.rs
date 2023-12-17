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
        let mut result: String = String::new();
        let verifier: Vec<String> = serde_json::from_str(&self.verifier[9..&self.verifier.len()-1]).unwrap();
        for num in verifier{
            //result += u128::from_str_radix(&num[2..], 16).unwrap();
            result += &u128::from_str_radix(&num[2..], 16).unwrap().to_string();
        };
        result[..32].parse().unwrap()
    }
}