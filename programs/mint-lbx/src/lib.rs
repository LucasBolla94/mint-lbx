use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

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

    pub fn deposit_sol_and_mint(ctx: Context<DepositSolAndMint>, amount: u64) -> Result<()> {
        let config = &ctx.accounts.config;

        // Transferência de SOL do usuário para o Vault
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.vault.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;

        // Calcular a quantidade de LBX com base na taxa de câmbio
        let lbx_amount = amount
            .checked_mul(config.exchange_rate)
            .ok_or(ErrorCode::CalculationOverflow)?;

        // Setup CPI: mintar tokens LBX para o usuário
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        // Definir as seeds como binding com tempo de vida estável
        let mint_authority_seeds: &[&[u8]] = &[
            b"mint_authority",
            &[ctx.bumps.mint_authority],
        ];
        let signer = &[mint_authority_seeds];

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer,
        );

        token::mint_to(cpi_ctx, lbx_amount)?;

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

#[derive(Accounts)]
pub struct DepositSolAndMint<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    /// CHECK: Esta conta é derivada com seed e usada apenas como autoridade de mint via PDA. A segurança é garantida pelo controle da seed.
    #[account(
        mut,
        seeds = [b"mint_authority"],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Config {
    pub owner: Pubkey,          // 32 bytes
    pub exchange_rate: u64,     // 8 bytes
}

impl Config {
    pub const LEN: usize = 32 + 8; // Total: 40 bytes
}

#[error_code]
pub enum ErrorCode {
    #[msg("Only the owner can update the exchange rate.")]
    Unauthorized,
    #[msg("Overflow ao calcular a quantidade de LBX.")]
    CalculationOverflow,
}
