use dep::std;

struct Random{
    signature: [u8;64]
}

impl Random{
    fn hash(self) -> [u8;32]{
        std::hash::sha256(self.signature)
    }
}