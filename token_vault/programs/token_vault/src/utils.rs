pub fn _convert_to_shares(
    total_supply: u64,
    total_assets: u64,
    assets: u64,
    round_up: bool,
) -> u64 {
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

pub fn _convert_to_assets(
    total_supply: u64,
    total_assets: u64,
    shares: u64,
    round_up: bool,
) -> u64 {
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

pub fn _max_deposit() -> u64 {
    u64::MAX
}

pub fn _max_mint() -> u64 {
    u64::MAX
}

pub fn _max_withdraw(total_supply: u64, total_assets: u64, shares_amount: u64) -> u64 {
    _convert_to_assets(total_supply, total_assets, shares_amount, false)
}

pub fn _max_redeem(shares_amount: u64) -> u64 {
    shares_amount
}

pub fn _preview_deposit(total_supply: u64, total_assets: u64, assets: u64) -> u64 {
    _convert_to_shares(total_supply, total_assets, assets, false)
}

pub fn _preview_mint(total_supply: u64, total_assets: u64, shares: u64) -> u64 {
    _convert_to_assets(total_supply, total_assets, shares, true)
}

pub fn _preview_redeem(total_supply: u64, total_assets: u64, shares: u64) -> u64 {
    _convert_to_assets(total_supply, total_assets, shares, false)
}

pub fn _preview_withdraw(total_supply: u64, total_assets: u64, shares: u64) -> u64 {
    _convert_to_assets(total_supply, total_assets, shares, false)
}
