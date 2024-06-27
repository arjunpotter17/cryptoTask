use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use sha2::{Sha256, Digest};

use crate::states::Escrow;
use crate::errors::errors::ErrorCode;

#[derive(Accounts)]
pub struct Payments<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"escrow_vault", escrow.key().as_ref()], 
        bump = escrow.vault_bump,
    )]
    pub escrow_vault: SystemAccount<'info>,
    #[account(
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}

impl<'info> Payments<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.escrow_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64, keyword: String) -> Result<()> {
        let mut hasher = Sha256::new();
        hasher.update(keyword.as_bytes());
        let result = hasher.finalize();
        if result.as_slice() != self.escrow.hash {
            return Err(ErrorCode::InvalidKeyword.into());
        }

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            to: self.user.to_account_info(),
            from: self.escrow_vault.to_account_info(),
        };

        let signer_seeds = &[b"escrow_vault", self.escrow.to_account_info().key.as_ref(), &[self.escrow.vault_bump]];
        let seeds = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn close(&mut self, keyword: String) -> Result<()> {
        let mut hasher = Sha256::new();
        hasher.update(keyword.as_bytes());
        let result = hasher.finalize();
        if result.as_slice() != self.escrow.hash {
            return Err(ErrorCode::InvalidKeyword.into());
        }

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            to: self.user.to_account_info(),
            from: self.escrow_vault.to_account_info(),
        };

        let signer_seeds = &[b"escrow_vault", self.escrow.to_account_info().key.as_ref(), &[self.escrow.vault_bump]];
        let seeds = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
        transfer(cpi_ctx, self.escrow_vault.lamports())?;
        Ok(())
    }
}
