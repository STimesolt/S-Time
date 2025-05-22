use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod validation {
    use super::*;

    pub fn validate_time_slice(
        ctx: Context<ValidateTimeSlice>,
        time_slice_id: String,
    ) -> Result<()> {
        let time_slice = &ctx.accounts.time_slice;
        let clock = Clock::get()?;

        // Validate time slice exists and is owned by the validator
        require!(
            time_slice.owner == ctx.accounts.validator.key(),
            ValidationError::Unauthorized
        );

        // Validate time slice is not expired
        require!(
            time_slice.end_time > clock.unix_timestamp,
            ValidationError::TimeSliceExpired
        );

        // Validate time slice duration
        let duration = time_slice.end_time - time_slice.start_time;
        require!(
            duration <= 365 * 24 * 60 * 60, // 1 year in seconds
            ValidationError::InvalidDuration
        );

        // Validate time slice is in the future
        require!(
            time_slice.start_time > clock.unix_timestamp,
            ValidationError::InvalidStartTime
        );

        Ok(())
    }

    pub fn verify_ownership(
        ctx: Context<VerifyOwnership>,
        time_slice_id: String,
    ) -> Result<()> {
        let time_slice = &ctx.accounts.time_slice;

        require!(
            time_slice.owner == ctx.accounts.owner.key(),
            ValidationError::Unauthorized
        );

        Ok(())
    }

    pub fn check_availability(
        ctx: Context<CheckAvailability>,
        time_slice_id: String,
    ) -> Result<()> {
        let time_slice = &ctx.accounts.time_slice;
        let clock = Clock::get()?;

        require!(
            time_slice.start_time > clock.unix_timestamp,
            ValidationError::TimeSliceNotAvailable
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ValidateTimeSlice<'info> {
    #[account(mut)]
    pub time_slice: Account<'info, TimeSlice>,
    #[account(mut)]
    pub validator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyOwnership<'info> {
    pub time_slice: Account<'info, TimeSlice>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckAvailability<'info> {
    pub time_slice: Account<'info, TimeSlice>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ValidationError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Time slice has expired")]
    TimeSliceExpired,
    #[msg("Invalid duration")]
    InvalidDuration,
    #[msg("Invalid start time")]
    InvalidStartTime,
    #[msg("Time slice is not available")]
    TimeSliceNotAvailable,
} 