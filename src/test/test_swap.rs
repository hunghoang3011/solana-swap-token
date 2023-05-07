use anchor_lang::prelude::*;

use crate::{Swap, SwapState};

#[test]
fn test_initialize() {
    let mut program = Program::new("swap", None);

    // Create the authority account.
    let authority = Keypair::new();

    // Create the move token account.
    let move_token_account = Keypair::new();

    // Create the sol token account.
    let sol_token_account = Keypair::new();

    // Create the fee account.
    let fee_account = Keypair::new();

    // Create the swap state account.
    let swap_state_account = Keypair::new();

    // Create the context.
    let mut context = Context::new(&program, &authority);
    context.accounts.authority = authority;
    context.accounts.move_token_account = move_token_account;
    context.accounts.sol_token_account = sol_token_account;
    context.accounts.fee_account = fee_account;
    context.accounts.swap_state = swap_state_account;

    // Initialize the swap.
    let res = program.initialize(&mut context);
    assert_ok!(res);

    // Check that the swap state was initialized correctly.
    let swap_state = SwapState::load(&mut context.accounts.swap_state)?;
    assert_eq!(swap_state.move_supply, 0);
    assert_eq!(swap_state.sol_supply, 0);
    assert_eq!(swap_state.fee_rate, 1000);
}

#[test]
fn test_deposit_move() {
    let mut program = Program::new("swap", None);

    // Create the authority account.
    let authority = Keypair::new();

    // Create the move token account.
    let move_token_account = Keypair::new();

    // Create the sol token account.
    let sol_token_account = Keypair::new();

    // Create the fee account.
    let fee_account = Keypair::new();

    // Create the swap state account.
    let swap_state_account = Keypair::new();

    // Initialize the swap.
    let res = program.initialize(&mut program.context, &authority);
    assert_ok!(res);

    // Deposit 10 MOVE tokens.
    let amount = 10;
    let mut context = program.context.clone();
    context.accounts.move_token_account = move_token_account;
    res = program.deposit_move(&mut context, &authority, amount);
    assert_ok!(res);

    // Check that the move supply was increased correctly.
    let swap_state = SwapState::load(&mut context.accounts.swap_state)?;
    assert_eq!(swap_state.move_supply, amount);
}

#[test]
fn test_deposit_sol() {
    let mut program = Program::new("swap", None);

    // Create the authority account.
    let authority = Keypair::new();

    // Create the move token account.
    let move_token_account = Keypair::new();

    // Create the sol token account.
    let sol_token_account = Keypair::new();

    // Create the fee account.
    let fee_account = Keypair::new();

    // Create the swap state account.
    let swap_state_account = Keypair::new();

    // Initialize the swap.
    let res = program.initialize(&mut program.context, &authority);
    assert_ok!(res);

    // Deposit 1 SOL.
    let amount = 1_000_000_000;
    let mut context = program.context.clone();
    context.accounts.sol_token_account = sol_token_account;
    res = program.deposit_sol(&mut context, &authority, amount);
    assert_ok!(res
