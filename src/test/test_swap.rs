use anchor_lang::prelude::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    system_program::SystemProgram,
    transaction::Transaction,
};

#[derive(Accounts)]
pub struct SwapInstructionArgs {
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

    pub const SWAP_INSTRUCTION_ID: InstructionId = 5;

    pub fn swap(ctx: Context<SwapInstructionArgs>, amount: u64, token: String) -> ProgramResult {
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

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        instruction::Instruction,
        native::system_instruction::create_account,
        signature::Signer,
        transaction::Transaction,
    };

    #[test]
    fn test_swap() {
        // Create the program signer.
        let program_id = Pubkey::new_from_array([1; 32]);
        let signer = Signer::new(&program_id);

        // Create the accounts.
        let payer_account = AccountInfo::new(Pubkey::new_from_array([2; 32]));
        let swap_source_account = AccountInfo::new(Pubkey::new_from_array([3; 32]));
        let swap_destination_account = AccountInfo::new(Pubkey::new_from_array([4; 32]));
        let fee_account = AccountInfo::new(Pubkey::new_from_array([5; 32]));

        // Create the transaction.
        let mut transaction = Transaction::new_with_payer(
            &Instruction::new_with_accounts(
                SWAP_INSTRUCTION_ID,
                &program_id,
                vec![payer_account, swap_source_account, swap_destination_account, fee_account],
                vec![],
            ),
            payer_account,
            1000000,
        );

        // Sign the transaction.
        transaction.sign(&signer, signer.keypair().secret());

        // Submit the transaction to the cluster.
        let response = solana_sdk::rpc::submit_transaction_with_confirmations(transaction, 1).unwrap();

        // Check the transaction status.
        let status = response.transaction_status();
        assert_eq!(status, TransactionStatus::Success);
    }
}
