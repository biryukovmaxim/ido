#![feature(arbitrary_enum_discriminant)]

use crate::space::Space;
use crate::state::StateMachineWrapper;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use num::Integer;

mod error;
mod space;
mod state;

declare_id!("8NbWKoUtHtRUd2RP13B9YVB2rS5XAqT1ny5BzijHhVAn");

const PRICE_MULTIPLAYER: TokenPrice = TokenPrice {
    numerator: 103,
    denominator: 100,
};

#[program]
pub mod ido {
    use super::*;

    #[access_control(validate_ido_times(ido_times))]
    pub fn initialize(
        ctx: Context<Initialize>,
        init_ido_tokens_amount: u64,
        ido_times: IdoTimes,
        token_price: TokenPrice,
    ) -> Result<()> {
        ctx.accounts.state.ido_authority = ctx.accounts.ido_authority.key();
        ctx.accounts.state.ido_token = ctx.accounts.ido_token.key();
        ctx.accounts.state.stable_token = ctx.accounts.stable_token.key();
        ctx.accounts.state.stable_token_pool = ctx.accounts.stable_pool.key();
        ctx.accounts.state.ido_tokens_amount = init_ido_tokens_amount;
        ctx.accounts.state.ido_times = ido_times;

        let tmp = TokenPrice {
            numerator: token_price.numerator * ctx.accounts.ido_token.decimals as u64,
            denominator: token_price.denominator * ctx.accounts.stable_token.decimals as u64,
        };
        let gcd = tmp.numerator.gcd(&tmp.denominator);
        ctx.accounts.state.token_price = TokenPrice {
            numerator: tmp.numerator / gcd,
            denominator: tmp.denominator / gcd,
        };

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(init_ido_tokens_amount: u64, ido_times: IdoTimes, token_price: TokenPrice)]
pub struct Initialize<'info> {
    #[account(
    init,
    seeds = [b"state"],
    bump,
    payer = ido_authority,
    space = 8 + ProgramState::space()
    )]
    pub state: Account<'info, ProgramState>,
    #[account(
    init,
    payer = ido_authority,
    mint::decimals = 6,
    mint::authority = this_program,
    )]
    pub ido_token: Account<'info, Mint>,
    #[account(
    init,
    payer = ido_authority,
    associated_token::mint = stable_token,
    associated_token::authority = this_program,
    )]
    pub stable_pool: Account<'info, TokenAccount>,
    pub stable_token: Account<'info, Mint>,

    #[account(mut)]
    pub ido_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address = crate::ID)]
    pub this_program: Program<'info, IDO>,
}

#[account]
pub struct ProgramState {
    pub ido_authority: Pubkey,
    pub state_number: u64,
    pub current_round: StateMachineWrapper,
    pub order_number: u64,
    pub ido_token: Pubkey,
    pub stable_token: Pubkey,
    pub stable_token_pool: Pubkey,
    pub ido_tokens_amount: u64,
    pub token_price: TokenPrice,
    pub ido_times: IdoTimes,
}

impl Space for ProgramState {
    fn space() -> usize {
        32 + 8
            + StateMachineWrapper::space()
            + 8
            + 32
            + 32
            + 32
            + 8
            + TokenPrice::space()
            + IdoTimes::space()
    }
}

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct IdoTimes {
    pub start_at: i64,
    pub end_at: i64,
    pub default_round_length: i64,
}

impl Space for IdoTimes {
    fn space() -> usize {
        24
    }
}

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct TokenPrice {
    pub numerator: u64,
    pub denominator: u64,
}

impl Space for TokenPrice {
    fn space() -> usize {
        16
    }
}

#[derive(Clone)]
pub struct IDO;

impl anchor_lang::Id for IDO {
    fn id() -> Pubkey {
        crate::ID
    }
}

// Asserts the IDO starts in the future.
fn validate_ido_times(ido_times: IdoTimes) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    match ido_times {
        IdoTimes {
            start_at,
            end_at: _,
            default_round_length: _,
        } if start_at <= current_time => err!(error::ErrorCode::IdoFuture),
        IdoTimes {
            start_at,
            end_at,
            default_round_length,
        } if (end_at - start_at) < default_round_length => err!(error::ErrorCode::IdoShort),
        IdoTimes {
            start_at,
            end_at,
            default_round_length: _,
        } if (end_at - start_at) < 0 => err!(error::ErrorCode::SeqTimes),
        _ => Ok(()),
    }
}

fn get_new_price(old_price: TokenPrice) -> TokenPrice {
    let tmp = TokenPrice {
        numerator: old_price.numerator * PRICE_MULTIPLAYER.numerator,
        denominator: old_price.denominator * PRICE_MULTIPLAYER.denominator,
    };
    let gcd = tmp.numerator.gcd(&tmp.denominator);
    TokenPrice {
        numerator: tmp.numerator / gcd,
        denominator: tmp.denominator / gcd,
    }
}
