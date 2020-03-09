use clarity::{Address, Signature};
use crate::error::Error;

/// Fixed-width ECDSA secp256k1 signature
pub use ecdsa::curve::secp256k1::FixedSignature;

/// Transaction signer for ECDSA secp256k1 signatures
pub type Signer = dyn ecdsa::signature::Signer<FixedSignature>;

pub struct TxSigner {

    /// Validator address
    pub committee_member: Address,

    pub signer:Box<Signer>,
}


impl TxSigner {
    fn sign(&mut self, msg: &[u8]) -> Result<Signature, Error> {

        let sig = self.signer.try_sign(msg)?;

        Ok(Signature::from_fixed(sig,self.committee_member,msg))

    }
}
