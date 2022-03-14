use std::sync::Arc;

use clap::Parser;
use ethers::providers::{Http, Middleware, ProviderError, StreamExt};
use ethers::signers::LocalWallet;
use ethers::types::{Block, Transaction};

use crate::cli::{ClientParseError, Opts};
use crate::client::Client;

pub mod cli;
pub mod client;

#[derive(Debug)]
pub enum Error {
    ClientOptsError(ClientParseError),
    ProviderError(ProviderError),
}

impl From<ClientParseError> for Error {
    fn from(value: ClientParseError) -> Self {
        Self::ClientOptsError(value)
    }
}

impl From<ProviderError> for Error {
    fn from(value: ProviderError) -> Self {
        Self::ProviderError(value)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    let client: Arc<Client<LocalWallet, Http>> = Arc::new(opts.try_into()?);

    let mut stream = client.provider().watch_blocks().await?;

    /* listen for new blocks */
    while let Some(block_hash) = stream.next().await {
        let block: Block<Transaction> = client
            .provider()
            .get_block_with_txs(block_hash)
            .await?
            .unwrap();
        println!("{}", serde_json::to_string(&block).expect("Invalid data"));
    }

    Ok(())
}
