use ethers::prelude::{abigen, EthEvent};
use ethers::types::{Address, U256};

abigen!(PoolCommitter, "abis/v2/PoolCommitter.json");

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

pub static POOLS_V2_ARBITRUM_TESTNET_POOLCOMMITTERS: &[&str; 4] = &[
    "0xc8635F7da7830f2268B3210D64D9584A864C7809",
    "0xa321c542a23f5173361f29c3809FAa74C25dAB46",
    "0x5e3F781936814302eFe5676A8BcE66ACbbD954b6",
    "0xA7374E54F286427c896810F6b60E85e5dD913BbE",
];

pub static POOLS_V1_ARBITRUM_MAINNET_POOLCOMMITTERS: &[&str; 9] = &[
    "0x993321599Fc9D0c5a496044308f16C70575DABBa",
    "0x8186948382f67c7160Fc7b872688AdC293aDF789",
    "0xb913D14B3a3bB1D06B2dB1Fd141f2432bB25F5F2",
    "0xb894D3775862FFdE084eD31f9e42388e592E3137",
    "0x149BDeAC3E90522D8043452910Ef41f7cb75E3f3",
    "0x539Bf88D729B65F8eC25896cFc7a5f44bbf1816b",
    "0xFDE5D7B7596AF6aC5df7C56d76E14518A9F578dF",
    "0x047Cd47925C2390ce26dDeB302b8b165d246d450",
    "0x72c4e7Aa6c743DA4e690Fa7FA66904BC3f2C9C04",
];
