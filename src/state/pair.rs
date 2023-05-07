use anchor_lang::prelude::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    system_program::SystemProgram,
    transaction::Transaction,
};

#[derive(Accounts)]
pub struct PairInstructionArgs {
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub swap_source: AccountInfo<'info>,
    #[account(mut)]
    pub swap_destination: AccountInfo<'info>,
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,
    #[account(address = SystemProgram::id())]
    pub system_program: AccountInfo<'info>,
}

#[program]
pub mod my_token_swap {
    use super::*;

    pub const PAIR_INSTRUCTION_ID: InstructionId = 4;

    pub fn pair(ctx: Context<PairInstructionArgs>, amount: u64, token: String) -> ProgramResult {
        // Check if the payer has enough funds to cover the swap.
        let payer_balance = ctx.accounts.payer.lamports();
        if payer_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        // Check if the swap source account has enough tokens to cover the swap.
        let swap_source_balance = ctx.accounts.swap_source.load_len();
        if swap_source_balance < amount {
            return Err(ProgramError::InvalidAccountData);
        }

        // Update the swap source account.
        let new_swap_source_balance = swap_source_balance - amount;
        ctx.accounts.swap_source.save_len(new_swap_source_balance);

        // Deposit the swapped tokens into the swap destination account.
        ctx.accounts.swap_destination.try_deposit(amount)?;

        // Send the fee to the fee account.
        ctx.accounts.fee_account.try_deposit(amount / 10)?;

        Ok(())
    }
}