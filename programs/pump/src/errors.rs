use anchor_lang::prelude::*;

#[error_code]
pub enum PumpError {
    #[msg("The value is not in the expected range")]
    NotAuthorized,

    #[msg("The value is not in the expected range")]
    IncorrectValue,

    #[msg("Amount out is smaller than required amount")]
    ReturnAmountTooSmall,

    #[msg("An overflow or underflow occurred during the calculation")]
    OverflowOrUnderflowOccurred,
}
