use cosmwasm_std::StdError;
use cw_utils::{Expiration, Scheduled};
use hex::FromHexError;
use thiserror::Error;
use cosmwasm_std::{
    Uint128
};

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Hex(#[from] FromHexError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient Tokens")]
    InsufficientFund {},

    #[error("OnlyOneStage")]
    OnlyOneStage {},

    #[error("Invalid input")]
    InvalidInput {},

    #[error("Already claimed")]
    Claimed {},

    #[error("Wrong length")]
    WrongLength {},

    #[error("Verification failed")]
    VerificationFailed {},

    #[error("Cannot migrate from different contract type: {previous_contract}")]
    CannotMigrate { previous_contract: String },

    #[error("Airdrop stage {stage} expired at {expiration}")]
    StageExpired { stage: u8, expiration: Expiration },

    #[error("Airdrop stage {stage} not expired yet")]
    StageNotExpired { stage: u8, expiration: Expiration },

    #[error("Airdrop stage {stage} begins at {start}")]
    StageNotBegun { stage: u8, start: Scheduled },

    #[error("Total Amount {total_amount}, claimed amount {claimed_amount}")]
    LogAmount { total_amount: Uint128, claimed_amount: Uint128 },
}
