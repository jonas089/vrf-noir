use dep::std;

struct Random{
    nonce: [u8;32]
}

impl Random{
    fn hash(self) -> [u8;32]{
        std::hash::sha256(self.nonce)
    }
}