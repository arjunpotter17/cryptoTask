
use anchor_lang::error_code;


#[error_code]
pub enum ErrorCode {
    #[msg("The provided keyword does not match the stored hash.")]
    InvalidKeyword,
}