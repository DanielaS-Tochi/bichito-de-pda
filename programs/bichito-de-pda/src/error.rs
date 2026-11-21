use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only the pet owner can update this pet")]
    Unauthorized,
    #[msg("Pet name must contain between 1 and 32 UTF-8 bytes")]
    InvalidPetName,
}
