use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod marketplace {
    use super::*;

    pub fn create_order(
        ctx: Context<CreateOrder>,
        time_slice_id: String,
        price: u64,
        order_type: u8,
    ) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let clock = Clock::get()?;

        require!(price > 0, MarketplaceError::InvalidPrice);
        require!(order_type <= 2, MarketplaceError::InvalidOrderType);

        order.id = format!(
            "ORDER-{}-{}-{}",
            ctx.accounts.seller.key(),
            time_slice_id,
            clock.slot
        );
        order.time_slice_id = time_slice_id;
        order.seller = ctx.accounts.seller.key();
        order.price = price;
        order.order_type = order_type;
        order.status = 0; // PENDING
        order.created_at = clock.unix_timestamp;

        Ok(())
    }

    pub fn execute_order(ctx: Context<ExecuteOrder>) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let time_slice = &mut ctx.accounts.time_slice;

        require!(order.status == 0, MarketplaceError::OrderNotActive);
        require!(
            order.seller == time_slice.owner,
            MarketplaceError::Unauthorized
        );

        // Transfer time slice ownership
        time_slice.owner = ctx.accounts.buyer.key();
        order.status = 1; // EXECUTED

        Ok(())
    }

    pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
        let order = &mut ctx.accounts.order;

        require!(order.status == 0, MarketplaceError::OrderNotActive);
        require!(
            order.seller == ctx.accounts.seller.key(),
            MarketplaceError::Unauthorized
        );

        order.status = 2; // CANCELLED
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + Order::LEN,
        seeds = [
            b"order",
            seller.key().as_ref(),
            time_slice_id.as_bytes(),
        ],
        bump
    )]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteOrder<'info> {
    #[account(mut)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub time_slice: Account<'info, TimeSlice>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Order {
    pub id: String,
    pub time_slice_id: String,
    pub seller: Pubkey,
    pub price: u64,
    pub order_type: u8,
    pub status: u8,
    pub created_at: i64,
}

impl Order {
    pub const LEN: usize = 32 + // id
        32 + // time_slice_id
        32 + // seller
        8 + // price
        1 + // order_type
        1 + // status
        8; // created_at
}

#[error_code]
pub enum MarketplaceError {
    #[msg("Price must be greater than 0")]
    InvalidPrice,
    #[msg("Invalid order type")]
    InvalidOrderType,
    #[msg("Order is not active")]
    OrderNotActive,
    #[msg("Unauthorized access")]
    Unauthorized,
} 