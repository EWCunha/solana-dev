use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, MintTo, TokenInterface, TokenAccount, TransferChecked, Burn};

declare_id!("sCt6iFaX8TJEZVVkCR7MBg4DnPvNV4p98r9ur7Sw5oN");

mod utils;
use utils::*;

mod errors;
use errors::*;

#[program]
pub mod token_vault {
    use super::*;

    pub fn initialize(ctx: Context<InitializeVault>, _decimals: u8, asset_mint: Pubkey, vault_shares_token_account: Pubkey, vault_asset_token_account: Pubkey) -> Result<()> {
        msg!("Creating mint");

        let vault = &mut ctx.accounts.vault;
        vault.shares_mint = ctx.accounts.shares_mint.key();
        vault.asset_mint = asset_mint;
        vault.vault_shares_token_account = vault_shares_token_account;
        vault.vault_asset_token_account = vault_asset_token_account;

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
        Ok(_max_deposit())
    }

    pub fn max_mint(_ctx: Context<Convert>) -> Result<u64> {
        Ok(_max_mint())
    }

    pub fn max_withdraw(ctx: Context<MaxWithdraw>) -> Result<u64> {
        Ok(_max_withdraw(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            ctx.accounts.shares_account.amount
        ))
    }

    pub fn max_redeem(ctx: Context<MaxRedeem>) -> Result<u64> {
        Ok(_max_redeem(ctx.accounts.shares_account.amount))
    }

    pub fn preview_deposit(ctx: Context<Convert>, assets: u64) -> Result<u64> {
        Ok(_preview_deposit(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            assets,
        ))
    }

    pub fn preview_mint(ctx: Context<Convert>, shares: u64) -> Result<u64> {
        Ok(_preview_mint(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            shares,
        ))
    }

    pub fn preview_redeem(ctx: Context<Convert>, shares: u64) -> Result<u64> {
        Ok(_preview_redeem(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            shares,
        ))
    }

    pub fn preview_withdraw(ctx: Context<Convert>, shares: u64) -> Result<u64> {
        Ok(_preview_withdraw(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            shares, 
        ))
    }

    pub fn deposit(ctx: Context<Deposit>, assets: u64) -> Result<u64> {
        msg!("Depositing {} tokens", assets);

        require!(assets <= _max_deposit(), TokenVaultError::DepositAmountTooLarge);

        let shares = _preview_deposit(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            assets,
        );

        _deposit(
            &ctx.accounts.shares_mint,
            &ctx.accounts.vault_shares_token_account,
            &ctx.accounts.asset_token_mint,
            &ctx.accounts.user_asset_token_account,
            &ctx.accounts.vault_asset_token_account,
            &ctx.accounts.token_program,
            &ctx.accounts.signer,
            assets,
            shares
        )?;

        Ok(shares)
    }

    pub fn mint(ctx: Context<Deposit>, shares: u64) -> Result<u64> {
        msg!("Minting {} shares", shares);

        require!(shares <= _max_mint(), TokenVaultError::MintAmountTooLarge);

        let assets = _preview_mint(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            shares,
        );

        _deposit(
            &ctx.accounts.shares_mint,
            &ctx.accounts.vault_shares_token_account,
            &ctx.accounts.asset_token_mint,
            &ctx.accounts.user_asset_token_account,
            &ctx.accounts.vault_asset_token_account,
            &ctx.accounts.token_program,
            &ctx.accounts.signer,
            assets,
            shares
        )?;

        Ok(assets)
    }

    pub fn withdraw(ctx: Context<Withdraw>, assets: u64) -> Result<u64> {
        msg!("Withdrawing {} shares", assets);

        require!(assets <= _max_withdraw(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            ctx.accounts.vault_shares_token_account.amount
        ), TokenVaultError::MintAmountTooLarge);

        let shares = _preview_withdraw(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            assets,
        );

        _withdraw(
            &ctx.accounts.shares_mint,
            &ctx.accounts.vault_shares_token_account.to_account_info(),
            &ctx.accounts.asset_token_mint,
            &ctx.accounts.user_asset_token_account.to_account_info(),
            &ctx.accounts.vault_asset_token_account.to_account_info(),
            &ctx.accounts.token_program,
            &ctx.accounts.signer,
            &ctx.accounts.shares_authority,
            assets,
            shares
        )?;

        Ok(assets)
    }

    pub fn redeem(ctx: Context<Withdraw>, shares: u64) -> Result<u64> {
        msg!("Redeeming {} shares", shares);

        require!(shares <= _max_redeem(shares), TokenVaultError::MintAmountTooLarge);

        let assets = _preview_redeem(
            ctx.accounts.shares_mint.supply, 
            ctx.accounts.asset_account.amount, 
            shares,
        );

        _withdraw(
            &ctx.accounts.shares_mint,
            &ctx.accounts.vault_shares_token_account.to_account_info(),
            &ctx.accounts.asset_token_mint,
            &ctx.accounts.user_asset_token_account.to_account_info(),
            &ctx.accounts.vault_asset_token_account.to_account_info(),
            &ctx.accounts.token_program,
            &ctx.accounts.signer,
            &ctx.accounts.shares_authority,
            assets,
            shares
        )?;

        Ok(assets)
    }
  

    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    //     msg!("Withdrawing {} tokens", amount);
    //     Ok(())
    // }
}

fn _deposit<'info>(
    shares_token_mint: &InterfaceAccount<'info, Mint>,
    vault_shares_token_account: &AccountInfo<'info>, 
    asset_token_mint: &InterfaceAccount<'info, Mint>, 
    user_asset_token_account: &AccountInfo<'info>,
    vault_asset_token_account: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    signer: &AccountInfo<'info>, 
    assets: u64,
    shares: u64,
) -> Result<()> { 
    let cpi_accounts = MintTo {
        mint: shares_token_mint.to_account_info(),
        to: vault_shares_token_account.to_account_info(),
        authority: signer.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token_interface::mint_to(cpi_context, shares)?;


    let cpi_accounts: TransferChecked<'_> = TransferChecked {
        mint: asset_token_mint.to_account_info(),
        from: user_asset_token_account.to_account_info(),
        to: vault_asset_token_account.to_account_info(),
        authority: signer.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token_interface::transfer_checked(cpi_context, assets, asset_token_mint.decimals)?;

    Ok(())
}

fn _withdraw<'info>(
    shares_token_mint: &InterfaceAccount<'info, Mint>,
    vault_shares_token_account: &AccountInfo<'info>, 
    asset_token_mint: &InterfaceAccount<'info, Mint>, 
    user_asset_token_account: &AccountInfo<'info>,
    vault_asset_token_account: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    signer: &AccountInfo<'info>, 
    shares_authority: &UncheckedAccount<'info>,
    assets: u64,
    shares: u64,
) -> Result<()> {
    let cpi_accounts = Burn {
        mint: shares_token_mint.to_account_info(),
        from: vault_shares_token_account.to_account_info(),
        authority: signer.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token_interface::burn(cpi_context, shares)?;

    let cpi_accounts: TransferChecked<'_> = TransferChecked {
        mint: asset_token_mint.to_account_info(),
        from: vault_asset_token_account.to_account_info(),
        to: user_asset_token_account.to_account_info(),
        authority: shares_authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token_interface::transfer_checked(cpi_context, assets, asset_token_mint.decimals)?;

    Ok(())
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

    #[account(
        init,
        payer = signer,
        space = 8 + size_of::<Vault>(),
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        seeds = [b"vault_mint"],
        bump
    )]
    pub shares_mint: InterfaceAccount<'info, Mint>,   

    #[account(mut)]
    pub vault_shares_token_account: AccountInfo<'info>,

    #[account(mut)]
    pub vault_asset_token_account: AccountInfo<'info>,

    #[account(mut)]
    pub asset_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub user_asset_token_account: AccountInfo<'info>,

    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub asset_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [b"vault_mint"],
        bump
    )]
    pub shares_mint: InterfaceAccount<'info, Mint>,   

    #[account(mut)]
    pub vault_shares_token_account:  InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub vault_asset_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub asset_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub user_asset_token_account: InterfaceAccount<'info, TokenAccount>,

    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub asset_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: PDA, no need to deserialize
    #[account(
        seeds = [b"authority"],
        bump
    )]
    pub shares_authority: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
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

#[account]
pub struct Vault {
    pub shares_mint: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_shares_token_account: Pubkey,
    pub vault_asset_token_account: Pubkey,
}   