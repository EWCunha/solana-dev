use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, TokenAccount};

declare_id!("sCt6iFaX8TJEZVVkCR7MBg4DnPvNV4p98r9ur7Sw5oN");

#[program]
pub mod token_vault {
    use super::*;

    pub fn initialize(_ctx: Context<InitializeVault>, _decimals: u8) -> Result<()> {
        msg!("Creating mint");
        Ok(())
    }

    pub fn convert_to_shares(ctx: Context<Convert>, assets: u64, round_up: bool) -> Result<u64> {
        msg!("Converting {} tokens to shares", assets);

        Ok(
            _convert_to_shares(
                ctx.accounts.shares_mint.supply, 
                ctx.accounts.asset_account.amount, 
                assets,
                round_up
            )
        )
    }

    pub fn convert_to_assets(ctx: Context<Convert>, shares: u64, round_up: bool) -> Result<u64> {
        msg!("Converting {} tokens to shares", shares);

        Ok(
            _convert_to_assets(
                ctx.accounts.shares_mint.supply, 
                ctx.accounts.asset_account.amount, 
                shares,
                round_up
            )
        )
    }

    pub fn max_deposit(_ctx: Context<Convert>) -> Result<u64> {
        Ok(u64::MAX)
    }

    pub fn max_mint(_ctx: Context<Convert>) -> Result<u64> {
        Ok(u64::MAX)
    }

    pub fn max_withdraw(ctx: Context<MaxWithdraw>) -> Result<u64> {
        Ok(
            _convert_to_assets(
                ctx.accounts.shares_mint.supply, 
                ctx.accounts.asset_account.amount, 
                ctx.accounts.shares_account.amount,
                false
            )
        )
    }

    pub fn max_redeem(ctx: Context<MaxRedeem>) -> Result<u64> {
        Ok(ctx.accounts.shares_account.amount)
    }

    pub fn preview_deposit(ctx: Context<Convert>, assets: u64) -> Result<u64> {
        Ok(_convert_to_shares(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            assets, 
            false
        ))
    }

    pub fn preview_mint(ctx: Context<Convert>, shares: u64) -> Result<u64> {
        Ok(_convert_to_assets(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            shares, 
            true
        ))
    }

    pub fn preview_redeem(ctx: Context<Convert>, shares: u64) -> Result<u64> {
        Ok(_convert_to_assets(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            shares, 
            false
        ))
    }

    pub fn preview_withdraw(ctx: Context<Convert>, shares: u64) -> Result<u64> {
        Ok(_convert_to_assets(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            shares, 
            false
        ))
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        msg!("Depositing {} tokens", amount);

       let max_shares = max_mint(ctx.accounts.shares_mint.supply, ctx.accounts.asset_account.amount);

        Ok(())
    }

  

    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    //     msg!("Withdrawing {} tokens", amount);
    //     Ok(())
    // }
}

fn _convert_to_shares(total_supply: u64, total_assets: u64, assets: u64, round_up: bool) -> u64 {
    let total_supply = if total_supply == 0 {
        total_supply + 1
    } else {
        total_supply
    };

    let total_assets = if total_assets == 0 {
        total_assets + 1
    } else {
        total_assets
    };

    let shares_amount = assets * total_supply / total_assets;

    if round_up && shares_amount * total_assets != assets * total_supply {
        shares_amount + 1
    } else {
        shares_amount
    }
}

fn _convert_to_assets(total_supply: u64, total_assets: u64, shares: u64, round_up: bool) -> u64 {
    let total_supply = if total_supply == 0 {
        total_supply + 1
    } else {
        total_supply
    };

    let total_assets = if total_assets == 0 {
        total_assets + 1
    } else {
        total_assets
    };

    let assets_amount = shares * total_assets / total_supply;

    if round_up && shares * total_supply != shares * total_assets {
        assets_amount + 1
    } else {
        assets_amount
    }
}


#[derive(Accounts)]
#[instruction(_decimals: u8)]
pub struct InitializeVault<'info> {
    #[account(
        init, 
        payer = signer,
        mint::decimals = _decimals,
        mint::authority = shares_authority.key(),
        seeds = [b"vault_mint"],
        bump
    )]
    pub shares_mint: InterfaceAccount<'info, Mint>,   

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

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Convert<'info> {
    #[account(
        seeds = [b"vault_mint"],
        bump
    )]
    pub shares_mint: InterfaceAccount<'info, Mint>,   

    pub asset_account: InterfaceAccount<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct MaxWithdraw<'info> {
    #[account(
        seeds = [b"vault_mint"],
        bump
    )]
    pub shares_mint: InterfaceAccount<'info, Mint>,   

    pub shares_account: InterfaceAccount<'info, TokenAccount>,
    pub asset_account: InterfaceAccount<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct MaxRedeem<'info> {
    pub shares_account: InterfaceAccount<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Previews {
 
}