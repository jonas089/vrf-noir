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
}