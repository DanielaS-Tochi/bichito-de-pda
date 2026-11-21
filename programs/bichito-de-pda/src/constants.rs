use anchor_lang::prelude::*;

#[constant]
pub const PET_SEED: &[u8] = b"pet";

pub const MAX_PET_NAME_LENGTH: usize = 32;

pub const MAX_CARE_COUNT: u8 = 3;
