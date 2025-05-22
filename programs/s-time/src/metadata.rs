use anchor_lang::prelude::*;

#[program]
pub mod metadata {
    use super::*;

    pub fn create_metadata(
        ctx: Context<CreateMetadata>,
        time_slice_id: String,
        title: String,
        description: String,
        tags: Vec<String>,
        custom_data: Vec<u8>,
    ) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;
        let time_slice = &ctx.accounts.time_slice;

        require!(
            time_slice.owner == ctx.accounts.owner.key(),
            MetadataError::Unauthorized
        );

        metadata.time_slice_id = time_slice_id;
        metadata.owner = ctx.accounts.owner.key();
        metadata.title = title;
        metadata.description = description;
        metadata.tags = tags;
        metadata.custom_data = custom_data;
        metadata.created_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        title: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
        custom_data: Option<Vec<u8>>,
    ) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;

        require!(
            metadata.owner == ctx.accounts.owner.key(),
            MetadataError::Unauthorized
        );

        if let Some(title) = title {
            metadata.title = title;
        }
        if let Some(description) = description {
            metadata.description = description;
        }
        if let Some(tags) = tags {
            metadata.tags = tags;
        }
        if let Some(custom_data) = custom_data {
            metadata.custom_data = custom_data;
        }

        metadata.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn delete_metadata(ctx: Context<DeleteMetadata>) -> Result<()> {
        let metadata = &ctx.accounts.metadata;

        require!(
            metadata.owner == ctx.accounts.owner.key(),
            MetadataError::Unauthorized
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Metadata::LEN,
        seeds = [b"metadata", time_slice_id.as_bytes()],
        bump
    )]
    pub metadata: Account<'info, Metadata>,
    pub time_slice: Account<'info, TimeSlice>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(mut)]
    pub metadata: Account<'info, Metadata>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteMetadata<'info> {
    #[account(mut)]
    pub metadata: Account<'info, Metadata>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Metadata {
    pub time_slice_id: String,
    pub owner: Pubkey,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub custom_data: Vec<u8>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Metadata {
    pub const LEN: usize = 32 + // time_slice_id
        32 + // owner
        32 + // title
        32 + // description
        32 + // tags (vector)
        32 + // custom_data (vector)
        8 + // created_at
        8; // updated_at
}

#[error_code]
pub enum MetadataError {
    #[msg("Unauthorized access")]
    Unauthorized,
} 