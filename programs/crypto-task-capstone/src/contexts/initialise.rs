use anchor_lang::prelude::*;
use crate::states::Escrow;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"escrow", user.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Escrow::INIT_SPACE,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        seeds = [b"escrow_vault", escrow.key().as_ref()], // Ensure the seed is "escrow_vault"
        bump,
    )]
    pub escrow_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, seed: u64, hash: [u8; 32], amount: u64, bumps: &InitializeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            hash,
            seed,
            maker: self.user.key(),
            amount,
            escrow_bump: bumps.escrow,
            vault_bump: bumps.escrow_vault,
        });
        msg!("User Public Key: {}", self.user.key());
        msg!("Seed value (as bytes): {:?}", seed.to_le_bytes());
        msg!("Used bump seed: {}", self.escrow.escrow_bump);
        msg!("Used vault bump seed: {}", self.escrow.vault_bump);
        Ok(())
    }
}
