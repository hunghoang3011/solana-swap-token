use anchor_lang::prelude::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    system_program::SystemProgram,
    transaction::Transaction,
};

#[derive(Accounts)]
pub struct DepositInstructionArgs {
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub deposit_source: AccountInfo<'info>,
    #[account(mut)]
    pub deposit_destination: AccountInfo<'info>,
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,
    #[account(address = SystemProgram::id())]
    pub system_program: AccountInfo<'info>,
}

#[program]
pub mod my_token_swap {
    use super::*;

    pub const DEPOSIT_INSTRUCTION_ID: InstructionId = 1;

    pub fn deposit(ctx: Context<DepositInstructionArgs>, amount: u64, token: String) -> ProgramResult {
        // Check if the payer has enough funds to cover the deposit.
        let payer_balance = ctx.accounts.payer.lamports();
        if payer_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        // Check if the deposit source account has enough tokens to cover the deposit.
        let deposit_source_balance = ctx.accounts.deposit_source.load_len();
        if deposit_source_balance < amount {
            return Err(ProgramError::InvalidAccountData);
        }

        // Update the deposit source account.
        let new_deposit_source_balance = deposit_source_balance - amount;
        ctx.accounts.deposit_source.save_len(new_deposit_source_balance);

        // Deposit the tokens into the deposit destination account.
        ctx.accounts.deposit_destination.try_deposit(amount)?;

        // Send the fee to the fee account.
        ctx.accounts.fee_account.try_deposit(amount / 10)?;

        Ok(())
    }
}