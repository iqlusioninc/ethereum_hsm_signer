use clarity::{Address, Signature};
use crate::error::Error;
use crate::sig_service::signatures_client::SignaturesClient;
use crate::sig_service::{Signable,Signed,Empty};



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

    pub async fn run(&mut self,client: &mut SignaturesClient<tonic::transport::channel::Channel>){

        let empty_request = tonic::Request::new(Empty{});

        let response = client.get_signable(empty_request).await.unwrap().into_inner();

        let sig = self.sign(&response.msg).unwrap();

        let sig_request = tonic::Request::new(Signed{msg:response.msg,sig:(sig.to_bytes()).to_vec()});

        let _empty_response = client.provide_sig(sig_request).await.unwrap();

    }
}
