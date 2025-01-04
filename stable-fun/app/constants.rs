use anchor_lang::prelude::*;

pub const STABLECOIN_SEED: &[u8] = b"stablecoin";
pub const VAULT_SEED: &[u8] = b"vault";
pub const MIN_NAME_LENGTH: usize = 3;
pub const MAX_NAME_LENGTH: usize = 32;
pub const MIN_SYMBOL_LENGTH: usize = 2;
pub const MAX_SYMBOL_LENGTH: usize = 8;
pub const PRICE_DECIMALS: u64 = 1_000_000_000; // 9 decimals
pub const DEFAULT_COLLATERAL_RATIO: u64 = 12000; // 120%
pub const DEFAULT_FEE_PERCENTAGE: u64 = 30; // 0.3%
