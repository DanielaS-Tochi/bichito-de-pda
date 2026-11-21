use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, state::Pet};

#[derive(Accounts)]
pub struct ClosePet<'info> {
    #[account(
        mut,
        close = authority,
        seeds = [PET_SEED, pet.authority.as_ref()],
        bump = pet.bump
    )]
    pub pet: Account<'info, Pet>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handle_close_pet(ctx: Context<ClosePet>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.pet.authority,
        ctx.accounts.authority.key(),
        ErrorCode::Unauthorized,
    );
    require!(
        ctx.accounts.pet.care_count == MAX_CARE_COUNT,
        ErrorCode::PetNotReadyToClose,
    );

    msg!("Pet {} completed its care journey", ctx.accounts.pet.name);
    Ok(())
}
