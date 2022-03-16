use ethers::prelude::{abigen, EthEvent};
use ethers::types::{Address, U256};

abigen!(LeveragedPool, "abis/LeveragedPool.json");

pub const EVENT_NAME: &str = "CreateCommit";

#[derive(Copy, Clone, Debug)]
pub enum CommitType {
    ShortMint,
    ShortBurn,
    LongMint,
    LongBurn,
    LongBurnShortMint,
    ShortBurnLongMint,
}

#[derive(Clone, Debug, EthEvent)]
pub struct CreateCommit {
    user: Address,
    amount: U256,
    commit_type: U256,
    appropriate_update_interval_id: U256,
    from_aggregate_balance: bool,
    pay_for_claim: bool,
    minting_fee: [u8; 16],
}
