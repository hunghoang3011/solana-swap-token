use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap {
    #[account(mut)]
    authority: AccountInfo<'info>,
    #[account(mut)]
    move_token_account: AccountInfo<'info>,
    #[account(mut)]
    sol_token_account: AccountInfo<'info>,
    #[account(zero)]
    fee_account: AccountInfo<'info>,
    #[account(mut)]
    swap_state: AccountInfo<'info, SwapState>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SwapState {
    move_supply: u64,
    sol_supply: u64,
    fee_rate: u64,
}

impl Swap {
    pub fn deposit_move(&mut self, amount: u64) -> DispatchResult {
        self.move_token_account.lamports += amount;
        self.swap_state.move_supply += amount;
        Ok(())
    }

    pub fn deposit_sol(&mut self, amount: u64) -> DispatchResult {
        self.sol_token_account.lamports += amount;
        self.swap_state.sol_supply += amount;
        Ok(())
    }

    pub fn swap_move_for_sol(&mut self, amount: u64) -> DispatchResult {
        let sol_amount = amount / self.swap_state.fee_rate;
        self.sol_token_account.lamports -= sol_amount;
        self.move_token_account.lamports += amount;
        self.swap_state.move_supply -= amount;
        self.swap_state.sol_supply += sol_amount;
        Ok(())
    }

    pub fn swap_sol_for_move(&mut self, amount: u64) -> DispatchResult {
        let move_amount = amount * self.swap_state.fee_rate;
        self.move_token_account.lamports -= move_amount;
        self.sol_token_account.lamports += amount;
        self.swap_state.move_supply += move_amount;
        self.swap_state.sol_supply -= amount;
        Ok(())
    }
}
