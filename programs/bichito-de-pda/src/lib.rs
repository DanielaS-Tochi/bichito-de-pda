pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ECoVYdegYcMrm8tC7sW1FKa8fAjY1bsQqwttj5xG6Kwr");

#[program]
pub mod bichito_de_pda {
    use super::*;

    pub fn initialize_pet(ctx: Context<InitializePet>, name: String) -> Result<()> {
        crate::instructions::initialize_pet::handle_initialize_pet(ctx, name)
    }

    pub fn care_for_pet(ctx: Context<CareForPet>) -> Result<()> {
        crate::instructions::care_for_pet::handle_care_for_pet(ctx)
    }

    pub fn close_pet(ctx: Context<ClosePet>) -> Result<()> {
        crate::instructions::close_pet::handle_close_pet(ctx)
    }
}
