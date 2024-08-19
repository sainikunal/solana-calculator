use anchor_lang::prelude::*;
use crate::state::Calculator;

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

pub fn division(ctx: Context<Division>, num1: i32, num2: i32) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = num1 / num2;
    calculator.remainder = num1 % num2;
    Ok(())
}
