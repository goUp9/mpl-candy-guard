#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod svg;
pub mod utils;

pub use instructions::*;
pub use state::*;
pub use svg::*;
pub use utils::*;
pub use utils::errors::StrawberryError;

declare_id!("YOUR_PROGRAM_ID_HERE");

#[program]
pub mod strawberry_nft {
    use super::*;

    // Initialize the program state
    pub fn initialize(
        ctx: Context<Initialize>,
        max_supply: u32,
        mint_price: u64,
        whitelist_merkle_root: [u8; 32],
        whitelist_mint_limit: u8,
        creator_fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize(ctx, max_supply, mint_price, whitelist_merkle_root, whitelist_mint_limit, creator_fee_bps)
    }
    
    // Initialize candy guard
    pub fn initialize_candy_guard(
        ctx: Context<InitializeCandyGuard>,
        whitelist_config: Option<WhitelistGuardConfig>,
        mint_limit_config: Option<MintLimitGuardConfig>,
        start_date_config: Option<StartDateGuardConfig>,
        sol_payment_config: Option<SolPaymentGuardConfig>,
    ) -> Result<()> {
        instructions::candy_guard::initialize_candy_guard(
            ctx, 
            whitelist_config, 
            mint_limit_config, 
            start_date_config, 
            sol_payment_config
        )
    }

    // Mint a single NFT with candy guard
    pub fn mint_with_guard(
        ctx: Context<MintWithGuard>,
        personality_data: PersonalityData,
        whitelist_merkle_proof: Option<Vec<[u8; 32]>>,
    ) -> Result<()> {
        instructions::mint_with_guard(ctx, personality_data, whitelist_merkle_proof)
    }

    // Batch mint multiple NFTs with candy guard
    pub fn batch_mint_with_guard(
        ctx: Context<BatchMintWithGuard>,
        personality_data_array: Vec<PersonalityData>,
        whitelist_merkle_proof: Option<Vec<[u8; 32]>>,
    ) -> Result<()> {
        instructions::batch_mint_with_guard(ctx, personality_data_array, whitelist_merkle_proof)
    }

    // Admin functions
    pub fn update_mint_price(ctx: Context<AdminOnly>, new_price: u64) -> Result<()> {
        instructions::admin::update_mint_price(ctx, new_price)
    }

    pub fn toggle_active(ctx: Context<AdminOnly>) -> Result<()> {
        instructions::admin::toggle_active(ctx)
    }

    pub fn set_phase(ctx: Context<AdminOnly>, phase: u8) -> Result<()> {
        instructions::admin::set_phase(ctx, phase)
    }

    pub fn update_whitelist(ctx: Context<AdminOnly>, new_merkle_root: [u8; 32]) -> Result<()> {
        instructions::admin::update_whitelist(ctx, new_merkle_root)
    }

    pub fn update_creator(ctx: Context<AdminOnly>, new_creator: Pubkey) -> Result<()> {
        instructions::admin::update_creator(ctx, new_creator)
    }
}