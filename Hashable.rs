use super::*;
// just need this as a trait so that you can hash anything
pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}