use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pet {
    pub authority: Pubkey,
    #[max_len(32)]
    pub name: String,
    pub care_count: u8,
    pub bump: u8,
}
