use std::io;
use std::io::{stdout, Write};
use std::str::FromStr;
use std::sync::Arc;

use clap::Parser;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider, ProviderError, StreamExt};
use ethers::signers::LocalWallet;
use ethers::types::{Address, Block, BlockNumber, Filter, Transaction};

use crate::cli::{ClientParseError, Opts};
use crate::client::Client;
use crate::pool::PoolCommitter;

pub mod cli;
pub mod client;
pub mod pool;

pub const EVENT_NAME: &str = "UpkeepSuccessful";

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
        "0x98C58c1cEb01E198F8356763d5CbA8EB7b11e4E2",
    ] /* TODO: placeholder */
    .iter()
    .copied()
    .map(|x| Address::from_str(x).unwrap())
    .collect();

    let _pools: Vec<
        PoolCommitter<SignerMiddleware<Provider<Http>, LocalWallet>>,
    > = pool_addresses
        .iter()
        .copied()
        .map(|x| {
            PoolCommitter::new(
                x,
                Arc::new(client.as_ref().middleware().unwrap()),
            )
        })
        .collect();

    let commitment_filter: Filter = Filter::new().event(EVENT_NAME);

    let mut stream = client.provider().watch(&commitment_filter).await?;

    /* listen for new events */
    while let Some(log) = stream.next().await {
        writeln!(
            stdout(),
            "{}",
            serde_json::to_string(&log).expect("Invalid log data")
        )?;
    }

    Ok(())
}
