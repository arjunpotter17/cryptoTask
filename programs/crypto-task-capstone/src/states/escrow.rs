use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub hash: [u8; 32],
    pub maker: Pubkey,
    pub amount: u64,
    pub escrow_bump: u8,
    pub vault_bump: u8,
}

impl Space for Escrow {
    const INIT_SPACE: usize =8 + 32 + 8 + 32 + 8 + 1 + 1;
}
