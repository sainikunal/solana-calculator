use anchor_lang::prelude::*;
use crate::instructions::Addition;
use crate::state::CalculatorArgs;

pub fn addition_with_args(ctx: Context<Addition>, args: CalculatorArgs) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = args.num1 + args.num2;
    Ok(())
}
