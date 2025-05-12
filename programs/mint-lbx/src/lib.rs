use anchor_lang::prelude::*;

declare_id!("3pMtMuxW8C9fJ5Va7sGD57zq6s1h2btucUbAgDM5EepE");

#[program]
pub mod mint_lbx {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, exchange_rate: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.owner = ctx.accounts.authority.key();
        config.exchange_rate = exchange_rate;
        Ok(())
    }

    pub fn update_exchange_rate(ctx: Context<UpdateExchangeRate>, new_rate: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;

        require!(
            ctx.accounts.authority.key() == config.owner,
            ErrorCode::Unauthorized
        );

        config.exchange_rate = new_rate;
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(exchange_rate: u64)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Config::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateExchangeRate<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,
}




#[account]
pub struct Config {
    pub owner: Pubkey,  // 32 bytes
    pub exchange_rate: u64 // 8 bytes 
}

impl Config {
    pub const LEN: usize = 32 + 8; // Total: 40  Bytes 
}


#[error_code]
pub enum ErrorCode {
    #[msg("Only the owner can update the exchange rate.")]
    Unauthorized,
}