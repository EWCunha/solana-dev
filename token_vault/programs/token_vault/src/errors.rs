use anchor_lang::prelude::error_code;

#[error_code]
pub enum TokenVaultError {
    #[msg("Deposit amount is too large")]
    DepositAmountTooLarge,

    #[msg("Mint amount is too large")]
    MintAmountTooLarge,
}
