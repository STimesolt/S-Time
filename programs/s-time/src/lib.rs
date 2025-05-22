use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("TimeAssetManager111111111111111111111111111111111");

#[program]
pub mod s_time {
    use super::*;

    pub fn create_time_slice(
        ctx: Context<CreateTimeSlice>,
        start_time: i64,
        end_time: i64,
        metadata: String,
    ) -> Result<()> {
        let time_slice = &mut ctx.accounts.time_slice;
        let clock = Clock::get()?;

        require!(
            start_time > clock.unix_timestamp,
            TimeSliceError::InvalidStartTime
        );
        require!(end_time > start_time, TimeSliceError::InvalidEndTime);
        require!(
            end_time - start_time <= 31536000000, // 1 year in milliseconds
            TimeSliceError::DurationTooLong
        );

        time_slice.id = format!(
            "STIME-{}-{}-{}",
            ctx.accounts.owner.key(),
            start_time,
            end_time
        );
        time_slice.start_time = start_time;
        time_slice.end_time = end_time;
        time_slice.owner = ctx.accounts.owner.key();
        time_slice.mint_info = MintInfo {
            block_height: clock.slot,
            transaction_hash: ctx.accounts.owner.key().to_string(),
        };
        time_slice.permission_level = 0;
        time_slice.rarity_score = 0;
        time_slice.status = 0;
        time_slice.metadata = metadata;

        Ok(())
    }

    pub fn transfer_time_slice(
        ctx: Context<TransferTimeSlice>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let time_slice = &mut ctx.accounts.time_slice;
        require!(
            time_slice.owner == ctx.accounts.owner.key(),
            TimeSliceError::Unauthorized
        );

        time_slice.owner = new_owner;
        Ok(())
    }

    pub fn update_permission_level(
        ctx: Context<UpdatePermissionLevel>,
        new_level: u8,
    ) -> Result<()> {
        let time_slice = &mut ctx.accounts.time_slice;
        require!(
            time_slice.owner == ctx.accounts.owner.key(),
            TimeSliceError::Unauthorized
        );

        time_slice.permission_level = new_level;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTimeSlice<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + TimeSlice::LEN,
        seeds = [
            b"time_slice",
            owner.key().as_ref(),
            start_time.to_le_bytes().as_ref(),
            end_time.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub time_slice: Account<'info, TimeSlice>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTimeSlice<'info> {
    #[account(mut)]
    pub time_slice: Account<'info, TimeSlice>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePermissionLevel<'info> {
    #[account(mut)]
    pub time_slice: Account<'info, TimeSlice>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TimeSlice {
    pub id: String,
    pub start_time: i64,
    pub end_time: i64,
    pub owner: Pubkey,
    pub mint_info: MintInfo,
    pub permission_level: u8,
    pub rarity_score: u8,
    pub status: u8,
    pub metadata: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintInfo {
    pub block_height: u64,
    pub transaction_hash: String,
}

impl TimeSlice {
    pub const LEN: usize = 32 + // id
        8 + // start_time
        8 + // end_time
        32 + // owner
        8 + // block_height
        32 + // transaction_hash
        1 + // permission_level
        1 + // rarity_score
        1 + // status
        32; // metadata
}

#[error_code]
pub enum TimeSliceError {
    #[msg("Start time must be in the future")]
    InvalidStartTime,
    #[msg("End time must be after start time")]
    InvalidEndTime,
    #[msg("Time slice duration cannot exceed 1 year")]
    DurationTooLong,
    #[msg("Unauthorized access")]
    Unauthorized,
} 