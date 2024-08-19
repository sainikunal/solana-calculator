use anchor_lang::prelude::*;

#[account]
pub struct Calculator {
    pub greeting: String,
    pub remainder: i32,
    pub result: i32,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CalculatorArgs {
    pub num1: i32,
    pub num2: i32,
}

impl Calculator {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();
}
