use anchor_lang::prelude::*;
use crate::state::Calculator;

#[derive(Accounts)]
pub struct InitializeCalculator<'info> {
    #[account(
        init, 
        payer = user, 
        space = Calculator::SPACE,
        seeds=[b"calculator".as_ref()],
        bump,
    )]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_calculator(ctx: Context<InitializeCalculator>, init_message: String) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.greeting = init_message;
    calculator.bump = ctx.bumps.calculator;
    Ok(())
}
