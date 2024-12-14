use anchor_lang::prelude::*;

pub mod errors;
pub mod utils;
pub mod instructions;
pub mod states;
pub mod consts;

use crate::instructions::*;

declare_id!("7RiUM3T5TE6VrqKE9ekPfn6SZNQ7Z7FEmQCyoXytdEVD");

#[program]
pub mod pump {
    use super::*;

    //  called by admin to set global config
    //  need to check the signer is authority
    pub fn configure(ctx: Context<Configure>, new_config: states::Config) -> Result<()> {
        ctx.accounts.process(new_config)
    }
}
