use anchor_lang::prelude::*;
use solana_program::program_error::ProgramError;

declare_id!("D9MxitF878nXCgeTyiUYXGjsC8hh55HY2RzVnMRLwJSJ");

#[program]
pub mod sbpf_to_anchor {
    use super::*;

    // Main functions that dispatch to specific implementations
    pub fn is_valid(ctx: Context<IsValid>, dex_type: u8) -> Result<bool> {
        if dex_type == 0 {
            // Raydium
            let accounts = &ctx.accounts;
            raydium_is_valid(accounts.input_data.to_account_info())
        } else {
            // Default to false for unknown dex types
            Ok(false)
        }
    }

    pub fn get_quote(ctx: Context<GetQuote>, dex_type: u8) -> Result<u64> {
        let accounts = &ctx.accounts;

        if dex_type == 0 {
            // Raydium
            raydium_get_quote(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )
        } else if dex_type == 1 {
            // Pump Fun
            pump_fun_get_quote(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )
        } else {
            Err(ProgramError::InvalidArgument.into())
        }
    }

    pub fn get_liquidity(ctx: Context<GetLiquidity>, dex_type: u8) -> Result<(u64, u64)> {
        let accounts = &ctx.accounts;

        if dex_type == 0 {
            // Add this value to the first part of output to indicate Raydium
            let result = raydium_get_liquidity(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )?;
            Ok(result)
        } else if dex_type == 1 {
            // Add this value to the first part of output to indicate PumpFun
            let result = pump_fun_get_liquidity(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )?;
            Ok(result)
        } else {
            Err(ProgramError::InvalidArgument.into())
        }
    }

    pub fn get_quote_and_liquidity(
        ctx: Context<GetQuoteAndLiquidity>,
        dex_type: u8,
    ) -> Result<(u64, u64, u64)> {
        let accounts = &ctx.accounts;

        if dex_type == 0 {
            // Raydium
            raydium_get_quote_and_liquidity(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )
        } else if dex_type == 1 {
            // Pump Fun
            pump_fun_get_quote_and_liquidity(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )
        } else {
            Err(ProgramError::InvalidArgument.into())
        }
    }

    pub fn calculate_profit_optimised(ctx: Context<CalculateProfitOptimised>) -> Result<u64> {
        let accounts = &ctx.accounts;

        // Implements the calculate_profit_optimised function logic
        let amount = accounts.amount.amount;

        // We need to call each function directly instead of referencing the instruction
        // Get quote and liquidity first
        let (quote1, reserve_a, reserve_b) = if accounts.dex_type.dex_type == 0 {
            raydium_get_quote_and_liquidity(accounts.quote_ctx.clone(), amount, false)?
        } else {
            pump_fun_get_quote_and_liquidity(accounts.quote_ctx.clone(), amount, false)?
        };

        // Get liquidity for the reverse direction
        let (reverse_reserve_a, reverse_reserve_b) = if accounts.dex_type_reverse.dex_type == 0 {
            raydium_get_liquidity(accounts.liquidity_ctx.clone(), amount, true)?
        } else {
            pump_fun_get_liquidity(accounts.liquidity_ctx.clone(), amount, true)?
        };

        // Get the final quote
        let quote2 = if accounts.dex_type_reverse.dex_type == 0 {
            raydium_get_quote(accounts.quote_ctx_reverse.clone(), amount, true)?
        } else {
            pump_fun_get_quote(accounts.quote_ctx_reverse.clone(), amount, true)?
        };

        // Calculate profit: output minus input amount
        Ok(quote2.saturating_sub(amount))
    }

    pub fn calculate_hinted_max_amount_optimised(
        ctx: Context<CalculateHintedMaxAmountOptimised>,
    ) -> Result<u64> {
        let accounts = &ctx.accounts;
        let max_input = accounts.max_input.amount;
        let available = accounts.available.amount;
        let fee_numerator = accounts.fee_numerator.amount;
        let fee_denominator = accounts.fee_denominator.amount;

        if max_input > available {
            return Ok(0);
        }

        let amount = available.saturating_sub(max_input);
        let fee_adjusted = 10000u64.saturating_sub(fee_numerator);

        let mut result;
        if amount > 0x68db8bac710cc {
            result = amount / fee_adjusted * 10000;
        } else {
            result = amount * 10000 / fee_adjusted;
        }

        if result > 0x68db8bac710cc {
            result = result / 10000 * fee_denominator;
        } else {
            result = result * fee_denominator / 10000;
        }

        Ok(result)
    }

    pub fn calculate_upper_bound_optimised(
        ctx: Context<CalculateUpperBoundOptimised>,
    ) -> Result<u64> {
        let accounts = &ctx.accounts;
        let dex_type = accounts.dex_type.dex_type;
        let amount = accounts.amount.amount;

        // Default fee rate is 9975 (0.25% fee)
        let mut fee_rate = 9975u64;

        // If dex_type is 1, use 9900 (1% fee)
        if dex_type == 1 {
            fee_rate = 9900;
        }

        // Get the appropriate amount based on the is_token_a flag
        let available = if accounts.is_token_a.is_token_a == 0 {
            accounts.amounts.token_a_amount
        } else {
            accounts.amounts.token_b_amount
        };

        if available > amount {
            let remaining = amount.saturating_sub(available);
            let output_amount;

            if remaining > 0x68db8bac710cc {
                output_amount = remaining / fee_rate * 10000;
            } else {
                output_amount = remaining * 10000 / fee_rate;
            }

            let result;
            let multiplier = accounts.multiplier.amount;

            if output_amount > 0x68db8bac710cc {
                result = output_amount / 10000 * multiplier;
            } else {
                result = output_amount * multiplier / 10000;
            }

            Ok(result)
        } else {
            Ok(0)
        }
    }

    pub fn calculate_optimal_strategy_optimised(
        ctx: Context<CalculateOptimalStrategyOptimised>,
    ) -> Result<bool> {
        // 这个函数的逻辑过于复杂，需要更详细的拆解工作
        // 基本结构已添加，实际实现需要更多分析
        Ok(true)
    }
}

// Internal implementation functions
fn raydium_is_valid(input_data: AccountInfo) -> Result<bool> {
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // Check if both amounts are greater than 1000
    Ok(amount_a > 1000 && amount_b > 1000)
}

fn raydium_get_quote(input_data: AccountInfo, amount: u64, reverse: bool) -> Result<u64> {
    // Implementing the complex math from the assembly
    // This is a simplified version for demonstration
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // Adjust the fee - 0.25% fee (25 basis points)
    let adjusted_amount = amount - (amount * 25) / 10000;

    let quote = if !reverse {
        // amount_a to amount_b calculation
        if amount_a == 0 {
            return Ok(0);
        }
        amount_b * adjusted_amount / amount_a
    } else {
        // amount_b to amount_a calculation
        if amount_b == 0 {
            return Ok(0);
        }
        amount_a * adjusted_amount / amount_b
    };

    Ok(quote)
}

fn raydium_get_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64)> {
    // This is a simplified implementation that returns token amounts
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // In actual implementation we would calculate reserves based on the formula
    let reserve_a = amount_a;
    let reserve_b = amount_b;

    Ok((reserve_a, reserve_b))
}

fn raydium_get_quote_and_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64, u64)> {
    // Get the liquidity first
    let (reserve_a, reserve_b) = raydium_get_liquidity(input_data.clone(), amount, reverse)?;

    // Then get the quote
    let quote = raydium_get_quote(input_data, amount, reverse)?;

    Ok((quote, reserve_a, reserve_b))
}

fn pump_fun_get_quote(input_data: AccountInfo, amount: u64, reverse: bool) -> Result<u64> {
    // Similar to raydium_get_quote but with different parameters
    // This is a placeholder implementation
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // Adjust the fee - 1% fee (100 basis points)
    let adjusted_amount = amount - (amount * 100) / 10000;

    let quote = if !reverse {
        // amount_a to amount_b calculation
        if amount_a == 0 {
            return Ok(0);
        }
        amount_b * adjusted_amount / amount_a
    } else {
        // amount_b to amount_a calculation
        if amount_b == 0 {
            return Ok(0);
        }
        amount_a * adjusted_amount / amount_b
    };

    Ok(quote)
}

fn pump_fun_get_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64)> {
    // Similar to raydium_get_liquidity but with pump_fun specific logic
    // This is a placeholder implementation
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // In actual implementation we would calculate reserves based on the formula
    let reserve_a = amount_a;
    let reserve_b = amount_b;

    Ok((reserve_a, reserve_b))
}

fn pump_fun_get_quote_and_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64, u64)> {
    // Get the liquidity first
    let (reserve_a, reserve_b) = pump_fun_get_liquidity(input_data.clone(), amount, reverse)?;

    // Then get the quote
    let quote = pump_fun_get_quote(input_data, amount, reverse)?;

    Ok((quote, reserve_a, reserve_b))
}

// Account structures for the instructions
#[derive(Accounts)]
pub struct IsValid<'info> {
    pub input_data: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetQuote<'info> {
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
}

#[derive(Accounts)]
pub struct GetLiquidity<'info> {
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
}

#[derive(Accounts)]
pub struct GetQuoteAndLiquidity<'info> {
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
}

#[derive(Accounts)]
pub struct CalculateProfitOptimised<'info> {
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
    #[account(mut)]
    pub dex_type_reverse: Account<'info, DexType>,
    pub quote_ctx: AccountInfo<'info>,
    pub liquidity_ctx: AccountInfo<'info>,
    pub quote_ctx_reverse: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CalculateHintedMaxAmountOptimised<'info> {
    #[account(mut)]
    pub max_input: Account<'info, AmountData>,
    #[account(mut)]
    pub available: Account<'info, AmountData>,
    #[account(mut)]
    pub fee_numerator: Account<'info, AmountData>,
    #[account(mut)]
    pub fee_denominator: Account<'info, AmountData>,
}

#[derive(Accounts)]
pub struct CalculateUpperBoundOptimised<'info> {
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
    #[account(mut)]
    pub amounts: Account<'info, TokenAmounts>,
    #[account(mut)]
    pub is_token_a: Account<'info, IsTokenA>,
    #[account(mut)]
    pub multiplier: Account<'info, AmountData>,
}

#[derive(Accounts)]
pub struct CalculateOptimalStrategyOptimised<'info> {
    // 需要根据函数实现细节来定义所需的账户
    pub misc_account: AccountInfo<'info>,
}

// Data structures used in the program
#[account]
pub struct AmountData {
    pub amount: u64,
}

#[account]
pub struct ReverseFlag {
    pub reverse: bool,
}

#[account]
pub struct DexType {
    pub dex_type: u8,
}

#[account]
pub struct TokenAmounts {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[account]
pub struct IsTokenA {
    pub is_token_a: u8,
}

// Custom error codes
#[error_code]
pub enum SwapError {
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid DEX type")]
    InvalidDexType,
}
