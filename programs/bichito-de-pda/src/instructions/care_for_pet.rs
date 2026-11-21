use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, state::Pet};

#[derive(Accounts)]
pub struct CareForPet<'info> {
    #[account(
        mut,
        seeds = [PET_SEED, pet.authority.as_ref()],
        bump = pet.bump
    )]
    pub pet: Account<'info, Pet>,
    pub authority: Signer<'info>,
}

pub fn handle_care_for_pet(ctx: Context<CareForPet>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.pet.authority,
        ctx.accounts.authority.key(),
        ErrorCode::Unauthorized,
    );
    require!(
        ctx.accounts.pet.care_count < MAX_CARE_COUNT,
        ErrorCode::CareLimitReached,
    );

    ctx.accounts.pet.care_count += 1;
    msg!(
        "{} received care {}/{}",
        ctx.accounts.pet.name,
        ctx.accounts.pet.care_count,
        MAX_CARE_COUNT
    );
    Ok(())
}
