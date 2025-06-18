use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, TokenAccount};

declare_id!("sCt6iFaX8TJEZVVkCR7MBg4DnPvNV4p98r9ur7Sw5oN");

#[program]
pub mod token_vault {
    use super::*;

    pub fn initialize(ctx: Context<InitializeVault>) -> Result<()> {
        msg!("Creating mint");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init, 
        payer = signer,
        mint::decimals = 9,
        mint::authority = shares_authority.key(),
        seeds = [b"vault_mint"],
        bump
    )]
    pub shares_mint: InterfaceAccount<'info, Mint>,   

    // #[account(
    //     init,
    //     payer = signer,
    //     token::mint = shares_mint,
    //     token::authority = shares_authority.key(),
    //     token::token_program = token_program,
    //     seeds = [b"vault_ta"],
    //     bump
    // )]
    // pub token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: PDA, no need to deserialize
    #[account(
        seeds = [b"authority"],
        bump
    )]
    pub shares_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
