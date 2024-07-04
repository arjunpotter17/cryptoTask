use anchor_lang::prelude::*;

declare_id!("FHiSSmRJQdUmp6Z7uZjj2U99BLS8yreeTLUd6xc9ReCD");

mod states;
mod errors;

mod contexts;
use contexts::*;

#[program]
pub mod crypto_task_capstone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,  seed: u64, hash: [u8; 32], amount: u64) -> Result<()> {
        ctx.accounts.init(seed, hash, amount, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payments>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Payments>, amount: u64, keyword: String) -> Result<()> {
        ctx.accounts.withdraw(amount, keyword)?;

        Ok(())
    }

    pub fn close(ctx: Context<Payments>, keyword: String) -> Result<()> {
        ctx.accounts.close(keyword)?;

        Ok(())
    }
}
