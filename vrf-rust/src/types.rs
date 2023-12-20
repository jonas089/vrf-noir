use serde::{Serialize, Deserialize};
use serde_json;
use regex::Regex;
#[derive(Serialize, Deserialize)]
pub struct Proof{
    pub proof: String,
    pub verifier: String
}

#[derive(Debug)]
pub struct Verifier{
    pub nonce: Option<Vec<String>>,
    pub output: Vec<String>,
    pub x: Option<Vec<String>>,
    pub y: Option<Vec<String>>
}

impl Proof{
    pub fn to_string(&mut self) -> String{
        serde_json::to_string(self).unwrap()
    }
    pub fn from_string(proof: String) -> Proof{
        serde_json::from_str(&proof).unwrap()
    }
    pub fn get_random_number(&self, is_x_public: bool, is_y_public: bool, is_nonce_public: bool) -> u128{
        // collision is more likely than with a sha256 hash, but should still be very low.
        // a BigInt could be used to take the entire hash into consideration / ensure the randomness matches that of sha256.
        // sha256 can be replaced w. an even stronger hashing algorithm, but for most use cases this implementation should really be sufficient.

        //let mut result: String = String::new();
        //let verifier: Vec<String> = serde_json::from_str(&self.verifier[9..&self.verifier.len()-1]).unwrap();

        let mut vf = Verifier{
            nonce: None,
            output: Vec::new(),
            x: None,
            y: None
        };
        let re = Regex::new(r"(\w+)\s*=\s*\[(.*?)\]").map_err(|_| "Failed to compile regex").unwrap();    
        for cap in re.captures_iter(&self.verifier) {
            let array_name = cap.get(1).unwrap().as_str();
            let values = cap.get(2).unwrap().as_str()
                .split(", ")
                .map(|s| s.trim_matches('"').to_string())
                .collect::<Vec<String>>();
    
            match array_name {
                "nonce" => vf.nonce = Some(values),
                "return" => vf.output = values,
                "x" => vf.x = Some(values),
                "y" => vf.y = Some(values),
                _ => eprint!("Invalid value in Verifier!")
            }
        }
        println!("Public information: {:?}", &vf);

        let mut result: String = String::new();
        for num in vf.output{
            result += &u128::from_str_radix(&num[2..], 16).unwrap().to_string();
        };
        result[..32].parse().unwrap()
    }
}