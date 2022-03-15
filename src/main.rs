use std::io;
use std::io::{stdout, Write};
use std::str::FromStr;
use std::sync::Arc;

use clap::Parser;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider, ProviderError, StreamExt};
use ethers::signers::LocalWallet;
use ethers::types::{Address, Block, Transaction};

use crate::cli::{ClientParseError, Opts};
use crate::client::Client;
use crate::pool::LeveragedPool;

pub mod cli;
pub mod client;
pub mod pool;

/// Represents global errors emitted from the bot process itself
#[derive(Debug)]
pub enum Error {
    /// Error parsing client from provided arguments
    ClientOptsError(ClientParseError),
    /// Error in blockchain interaction
    ProviderError(ProviderError),
    /// Error in I/O
    IOError(io::Error),
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

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    /* TODO: generalise this parsing of clients when Websockets is unblocked */
    let client: Arc<Client<LocalWallet, Http>> = Arc::new(opts.try_into()?);

    let pool_addresses: Vec<Address> = vec![
        "0x23A5744eBC353944A4d5baaC177C16b199AfA4ed",
    ] /* TODO: placeholder */
    .iter()
    .copied()
    .map(|x| Address::from_str(x).unwrap())
    .collect();
    let pools: Vec<
        LeveragedPool<SignerMiddleware<Provider<Http>, LocalWallet>>,
    > = pool_addresses
        .iter()
        .copied()
        .map(|x| {
            LeveragedPool::new(
                x,
                Arc::new(client.as_ref().middleware().unwrap()),
            )
        })
        .collect();

    let mut stream = client.provider().watch_blocks().await?;

    /* listen for new blocks */
    while let Some(block_hash) = stream.next().await {
        let block: Block<Transaction> = client
            .provider()
            .get_block_with_txs(block_hash)
            .await?
            .unwrap();
        writeln!(
            stdout(),
            "{}",
            serde_json::to_string(&block).expect("Invalid data")
        )?;
    }

    Ok(())
}
