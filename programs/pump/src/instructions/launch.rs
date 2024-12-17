use crate::{errors::PumpError, states::Config};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        seeds = [Config::SEED_PREFIX.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    #[account(
        init,
        payer = creator,
        mint::decimals = global_config.token_decimal,
        mint::authority = global_config.key(),
    )]
    token_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = creator,
        space = 8 + std::mem::size_of::<BondingCurve>(),
        seeds = [BONDING_CURVE.as_bytes(), &token_mint.key().to_bytes()],
        bump
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(address = token::ID)]
    token_program: Program<'info, Token>,
    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,
}

impl<'info> Configure<'info> {
    pub fn process(&mut self, new_config: Config) -> Result<()> {
        require!(self.config.authority.eq(&Pubkey::default())
            || self.config.authority.eq(&self.admin.key()), PumpError::NotAuthorized);

        self.config.set_inner(new_config);

        Ok(())
    }
}
