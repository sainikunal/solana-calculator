mod state;
mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("5qZF7H7Md97gz7rc4wTVNkXzzSQ9HHcfMEaZ7dBbn3gf");

#[program]
pub mod solana_calculator {
    use super::*;

    pub fn initialize_calculator(ctx: Context<InitializeCalculator>, init_message: String) -> Result<()> {
        instructions::initialize_calculator(ctx, init_message)
    }

    pub fn addition(ctx: Context<Addition>, num1: i32, num2: i32) -> Result<()> {
        instructions::addition(ctx, num1, num2)
    }

    pub fn addition_with_args(ctx: Context<Addition>, args: CalculatorArgs) -> Result<()> {
        instructions::addition_with_args(ctx, args)
    }

    pub fn subtraction(ctx: Context<Subtraction>, num1: i32, num2: i32) -> Result<()> {
        instructions::subtraction(ctx, num1, num2)
    }

    pub fn division(ctx: Context<Division>, num1: i32, num2: i32) -> Result<()> {
        instructions::division(ctx, num1, num2)
    }
}
