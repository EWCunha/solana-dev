use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction::transfer,
};
use std::mem::size_of;

declare_id!("GcaZsJKoLr6fWLsk6sYqBQEx4HjmjtDeRMYmrYUZA81z");

const CONFIG_SEED: &[u8] = b"config";
const WALLET_SEED: &[u8] = b"wallet";

#[program]
pub mod wallet_locker {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, time_lock: u64) -> Result<()> {
        msg!("Initializing config");

        ctx.accounts.config.time_lock = time_lock;
        ctx.accounts.config.authority = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn set_time_lock(ctx: Context<SetTimeLock>, time_lock: u64) -> Result<()> {
        msg!("Setting time lock to {:?}", time_lock);

        ctx.accounts.config.time_lock = time_lock;

        Ok(())
    }

    pub fn initialize_wallet(ctx: Context<InitializeWallet>) -> Result<()> {
        msg!("Initializing wallet for {:?}", ctx.accounts.signer.key());

        Ok(())
    }

    pub fn deposit(ctx: Context<DepositAndWithdraw>, amount: u64) -> Result<()> {
        msg!(
            "Depositing {:?} into wallet {}",
            amount,
            ctx.accounts.wallet.key()
        );

        let transfer_instruction = transfer(
            &ctx.accounts.signer.key(),
            &ctx.accounts.wallet.key(),
            amount,
        );

        invoke(
            &transfer_instruction,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.wallet.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        msg!("Deposit successful");

        ctx.accounts.wallet.last_deposit = Clock::get()?.unix_timestamp as u64;

        Ok(())
    }

    pub fn withdraw(ctx: Context<DepositAndWithdraw>, amount: u64) -> Result<()> {
        msg!(
            "Withdrawing {:?} from wallet {}",
            amount,
            ctx.accounts.wallet.key()
        );

        let now = Clock::get()?.unix_timestamp as u64;
        require!(
            now - ctx.accounts.wallet.last_deposit >= ctx.accounts.config.time_lock,
            Errors::WithdrawalNotAllowed
        );

        ctx.accounts.wallet.sub_lamports(amount)?;
        ctx.accounts.signer.add_lamports(amount)?;

        msg!("Withdrawal successful");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(init, payer = authority, space = 8 + size_of::<Config>(), seeds = [CONFIG_SEED], bump)]
    config: Account<'info, Config>,

    #[account(mut)]
    authority: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTimeLock<'info> {
    #[account(mut, has_one = authority)]
    config: Account<'info, Config>,

    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeWallet<'info> {
    #[account(init, payer = signer, space = 8 + size_of::<Wallet>(), seeds = [WALLET_SEED, signer.key().as_ref()], bump)]
    wallet: Account<'info, Wallet>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositAndWithdraw<'info> {
    #[account(mut, seeds = [WALLET_SEED, signer.key().as_ref()], bump)]
    wallet: Account<'info, Wallet>,

    #[account(mut)]
    signer: Signer<'info>,

    config: Account<'info, Config>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Wallet {
    pub last_deposit: u64,
}

#[account]
pub struct Config {
    pub time_lock: u64,
    pub authority: Pubkey,
}

#[error_code]
pub enum Errors {
    #[msg("Withdrawal not allowed yet")]
    WithdrawalNotAllowed,
}
