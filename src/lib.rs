use anchor_lang::prelude::*;

declare_id!("SwapTOken");

#[program]
pub mod my_project {
    use super::*;

    pub fn create_account(ctx: Context<CreateAccount>) -> ProgramResult {
        let account = ctx.accounts.account;
        account.data.initialize(&mut ctx.banks_mut());
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>) -> ProgramResult {
        let from_account = ctx.accounts.from_account;
        let to_account = ctx.accounts.to_account;
        let amount = from_account.data.amount;
        to_account.data.add_amount(amount);
        from_account.data.subtract_amount(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAccount<'a> {
    #[account(mut)]
    pub account: Account<'a, MyAccount>,
    pub system_program: Sysvar<'a, System>,
}

#[derive(Accounts)]
pub struct Transfer<'a> {
    #[account(mut)]
    pub from_account: Account<'a, MyAccount>,
    #[account(mut)]
    pub to_account: Account<'a, MyAccount>,
    pub system_program: Sysvar<'a, System>,
}

#[derive(Debug, Default)]
pub struct MyAccount {
    pub amount: u64,
}
