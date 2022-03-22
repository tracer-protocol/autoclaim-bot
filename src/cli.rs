use std::fs;
use std::path::PathBuf;

use clap::Parser;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::providers::{Http, Provider};
use ethers::signers::{
    coins_bip39::English, LocalWallet, MnemonicBuilder, WalletError,
};

use crate::client::Client;

#[derive(Clone, Debug, Parser)]
#[clap(about, version, author)]
pub struct Opts {
    rpc: String,
    #[clap(long)]
    private_key: Option<PathBuf>,
    #[clap(long)]
    seed_phrase: Option<PathBuf>,
    #[clap(long)]
    seed_phrase_index: Option<u32>,
    #[clap(long)]
    read_only: Option<bool>,
}

#[derive(Debug)]
pub enum ClientParseError {
    NoOffset,
    NotLocalWallet,
    NotHttp(url::ParseError),
    NotWs(url::ParseError),
    InvalidWallet(WalletError),
    IoError(std::io::Error),
    KeyError(ethers::core::k256::ecdsa::Error),
    HexError(hex::FromHexError),
}

impl From<WalletError> for ClientParseError {
    fn from(value: WalletError) -> Self {
        Self::InvalidWallet(value)
    }
}

impl From<std::io::Error> for ClientParseError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ethers::core::k256::ecdsa::Error> for ClientParseError {
    fn from(value: ethers::core::k256::ecdsa::Error) -> Self {
        Self::KeyError(value)
    }
}

impl From<hex::FromHexError> for ClientParseError {
    fn from(value: hex::FromHexError) -> Self {
        Self::HexError(value)
    }
}

impl TryFrom<Opts> for Client<LocalWallet, Http> {
    type Error = ClientParseError;

    fn try_from(value: Opts) -> Result<Self, Self::Error> {
        let http_provider: Provider<Http> =
            match Provider::<Http>::try_from(value.rpc) {
                Ok(t) => t,
                Err(e) => return Err(Self::Error::NotHttp(e)),
            };

        if let Some(read_only_mode) = value.read_only {
            /* no short-circuit due to (https://github.com/rust-lang/rust/issues/53667) */
            if read_only_mode {
                return Ok(Self::new(None, http_provider));
            }
        }

        if let Some(private_key_path) = value.private_key {
            let wallet: LocalWallet = SigningKey::from_bytes(&hex::decode(
                &fs::read_to_string(private_key_path)?.trim(),
            )?)?
            .into();
            Ok(Self::new(Some(wallet), http_provider))
        } else if let Some(seed_phrase_path) = value.seed_phrase {
            if let Some(index) = value.seed_phrase_index {
                /* child key at derivation path: m/44'/60'/0'/0/{index} */
                let wallet: LocalWallet = MnemonicBuilder::<English>::default()
                    .phrase(seed_phrase_path)
                    .index(index)?
                    .build()?;

                Ok(Self::new(Some(wallet), http_provider))
            } else {
                Err(Self::Error::NoOffset)
            }
        } else {
            Err(Self::Error::NotLocalWallet)
        }
    }
}

/* disabled due to (https://github.com/gakonst/ethers-rs/issues/1020) */

/*
impl TryFrom<Opts> for Client<LocalWallet, Ws> {
    type Error = ClientParseError;

    fn try_from(value: Opts) -> Result<Self, Self::Error> {
        let ws_provider: Provider<Ws> =
            match Provider::<Ws>::try_from(value.rpc) {
                Ok(t) => t,
                Err(e) => return Err(Self::Error::NotWs(e)),
            };

        if let Some(private_key_path) = value.private_key {
            let wallet: LocalWallet =
                SigningKey::from_bytes(&fs::read(private_key_path)?)?.into();
            Ok(Self::new(wallet, ws_provider))
        } else if let Some(seed_phrase_path) = value.seed_phrase {
            if let Some(index) = value.seed_phrase_index {
                /* child key at derivation path: m/44'/60'/0'/0/{index} */
                let wallet: LocalWallet = MnemonicBuilder::<English>::default()
                    .phrase(seed_phrase_path)
                    .index(index)?
                    .build()?;

                Ok(Self::new(wallet, ws_provider))
            } else {
                Err(Self::Error::NoOffset)
            }
        } else {
            Err(Self::Error::NotLocalWallet)
        }
    }
}
*/
