use anchor_lang::prelude::*;

declare_id!("DpQ62PGS8Fn5djRMCpoP5DJZyQua9sfsQUuCAzCNPjGh");

#[program]
pub mod ping_counter {
    use super::*;

    pub fn initialize(ctx: Context<CounterInitialize>, initial_count: u64) -> Result<()> {
        msg!(
            "Initializing counter with count {} for user {}",
            initial_count,
            ctx.accounts.user.key()
        );

        ctx.accounts.counter.count = initial_count;

        msg!("Account initialized");

        Ok(())
    }

    pub fn increment(ctx: Context<CounterIncrement>) -> Result<()> {
        msg!(
            "Incrementing counter for user {}, counter account: {}",
            ctx.accounts.user.key(),
            ctx.accounts.counter.key()
        );

        ctx.accounts.counter.count += 1;

        msg!("Counter incremented to {}", ctx.accounts.counter.count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CounterInitialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<Counter>(),
    )]
    counter: Account<'info, Counter>,

    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
struct CounterIncrement<'info> {
    #[account(mut)]
    counter: Account<'info, Counter>,

    #[account(mut)]
    user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
