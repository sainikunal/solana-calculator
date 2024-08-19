use anchor_lang::prelude::*;

declare_id!("5qZF7H7Md97gz7rc4wTVNkXzzSQ9HHcfMEaZ7dBbn3gf");

#[program]
pub mod solana_calculator {
    use super::*;

    pub fn initialize_calculator(ctx: Context<InitializeCalculator>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn addition(ctx: Context<Addition>, num1: i32, num2: i32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn addition_with_args(ctx: Context<Addition>, args: CalculatorArgs) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = args.num1 + args.num2;
        Ok(())
    }

    pub fn subtraction(ctx: Context<Subtraction>, num1: i32, num2: i32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn division(ctx: Context<Division>, num1: i32, num2: i32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCalculator<'info> {
    #[account(init, payer = user, space = Calculator::SPACE)]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CalculatorArgs {
    pub num1: i32,
    pub num2: i32,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub remainder: i32,
    pub result: i32,
}

impl Calculator {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();
}