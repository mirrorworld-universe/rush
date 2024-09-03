use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Debug, Error, FromPrimitive)]
pub enum TokenSaleError {
    #[error("Invalid account data length")]
    InvalidAccountDataLength, // 0

    #[error("Mint and Sale authority don't match")]
    MintAndSaleAuthorityMismatch, // 1

    #[error("Account must be non-executable")]
    MustBeNonExecutable, // 2

    #[error("Not a signer")]
    NeedSigner, // 3

    #[error("Unexpected PDA seeds")]
    UnexpectedPDASeeds, // 4

    #[error("Account not yet initialized")]
    AccountUninitialized, // 5

    #[error("Failed to decode hash")]
    FailedToDecodeSha256Hash, // 6

    #[error("Invalid SPL Token Program")]
    InvalidTokenProgramID, // 7

    #[error("Mint and Sale authority don't match")]
    AccountsAndTokenBaseMismatch, // 8

    #[error("Not whitelisted")]
    NotWhitelisted, // 9

    #[error("Incompatible Proof Format")]
    IncompatibleProof, // 10
}

// allow .into() for Custom Error to ProgramError conversion
impl From<TokenSaleError> for ProgramError {
    fn from(e: TokenSaleError) -> Self {
        // https://docs.rs/solana-program/latest/solana_program/program_error/enum.ProgramError.html#variant.Custom
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TokenSaleError {
    fn type_of() -> &'static str {
        "TokenSaleError"
    }
}

impl PrintProgramError for TokenSaleError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}
