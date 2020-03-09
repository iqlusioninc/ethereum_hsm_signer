//! Main entry point for EthereumSigner

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use ethereum_signer::application::APPLICATION;

/// Boot EthereumSigner
fn main() {
    abscissa_core::boot(&APPLICATION);
}
