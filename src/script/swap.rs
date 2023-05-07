use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap<'a> {
    #[account(mut)]
    pub from_account: Account<'a, MyAccount>,
    #[account(mut)]
    pub to_account: Account<'a, MyAccount>,
    pub amount: u64,
    pub system_program: Sysvar<'a, System>,
}

impl<'a> Swap<'a> {
    pub fn swap(ctx: Context<Self>, amount: u64) -> ProgramResult {
        let from_account = ctx.accounts.from_account;
        let to_account = ctx.accounts.to_account;
        let from_amount = from_account.data.amount;
        let to_amount = to_account.data.amount;

        if from_amount < amount {
            return Err(ProgramError::InvalidArgument);
        }

        from_account.data.subtract_amount(amount);
        to_account.data.add_amount(amount);

        Ok(())
    }
}
