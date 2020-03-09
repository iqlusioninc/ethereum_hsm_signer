//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use std::{env, fs, path::PathBuf, process::exit};
use crate::config::EthereumSignerConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};
use clarity::{Address};
use crate::txsigner::TxSigner;
use zeroize::Zeroizing;




/// HSM authentication key ID
pub const HSM_AUTH_KEY: u16 = 0x0003;

/// HSM-backed ECDSA signing key ID
pub const HSM_SIGNING_KEY: u16 = 0x0004;

pub const COMMITTEE_MEMBER_ADDR: &str = "0x32Be343B94f860124dC4fEe278FDCBD38C102D88";


/// `start` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct StartCmd {
    /// To whom are we saying hello?
    #[options(free)]
    password_file: Vec<PathBuf>,
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let config = app_config();

        let hsm_connector = yubihsm::Connector::http(&Default::default());
        let hsm_credentials = self.load_hsm_credentials();
        let hsm = yubihsm::Client::open(hsm_connector, hsm_credentials, true).unwrap_or_else(|e| {
            status_err!("couldn't connect to YubiHSM: {}", e);
            exit(1);
        });

        let hsm_signer = yubihsm::ecdsa::Signer::create(hsm, HSM_SIGNING_KEY).unwrap_or_else(|e| {
            status_err!(
                "error creating YubiHSM ECDSA signer for key {}: {}",
                HSM_SIGNING_KEY,
                e
            );
            exit(1);
        });

        TxSigner{
            committee_member: COMMITTEE_MEMBER_ADDR.parse().unwrap(),
            signer:Box::new(hsm_signer),
        };

    }
}

impl StartCmd {
    fn load_hsm_credentials(&self) -> yubihsm::Credentials {
        let password_file = self.password_file.get(0).unwrap_or_else(|| {
            status_err!("usage: terra-voter /path/to/password/file");
            exit(1);
        });

        let password = Zeroizing::new(fs::read_to_string(password_file).unwrap_or_else(|e| {
            status_err!("couldn't read key from {}: {}", password_file.display(), e);
            exit(1);
        }));

        yubihsm::Credentials::from_password(HSM_AUTH_KEY, password.trim_end().as_bytes())
    }
}


impl config::Override<EthereumSignerConfig> for StartCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: EthereumSignerConfig,
    ) -> Result<EthereumSignerConfig, FrameworkError> { 
        Ok(config)
    }
}
