use anchor_lang::prelude::*;
use crate::state::Calculator;

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

pub fn subtraction(ctx: Context<Subtraction>, num1: i32, num2: i32) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = num1 - num2;
    Ok(())
}
