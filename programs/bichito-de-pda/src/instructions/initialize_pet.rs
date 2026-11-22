use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, state::Pet};

#[derive(Accounts)]
pub struct InitializePet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + Pet::INIT_SPACE,
        seeds = [PET_SEED, authority.key().as_ref()],
        bump
    )]
    pub pet: Account<'info, Pet>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_pet(ctx: Context<InitializePet>, name: String) -> Result<()> {
    require!(
        !name.trim().is_empty() && name.len() <= MAX_PET_NAME_LENGTH,
        ErrorCode::InvalidPetName,
    );

    let pet = &mut ctx.accounts.pet;
    pet.authority = ctx.accounts.authority.key();
    pet.name = name;
    pet.care_count = 0;
    pet.bump = ctx.bumps.pet;

    msg!("Pet {} was registered", pet.name);
    Ok(())
}
