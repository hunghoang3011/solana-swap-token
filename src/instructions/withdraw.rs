use anchor_lang::prelude::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    system_program::SystemProgram,
    transaction::Transaction,
};

#[derive(Accounts)]
pub struct WithdrawInstructionArgs {
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub withdraw_source: AccountInfo<'info>,
    #[account(mut)]
    pub withdraw_destination: AccountInfo<'info>,
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,
    #[account(address = SystemProgram::id())]
    pub system_program: AccountInfo<'info>,
}

#[program]
pub mod my_token_swap {
    use super::*;

    pub const WITHDRAW_INSTRUCTION_ID: InstructionId = 2;

    pub fn withdraw(ctx: Context<WithdrawInstructionArgs>, amount: u64, token: String) -> ProgramResult {
        // Check if the payer has enough funds to cover the withdraw.
        let payer_balance = ctx.accounts.payer.lamports();
        if payer_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        // Check if the withdraw source account has enough tokens to cover the withdraw.
        let withdraw_source_balance = ctx.accounts.withdraw_source.load_len();
        if withdraw_source_balance < amount {
            return Err(ProgramError::InvalidAccountData);
        }

        // Update the withdraw source account.
        let new_withdraw_source_balance = withdraw_source_balance - amount;
        ctx.accounts.withdraw_source.save_len(new_withdraw_source_balance);

        // Withdraw the tokens from the withdraw source account.
        ctx.accounts.withdraw_destination.try_withdraw(amount)?;

        // Send the fee to the fee account.
        ctx.accounts.fee_account.try_deposit(amount / 10)?;

        Ok(())
    }
}