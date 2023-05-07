use anchor_lang::prelude::*;

use crate::{Swap, SwapState};

#[program]
pub mod swap {
    use super::*;

    pub fn initialize(ctx: Context<Swap>, authority: signer::Signer) -> DispatchResult {
        let mut state = SwapState::default();
        state.move_supply = 0;
        state.sol_supply = 0;
        state.fee_rate = 1000; // 1% fee
        ctx.accounts.swap_state.save(&mut state)?;
        Ok(())
    }

    pub fn deposit_move(ctx: Context<Swap>, authority: signer::Signer, amount: u64) -> DispatchResult {
        Swap::deposit_move(&mut ctx.accounts, amount)?;
        Ok(())
    }

    pub fn deposit_sol(ctx: Context<Swap>, authority: signer::Signer, amount: u64) -> DispatchResult {
        Swap::deposit_sol(&mut ctx.accounts, amount)?;
        Ok(())
    }

    pub fn swap_move_for_sol(ctx: Context<Swap>, authority: signer::Signer, amount: u64) ->
