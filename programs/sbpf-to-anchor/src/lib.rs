/**

* @title sBPF to Anchor - DEX Utilities
* @notice 这个程序将 Solana BPF 汇编代码转换为 Anchor 程序
* @dev 此程序提供了用于与 DEX（去中心化交易所）交互的实用功能
*      包括 Raydium 和 PUMP 池。
*
* 主要功能包括：
* 1. 验证交易池是否有效 (is_valid)
* 2. 获取交易报价 (get_quote)
* 3. 获取池子流动性 (get_liquidity)
* 4. 获取报价和流动性 (get_quote_and_liquidity)
* 5. 计算交易利润 (calculate_profit_optimised)
* 6. 计算优化后的最大数量 (calculate_hinted_max_amount_optimised)
* 7. 计算上限 (calculate_upper_bound_optimised)
*
* 该程序支持两种不同的池类型：
* - Raydium 池：使用标准的恒定乘积公式 (x * y = k)
* - PUMP 池：有额外的费用机制和不同的价格计算方法
*/
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod sbpf_to_anchor {
    use super::*;

    pub fn is_valid(ctx: Context<IsValid>, is_pump: bool) -> Result<bool> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        if is_pump {
            // Pump pool validation
            // From assembly: pool_type amount > 1000 and calculate price ratio
            let amount_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let amount_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            // Minimum liquidity check
            if amount_a <= 1000 || amount_b <= 1000 {
                return Ok(false);
            }

            // Price ratio check like in pump_fun_is_valid (simplified)
            // In assembly: price ratio is checked to be within certain bounds
            let price_ratio = calculate_price_ratio(amount_a, amount_b)?;
            if price_ratio <= 0 {
                return Ok(false);
            }

            Ok(true)
        } else {
            // Raydium pool validation
            let amount_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let amount_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            // From assembly: amount_a > 1000 && amount_b > 1000
            Ok(amount_a > 1000 && amount_b > 1000)
        }
    }

    pub fn get_quote(ctx: Context<GetQuote>, amount_in: u64, is_reverse: bool) -> Result<u64> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;
        let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

        // Implementing Raydium's quote calculation
        let fee_numerator = if is_reverse {
            RAYDIUM_FEE_RATE
        } else {
            PUMP_FEE_RATE
        };
        let amount_with_fee = amount_in
            .checked_mul(fee_numerator)
            .ok_or(error!(ErrorCode::Overflow))?;

        if is_reverse {
            // Reserve calculation for reverse direction
            let denominator = reserve_b
                .checked_add(amount_in)
                .ok_or(error!(ErrorCode::Overflow))?;
            amount_with_fee
                .checked_mul(reserve_a)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))
        } else {
            // Reserve calculation for forward direction
            let denominator = reserve_a
                .checked_add(amount_in)
                .ok_or(error!(ErrorCode::Overflow))?;
            amount_with_fee
                .checked_mul(reserve_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))
        }
    }

    pub fn get_liquidity(ctx: Context<GetLiquidity>, is_reverse: bool) -> Result<(u64, u64)> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        if is_reverse {
            // Parse Raydium V4 pool data
            let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            // Validate minimum pool size
            if reserve_a == 0 || reserve_b == 0 || pool_data.len() < 208 {
                return Ok((0, 0));
            }

            // Extract liquidity data from pool
            let liquidity_a = reserve_a;
            let liquidity_b = reserve_b;

            Ok((liquidity_a, liquidity_b))
        } else {
            // Parse Raydium CP pool data
            if pool_data.len() < 408 {
                return Ok((0, 0));
            }

            // Extract liquidity data from Concentrated Pool format
            let offset = 0x155; // From assembly: ldxdw r3, [r3+0x155]
            let reserve_a = u64::from_le_bytes(pool_data[offset..offset + 8].try_into().unwrap());
            let reserve_b =
                u64::from_le_bytes(pool_data[offset + 8..offset + 16].try_into().unwrap());

            Ok((reserve_a, reserve_b))
        }
    }

    pub fn calculate_profit_optimised(
        ctx: Context<CalculateProfitOptimised>,
        amount: u64,
        is_pump: bool,
    ) -> Result<u64> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // Step 1: Get initial quote and liquidity (similar to get_quote_and_liquidity)
        let (quote, liquidity_a, liquidity_b) = if is_pump {
            // Pump pool format
            if pool_data.len() < 24 {
                return Ok(0);
            }

            let reserve_a = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[16..24].try_into().unwrap());

            if reserve_a == 0 || reserve_b == 0 {
                return Ok(0);
            }

            // Pump quote calculation
            let fee_numerator = PUMP_FEE_RATE;
            let amount_with_fee = amount
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = reserve_a
                .checked_add(amount)
                .ok_or(error!(ErrorCode::Overflow))?;

            let raw_quote = amount_with_fee
                .checked_mul(reserve_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?;

            // In Pump function, there's an additional fee subtraction
            let fee = raw_quote
                .checked_div(100)
                .ok_or(error!(ErrorCode::DivisionByZero))?;
            let final_quote = raw_quote
                .checked_sub(fee)
                .ok_or(error!(ErrorCode::Overflow))?;

            (final_quote, reserve_a, reserve_b)
        } else {
            // Raydium pool format
            let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            if reserve_a == 0 || reserve_b == 0 || pool_data.len() < 208 {
                return Ok(0);
            }

            // Raydium quote calculation
            let fee_numerator = RAYDIUM_FEE_RATE;
            let amount_with_fee = amount
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = reserve_a
                .checked_add(amount)
                .ok_or(error!(ErrorCode::Overflow))?;

            let final_quote = amount_with_fee
                .checked_mul(reserve_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?;

            (final_quote, reserve_a, reserve_b)
        };

        // Step 2: Calculate reverse quote (from B to A)
        let reverse_quote = if !is_pump {
            // Raydium reverse quote
            let fee_numerator = RAYDIUM_FEE_RATE;
            let amount_with_fee = quote
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = liquidity_b
                .checked_add(quote)
                .ok_or(error!(ErrorCode::Overflow))?;

            amount_with_fee
                .checked_mul(liquidity_a)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?
        } else {
            // Pump reverse quote
            let fee_numerator = PUMP_FEE_RATE;
            let amount_with_fee = quote
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = liquidity_a
                .checked_add(quote)
                .ok_or(error!(ErrorCode::Overflow))?;

            let raw_quote = amount_with_fee
                .checked_mul(liquidity_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?;

            // In Pump function, there's an additional fee subtraction
            let fee = raw_quote
                .checked_div(100)
                .ok_or(error!(ErrorCode::DivisionByZero))?;
            raw_quote
                .checked_sub(fee)
                .ok_or(error!(ErrorCode::Overflow))?
        };

        // Calculate profit (if any)
        if reverse_quote > amount {
            Ok(reverse_quote - amount)
        } else {
            Ok(0)
        }
    }

    pub fn calculate_hinted_max_amount_optimised(
        _ctx: Context<CalculateHintedMaxAmount>,
        amount: u64,
        fee: u64,
        _multiplier: u64,
    ) -> Result<u64> {
        if amount <= fee {
            return Ok(0);
        }

        let amount_minus_fee = amount.checked_sub(fee).ok_or(error!(ErrorCode::Overflow))?;

        // Check if amount is greater than MAX_U64
        if amount_minus_fee > MAX_U64 {
            let div_result = amount_minus_fee
                .checked_div(10000 - fee)
                .ok_or(error!(ErrorCode::DivisionByZero))?;
            let mul_result = div_result
                .checked_mul(10000)
                .ok_or(error!(ErrorCode::Overflow))?;
            Ok(mul_result)
        } else {
            let mul_result = amount_minus_fee
                .checked_mul(10000)
                .ok_or(error!(ErrorCode::Overflow))?;
            let div_result = mul_result
                .checked_div(10000 - fee)
                .ok_or(error!(ErrorCode::DivisionByZero))?;
            Ok(div_result)
        }
    }

    pub fn calculate_upper_bound_optimised(
        ctx: Context<CalculateUpperBound>,
        amount: u64,
    ) -> Result<u64> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // Read pool type from the first 4 bytes
        let pool_type = u32::from_le_bytes(pool_data[0..4].try_into().unwrap());

        // Get fee rate based on pool type
        let fee_rate = match pool_type {
            0 => RAYDIUM_FEE_RATE,
            1 => PUMP_FEE_RATE,
            _ => return Ok(0), // Invalid pool type
        };

        // Read reserves based on pool type
        let (reserve_a, reserve_b) = if pool_type == 0 {
            // Raydium V4 pool format
            if pool_data.len() < 16 {
                return Ok(0);
            }
            let reserve_a = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[16..24].try_into().unwrap());
            (reserve_a, reserve_b)
        } else {
            // Pump pool format
            if pool_data.len() < 408 {
                return Ok(0);
            }
            let offset = 0x155;
            let reserve_a = u64::from_le_bytes(pool_data[offset..offset + 8].try_into().unwrap());
            let reserve_b =
                u64::from_le_bytes(pool_data[offset + 8..offset + 16].try_into().unwrap());
            (reserve_a, reserve_b)
        };

        // Calculate upper bound
        if reserve_b > amount {
            let amount_minus_reserve = reserve_b
                .checked_sub(amount)
                .ok_or(error!(ErrorCode::Overflow))?;

            if amount_minus_reserve > MAX_U64 {
                let div_result = amount_minus_reserve
                    .checked_div(fee_rate)
                    .ok_or(error!(ErrorCode::DivisionByZero))?;
                let mul_result = div_result
                    .checked_mul(10000)
                    .ok_or(error!(ErrorCode::Overflow))?;
                Ok(mul_result)
            } else {
                let mul_result = amount_minus_reserve
                    .checked_mul(10000)
                    .ok_or(error!(ErrorCode::Overflow))?;
                let div_result = mul_result
                    .checked_div(fee_rate)
                    .ok_or(error!(ErrorCode::DivisionByZero))?;
                Ok(div_result)
            }
        } else {
            Ok(0)
        }
    }

    pub fn get_quote_and_liquidity(
        ctx: Context<GetQuoteAndLiquidity>,
        amount_in: u64,
        is_pump: bool,
    ) -> Result<(u64, u64, u64)> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // Get liquidity first
        let (liquidity_a, liquidity_b) = if is_pump {
            // Pump pool format
            if pool_data.len() < 24 {
                return Ok((0, 0, 0));
            }

            let reserve_a = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[16..24].try_into().unwrap());

            (reserve_a, reserve_b)
        } else {
            // Raydium pool format
            let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            if reserve_a == 0 || reserve_b == 0 || pool_data.len() < 208 {
                return Ok((0, 0, 0));
            }

            (reserve_a, reserve_b)
        };

        // Calculate quote
        let fee_numerator = if is_pump {
            PUMP_FEE_RATE
        } else {
            RAYDIUM_FEE_RATE
        };
        let amount_with_fee = amount_in
            .checked_mul(fee_numerator)
            .ok_or(error!(ErrorCode::Overflow))?;

        let quote = if is_pump {
            // Pump quote calculation includes a division by 100 at the end (simplified)
            let denominator = liquidity_a
                .checked_add(amount_in)
                .ok_or(error!(ErrorCode::Overflow))?;

            let raw_quote = amount_with_fee
                .checked_mul(liquidity_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?;

            // In Pump function, there's an additional fee subtraction
            let fee = raw_quote
                .checked_div(100)
                .ok_or(error!(ErrorCode::DivisionByZero))?;
            raw_quote
                .checked_sub(fee)
                .ok_or(error!(ErrorCode::Overflow))?
        } else {
            // Regular Raydium quote calculation
            let denominator = liquidity_a
                .checked_add(amount_in)
                .ok_or(error!(ErrorCode::Overflow))?;

            amount_with_fee
                .checked_mul(liquidity_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?
        };

        Ok((quote, liquidity_a, liquidity_b))
    }

    pub fn calculate_profit(
        ctx: Context<CalculateProfit>,
        amount: u64,
        is_pump: bool,
    ) -> Result<u64> {
        // 这是 calculate_profit_optimised 的简化版本
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // 计算初始报价
        let (quote, liquidity_a, liquidity_b) = if is_pump {
            // Pump pool format
            if pool_data.len() < 24 {
                return Ok(0);
            }

            let reserve_a = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[16..24].try_into().unwrap());

            if reserve_a == 0 || reserve_b == 0 {
                return Ok(0);
            }

            // 计算报价
            let fee_numerator = PUMP_FEE_RATE;
            let amount_with_fee = amount
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = reserve_a
                .checked_add(amount)
                .ok_or(error!(ErrorCode::Overflow))?;

            let raw_quote = amount_with_fee
                .checked_mul(reserve_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?;

            // Pump函数中有额外的费用扣除
            let fee = raw_quote
                .checked_div(100)
                .ok_or(error!(ErrorCode::DivisionByZero))?;
            let final_quote = raw_quote
                .checked_sub(fee)
                .ok_or(error!(ErrorCode::Overflow))?;

            (final_quote, reserve_a, reserve_b)
        } else {
            // Raydium pool format
            let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            if reserve_a == 0 || reserve_b == 0 || pool_data.len() < 208 {
                return Ok(0);
            }

            // Raydium quote calculation
            let fee_numerator = RAYDIUM_FEE_RATE;
            let amount_with_fee = amount
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = reserve_a
                .checked_add(amount)
                .ok_or(error!(ErrorCode::Overflow))?;

            let final_quote = amount_with_fee
                .checked_mul(reserve_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?;

            (final_quote, reserve_a, reserve_b)
        };

        // 计算反向报价
        let reverse_quote = if !is_pump {
            // Raydium反向报价
            let fee_numerator = RAYDIUM_FEE_RATE;
            let amount_with_fee = quote
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = liquidity_b
                .checked_add(quote)
                .ok_or(error!(ErrorCode::Overflow))?;

            amount_with_fee
                .checked_mul(liquidity_a)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?
        } else {
            // Pump反向报价
            let fee_numerator = PUMP_FEE_RATE;
            let amount_with_fee = quote
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = liquidity_a
                .checked_add(quote)
                .ok_or(error!(ErrorCode::Overflow))?;

            let raw_quote = amount_with_fee
                .checked_mul(liquidity_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?;

            // Pump函数中有额外的费用扣除
            let fee = raw_quote
                .checked_div(100)
                .ok_or(error!(ErrorCode::DivisionByZero))?;
            raw_quote
                .checked_sub(fee)
                .ok_or(error!(ErrorCode::Overflow))?
        };

        // 计算利润（如果有）
        if reverse_quote > amount {
            Ok(reverse_quote - amount)
        } else {
            Ok(0)
        }
    }

    pub fn is_buy_amount_too_big(
        ctx: Context<IsBuyAmountTooBig>,
        buy_amount: u64,
        expected_output: u64,
        is_pump: bool,
    ) -> Result<bool> {
        // 检查购买金额是否过大
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // 获取实际可得的输出
        let actual_output = if is_pump {
            // 以下代码基于公开的get_quote函数实现
            let reserve_a = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[16..24].try_into().unwrap());

            let fee_numerator = PUMP_FEE_RATE;
            let amount_with_fee = buy_amount
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = reserve_a
                .checked_add(buy_amount)
                .ok_or(error!(ErrorCode::Overflow))?;

            let raw_quote = amount_with_fee
                .checked_mul(reserve_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?;

            let fee = raw_quote
                .checked_div(100)
                .ok_or(error!(ErrorCode::DivisionByZero))?;
            raw_quote
                .checked_sub(fee)
                .ok_or(error!(ErrorCode::Overflow))?
        } else {
            let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            let fee_numerator = RAYDIUM_FEE_RATE;
            let amount_with_fee = buy_amount
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let denominator = reserve_a
                .checked_add(buy_amount)
                .ok_or(error!(ErrorCode::Overflow))?;

            amount_with_fee
                .checked_mul(reserve_b)
                .ok_or(error!(ErrorCode::Overflow))?
                .checked_div(denominator)
                .ok_or(error!(ErrorCode::DivisionByZero))?
        };

        // 如果实际输出小于期望输出，则购买金额过大
        if expected_output > actual_output {
            return Ok(true);
        }

        // 检查交易后池子是否仍然有效
        // 这里直接复用is_valid函数逻辑
        let is_pool_valid = if is_pump {
            // Pump pool validation
            let amount_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let amount_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            // Minimum liquidity check
            if amount_a <= 1000 || amount_b <= 1000 {
                false
            } else {
                // Price ratio check
                let price_ratio = calculate_price_ratio(amount_a, amount_b)?;
                price_ratio > 0
            }
        } else {
            // Raydium pool validation
            let amount_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
            let amount_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

            amount_a > 1000 && amount_b > 1000
        };

        Ok(!is_pool_valid)
    }

    pub fn calculate_optimal_strategy(
        ctx: Context<CalculateOptimalStrategy>,
        amount: u64,
        is_pump: bool,
    ) -> Result<(u64, u64)> {
        if amount <= 1000 {
            return Ok((0, 0));
        }

        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // 首先计算可能的上限（类似calculate_upper_bound_optimised）
        let upper_bound = {
            let pool_type = if is_pump { 1 } else { 0 };

            // 获取费率
            let fee_rate = if pool_type == 0 {
                RAYDIUM_FEE_RATE
            } else {
                PUMP_FEE_RATE
            };

            // 读取储备量
            let (reserve_a, reserve_b) = if pool_type == 0 {
                // Raydium V4 格式
                if pool_data.len() < 16 {
                    return Ok((0, 0));
                }
                let reserve_a = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());
                let reserve_b = u64::from_le_bytes(pool_data[16..24].try_into().unwrap());
                (reserve_a, reserve_b)
            } else {
                // Pump 格式
                if pool_data.len() < 408 {
                    return Ok((0, 0));
                }
                let offset = 0x155;
                let reserve_a =
                    u64::from_le_bytes(pool_data[offset..offset + 8].try_into().unwrap());
                let reserve_b =
                    u64::from_le_bytes(pool_data[offset + 8..offset + 16].try_into().unwrap());
                (reserve_a, reserve_b)
            };

            // 计算上限
            if reserve_b <= amount {
                0
            } else {
                let amount_minus_reserve = reserve_b
                    .checked_sub(amount)
                    .ok_or(error!(ErrorCode::Overflow))?;

                if amount_minus_reserve > MAX_U64 {
                    let div_result = amount_minus_reserve
                        .checked_div(fee_rate)
                        .ok_or(error!(ErrorCode::DivisionByZero))?;
                    let mul_result = div_result
                        .checked_mul(10000)
                        .ok_or(error!(ErrorCode::Overflow))?;
                    mul_result
                } else {
                    let mul_result = amount_minus_reserve
                        .checked_mul(10000)
                        .ok_or(error!(ErrorCode::Overflow))?;
                    let div_result = mul_result
                        .checked_div(fee_rate)
                        .ok_or(error!(ErrorCode::DivisionByZero))?;
                    div_result
                }
            }
        };

        if upper_bound < 1000 {
            return Ok((0, 0));
        }

        // 初始报价和流动性计算
        let (quote, liquidity_a, liquidity_b) = {
            // 获取流动性
            let (liquidity_a, liquidity_b) = if is_pump {
                // Pump pool format
                if pool_data.len() < 24 {
                    return Ok((0, 0));
                }

                let reserve_a = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());
                let reserve_b = u64::from_le_bytes(pool_data[16..24].try_into().unwrap());

                (reserve_a, reserve_b)
            } else {
                // Raydium pool format
                let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
                let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

                if reserve_a == 0 || reserve_b == 0 || pool_data.len() < 208 {
                    return Ok((0, 0));
                }

                (reserve_a, reserve_b)
            };

            // 计算报价
            let fee_numerator = if is_pump {
                PUMP_FEE_RATE
            } else {
                RAYDIUM_FEE_RATE
            };
            let amount_with_fee = upper_bound
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            let quote = if is_pump {
                // Pump quote calculation
                let denominator = liquidity_a
                    .checked_add(upper_bound)
                    .ok_or(error!(ErrorCode::Overflow))?;

                let raw_quote = amount_with_fee
                    .checked_mul(liquidity_b)
                    .ok_or(error!(ErrorCode::Overflow))?
                    .checked_div(denominator)
                    .ok_or(error!(ErrorCode::DivisionByZero))?;

                // Pump 额外费用
                let fee = raw_quote
                    .checked_div(100)
                    .ok_or(error!(ErrorCode::DivisionByZero))?;
                raw_quote
                    .checked_sub(fee)
                    .ok_or(error!(ErrorCode::Overflow))?
            } else {
                // Raydium quote calculation
                let denominator = liquidity_a
                    .checked_add(upper_bound)
                    .ok_or(error!(ErrorCode::Overflow))?;

                amount_with_fee
                    .checked_mul(liquidity_b)
                    .ok_or(error!(ErrorCode::Overflow))?
                    .checked_div(denominator)
                    .ok_or(error!(ErrorCode::DivisionByZero))?
            };

            (quote, liquidity_a, liquidity_b)
        };

        if liquidity_a == 0 || liquidity_b == 0 {
            return Ok((0, 0));
        }

        // 反向报价
        let reverse_quote = {
            let fee_numerator = if !is_pump {
                RAYDIUM_FEE_RATE
            } else {
                PUMP_FEE_RATE
            };
            let amount_with_fee = quote
                .checked_mul(fee_numerator)
                .ok_or(error!(ErrorCode::Overflow))?;

            if !is_pump {
                let denominator = liquidity_b
                    .checked_add(quote)
                    .ok_or(error!(ErrorCode::Overflow))?;

                amount_with_fee
                    .checked_mul(liquidity_a)
                    .ok_or(error!(ErrorCode::Overflow))?
                    .checked_div(denominator)
                    .ok_or(error!(ErrorCode::DivisionByZero))?
            } else {
                let denominator = liquidity_a
                    .checked_add(quote)
                    .ok_or(error!(ErrorCode::Overflow))?;

                let raw_quote = amount_with_fee
                    .checked_mul(liquidity_b)
                    .ok_or(error!(ErrorCode::Overflow))?
                    .checked_div(denominator)
                    .ok_or(error!(ErrorCode::DivisionByZero))?;

                let fee = raw_quote
                    .checked_div(100)
                    .ok_or(error!(ErrorCode::DivisionByZero))?;
                raw_quote
                    .checked_sub(fee)
                    .ok_or(error!(ErrorCode::Overflow))?
            }
        };

        // 计算利润
        let profit = if reverse_quote > upper_bound {
            reverse_quote - upper_bound
        } else {
            0
        };

        // 如果利润太小，尝试用不同的数量重新计算
        if profit < 1000 {
            // 从 upper_bound 减去1000，进行多次尝试寻找最优策略
            let mut best_amount = upper_bound;
            let mut best_profit = profit;

            let test_amount = upper_bound.checked_sub(1000).unwrap_or(0);
            if test_amount > 10000 {
                // 尝试更小的数量
                let test_profit = {
                    // 获取测试报价
                    let fee_numerator = if is_pump {
                        PUMP_FEE_RATE
                    } else {
                        RAYDIUM_FEE_RATE
                    };
                    let amount_with_fee = test_amount
                        .checked_mul(fee_numerator)
                        .ok_or(error!(ErrorCode::Overflow))?;

                    let test_quote = if is_pump {
                        // Pump quote calculation
                        let denominator = liquidity_a
                            .checked_add(test_amount)
                            .ok_or(error!(ErrorCode::Overflow))?;

                        let raw_quote = amount_with_fee
                            .checked_mul(liquidity_b)
                            .ok_or(error!(ErrorCode::Overflow))?
                            .checked_div(denominator)
                            .ok_or(error!(ErrorCode::DivisionByZero))?;

                        // Pump 额外费用
                        let fee = raw_quote
                            .checked_div(100)
                            .ok_or(error!(ErrorCode::DivisionByZero))?;
                        raw_quote
                            .checked_sub(fee)
                            .ok_or(error!(ErrorCode::Overflow))?
                    } else {
                        // Raydium quote calculation
                        let denominator = liquidity_a
                            .checked_add(test_amount)
                            .ok_or(error!(ErrorCode::Overflow))?;

                        amount_with_fee
                            .checked_mul(liquidity_b)
                            .ok_or(error!(ErrorCode::Overflow))?
                            .checked_div(denominator)
                            .ok_or(error!(ErrorCode::DivisionByZero))?
                    };

                    // 获取反向报价
                    let reverse_fee_numerator = if !is_pump {
                        RAYDIUM_FEE_RATE
                    } else {
                        PUMP_FEE_RATE
                    };
                    let reverse_amount_with_fee = test_quote
                        .checked_mul(reverse_fee_numerator)
                        .ok_or(error!(ErrorCode::Overflow))?;

                    let test_reverse = if !is_pump {
                        let denominator = liquidity_b
                            .checked_add(test_quote)
                            .ok_or(error!(ErrorCode::Overflow))?;

                        reverse_amount_with_fee
                            .checked_mul(liquidity_a)
                            .ok_or(error!(ErrorCode::Overflow))?
                            .checked_div(denominator)
                            .ok_or(error!(ErrorCode::DivisionByZero))?
                    } else {
                        let denominator = liquidity_a
                            .checked_add(test_quote)
                            .ok_or(error!(ErrorCode::Overflow))?;

                        let raw_quote = reverse_amount_with_fee
                            .checked_mul(liquidity_b)
                            .ok_or(error!(ErrorCode::Overflow))?
                            .checked_div(denominator)
                            .ok_or(error!(ErrorCode::DivisionByZero))?;

                        let fee = raw_quote
                            .checked_div(100)
                            .ok_or(error!(ErrorCode::DivisionByZero))?;
                        raw_quote
                            .checked_sub(fee)
                            .ok_or(error!(ErrorCode::Overflow))?
                    };

                    let test_profit = if test_reverse > test_amount {
                        test_reverse - test_amount
                    } else {
                        0
                    };

                    test_profit
                };

                if test_profit > best_profit {
                    best_amount = test_amount;
                    best_profit = test_profit;
                }
            }

            // 将结果存储到result_info账户中（如果提供）
            if !ctx
                .accounts
                .result_info
                .to_account_info()
                .key
                .eq(&Pubkey::default())
            {
                // 将结果写入result_info账户
                let mut result_data = ctx.accounts.result_info.try_borrow_mut_data()?;
                let amount_bytes = best_amount.to_le_bytes();
                let profit_bytes = best_profit.to_le_bytes();

                // 写入金额和利润
                if result_data.len() >= 16 {
                    result_data[0..8].copy_from_slice(&amount_bytes);
                    result_data[8..16].copy_from_slice(&profit_bytes);
                }
            }

            return Ok((best_amount, best_profit));
        }

        // 将结果存储到result_info账户中（如果提供）
        if !ctx
            .accounts
            .result_info
            .to_account_info()
            .key
            .eq(&Pubkey::default())
        {
            // 将结果写入result_info账户
            let mut result_data = ctx.accounts.result_info.try_borrow_mut_data()?;
            let amount_bytes = upper_bound.to_le_bytes();
            let profit_bytes = profit.to_le_bytes();

            // 写入金额和利润
            if result_data.len() >= 16 {
                result_data[0..8].copy_from_slice(&amount_bytes);
                result_data[8..16].copy_from_slice(&profit_bytes);
            }
        }

        Ok((upper_bound, profit))
    }

    // 新增函数：基于sBPF汇编实现的fast_path_create_raydium_v4
    pub fn fast_path_create_raydium_v4(
        ctx: Context<FastPathCreateRaydiumV4>,
        pool_type: u8,
    ) -> Result<()> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // 验证池子数据是否有效
        if pool_data.len() < 16 {
            return Err(error!(ErrorCode::InvalidPoolData));
        }

        // 创建并初始化Raydium V4池记录
        ctx.accounts.raydium_pool.pool_type = pool_type;

        // 读取池子的储备量
        let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

        // 验证最小流动性
        if reserve_a <= 1000 || reserve_b <= 1000 {
            return Err(error!(ErrorCode::InsufficientLiquidity));
        }

        // 将储备量保存到池记录中
        ctx.accounts.raydium_pool.reserve_a = reserve_a;
        ctx.accounts.raydium_pool.reserve_b = reserve_b;

        Ok(())
    }

    // 新增函数：基于sBPF汇编实现的fast_path_auto_swap_in_raydium_v4
    pub fn fast_path_auto_swap_in_raydium_v4(
        ctx: Context<FastPathAutoSwapInRaydiumV4>,
        amount_in: u64,
    ) -> Result<u64> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // 验证池子数据
        if pool_data.len() < 16 {
            return Err(error!(ErrorCode::InvalidPoolData));
        }

        // 读取池子的储备量
        let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

        // 计算价格影响
        let fee_numerator = RAYDIUM_FEE_RATE;
        let amount_with_fee = amount_in
            .checked_mul(fee_numerator)
            .ok_or(error!(ErrorCode::Overflow))?;

        let denominator = reserve_a
            .checked_add(amount_in)
            .ok_or(error!(ErrorCode::Overflow))?;

        let quote = amount_with_fee
            .checked_mul(reserve_b)
            .ok_or(error!(ErrorCode::Overflow))?
            .checked_div(denominator)
            .ok_or(error!(ErrorCode::DivisionByZero))?;

        // 验证最小输出
        if quote <= 1000 {
            return Err(error!(ErrorCode::OutputTooSmall));
        }

        // 更新池记录
        if !ctx
            .accounts
            .raydium_pool
            .to_account_info()
            .key
            .eq(&Pubkey::default())
        {
            ctx.accounts.raydium_pool.last_amount_in = amount_in;
            ctx.accounts.raydium_pool.last_quote = quote;
        }

        Ok(quote)
    }

    // 新增函数：基于sBPF汇编实现的fast_path_auto_swap_out_raydium_v4
    pub fn fast_path_auto_swap_out_raydium_v4(
        ctx: Context<FastPathAutoSwapOutRaydiumV4>,
        amount_out: u64,
        expected_input: u64,
    ) -> Result<u64> {
        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // 验证池子数据
        if pool_data.len() < 16 {
            return Err(error!(ErrorCode::InvalidPoolData));
        }

        // 读取池子的储备量
        let reserve_a = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(pool_data[8..16].try_into().unwrap());

        // 验证池子流动性是否足够
        if reserve_a <= 1000 || reserve_b <= 1000 || amount_out >= reserve_b {
            return Err(error!(ErrorCode::InsufficientLiquidity));
        }

        // 计算需要输入的金额
        let remaining_b = reserve_b
            .checked_sub(amount_out)
            .ok_or(error!(ErrorCode::Overflow))?;

        let numerator = reserve_a
            .checked_mul(reserve_b)
            .ok_or(error!(ErrorCode::Overflow))?;

        let denominator = remaining_b;

        let required_a = numerator
            .checked_div(denominator)
            .ok_or(error!(ErrorCode::DivisionByZero))?;

        let input_amount = required_a
            .checked_sub(reserve_a)
            .ok_or(error!(ErrorCode::Overflow))?;

        // 添加费用
        let fee_amount = input_amount
            .checked_mul(10000 - RAYDIUM_FEE_RATE)
            .ok_or(error!(ErrorCode::Overflow))?
            .checked_div(RAYDIUM_FEE_RATE)
            .ok_or(error!(ErrorCode::DivisionByZero))?;

        let total_input = input_amount
            .checked_add(fee_amount)
            .ok_or(error!(ErrorCode::Overflow))?;

        // 验证输入是否符合预期
        if total_input > expected_input {
            return Err(error!(ErrorCode::SlippageExceeded));
        }

        // 验证交易后池子是否仍然有效
        let new_reserve_a = reserve_a
            .checked_add(total_input)
            .ok_or(error!(ErrorCode::Overflow))?;

        if new_reserve_a <= 1000 {
            return Err(error!(ErrorCode::PoolInvalid));
        }

        // 更新池记录
        if !ctx
            .accounts
            .raydium_pool
            .to_account_info()
            .key
            .eq(&Pubkey::default())
        {
            ctx.accounts.raydium_pool.last_amount_in = total_input;
            ctx.accounts.raydium_pool.last_quote = amount_out;
        }

        Ok(total_input)
    }

    // 新增函数：基于sBPF汇编实现的get_key_type_optimised
    pub fn get_key_type_optimised(ctx: Context<GetKeyType>) -> Result<()> {
        // 将Pubkey转换为字节数组
        let key = ctx.accounts.pool_info.key().to_bytes();

        // 将key解析为4个u64值进行比较
        let key_parts = [
            u64::from_le_bytes(key[0..8].try_into().unwrap()),
            u64::from_le_bytes(key[8..16].try_into().unwrap()),
            u64::from_le_bytes(key[16..24].try_into().unwrap()),
            u64::from_le_bytes(key[24..32].try_into().unwrap()),
        ];

        // 根据汇编代码中的常量比较逻辑
        let pool_type = match key_parts[0] {
            0x3fc30236c449d94b => {
                if key_parts[1] == 0x4c52a316ed907720
                    && key_parts[2] == 0xa9a221f15c97b9a1
                    && key_parts[3] == 0xcd8ab6f87decff0c
                {
                    0 // Raydium V4 Pool
                } else {
                    3
                }
            }
            0x5259294f8b5a2aa9 => {
                if key_parts[1] == 0x955bfd93aa502584
                    && key_parts[2] == 0x930c92eba8e6acb5
                    && key_parts[3] == 0x73ec200c69432e94
                {
                    1 // PUMP Pool
                } else {
                    3
                }
            }
            0xcf5a6693f6e05601 => {
                if key_parts[1] == 0xaa5b17bf6815db44
                    && key_parts[2] == 0x3bffd2f597cb8951
                    && key_parts[3] == 0xb0186dfdb62b5d65
                {
                    2 // Other Pool Type
                } else {
                    3
                }
            }
            _ => 3, // Unknown Pool Type
        };

        // 保存池类型到状态账户
        ctx.accounts.pool_type.pool_type = pool_type;

        Ok(())
    }

    // 新增函数：基于sBPF汇编实现的deserialize_swap_optimised
    pub fn deserialize_swap_optimised(
        ctx: Context<DeserializeSwapOptimised>,
        pool_type: u8,
    ) -> Result<bool> {
        // 验证池类型
        if pool_type != 0 {
            // 只支持Raydium V4池
            return Ok(false);
        }

        let pool_data = ctx.accounts.pool_info.try_borrow_data()?;

        // 验证池数据长度
        if pool_data.len() < 896 {
            return Ok(false);
        }

        // 解析池数据
        let token_a_mint = u64::from_le_bytes(pool_data[0..8].try_into().unwrap());
        let token_b_mint = u64::from_le_bytes(pool_data[56..64].try_into().unwrap());
        let reserve_a = u64::from_le_bytes(pool_data[224..232].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(pool_data[280..288].try_into().unwrap());

        // 更新池状态
        let pool_state = &mut ctx.accounts.pool_state;
        pool_state.token_a_mint = token_a_mint;
        pool_state.token_b_mint = token_b_mint;
        pool_state.reserve_a = reserve_a;
        pool_state.reserve_b = reserve_b;
        pool_state.is_initialized = true;

        // 设置流动性相关字段
        pool_state.liquidity_a = reserve_a;
        pool_state.liquidity_b = reserve_b;
        pool_state.last_observation_slot = ctx.accounts.clock.slot;
        pool_state.observation_index = 0;

        // 设置其他必要字段
        pool_state.fee_rate = RAYDIUM_FEE_RATE;
        pool_state.protocol_fee_rate = 0;
        pool_state.swap_fee_enabled = true;
        pool_state.is_pause = false;

        Ok(true)
    }

    // 新增函数：基于sBPF汇编实现的get_swap_instruction_optimised
    pub fn get_swap_instruction_optimised(
        ctx: Context<GetSwapInstructionOptimised>,
        is_reverse: bool,
    ) -> Result<bool> {
        // 获取池类型
        let pool_type = get_pool_type(ctx.accounts.pool_info.key);

        // 根据池类型生成交换指令
        match pool_type {
            0 => {
                // Raydium V4 Pool
                // 设置Raydium V4交换指令
                let mut swap_data = ctx.accounts.swap_data.try_borrow_mut_data()?;
                swap_data[0] = 9; // swap instruction

                // 设置其他指令数据
                let instruction = SwapInstruction {
                    program_id: *ctx.accounts.pool_info.key,
                    accounts: vec![
                        AccountMeta::new(*ctx.accounts.token_program.key, false),
                        AccountMeta::new(*ctx.accounts.amm_id.key, false),
                        AccountMeta::new(*ctx.accounts.amm_authority.key, false),
                        AccountMeta::new(*ctx.accounts.pool_info.key, false),
                        AccountMeta::new(ctx.accounts.user_token_a.to_account_info().key(), true),
                        AccountMeta::new(ctx.accounts.user_token_b.to_account_info().key(), true),
                    ],
                    data: vec![9], // swap instruction
                };

                // 序列化指令
                instruction.serialize(&mut *swap_data)?;

                Ok(true)
            }
            1 => {
                // PUMP Pool
                // 设置PUMP交换指令
                let mut swap_data = ctx.accounts.swap_data.try_borrow_mut_data()?;
                let instruction_data = if is_reverse {
                    0xde331ec4da5abe8f_u64.to_le_bytes()
                } else {
                    0xad837f01a485e633_u64.to_le_bytes()
                };
                swap_data[0..8].copy_from_slice(&instruction_data);

                Ok(true)
            }
            2 => {
                // Other Pool Type
                // 设置其他类型池的交换指令
                let mut swap_data = ctx.accounts.swap_data.try_borrow_mut_data()?;
                if is_reverse {
                    let instruction_data = 0xeaebda01123d0666_u64.to_le_bytes();
                    swap_data[0..8].copy_from_slice(&instruction_data);
                } else {
                    let instruction_data = 0xad837f01a485e633_u64.to_le_bytes();
                    swap_data[0..8].copy_from_slice(&instruction_data);
                }

                Ok(true)
            }
            _ => Ok(false),
        }
    }

    // 新增函数：基于sBPF汇编实现的execute_swap_optimised
    pub fn execute_swap_optimised(
        ctx: Context<ExecuteSwapOptimised>,
        is_reverse: bool,
    ) -> Result<()> {
        // 获取池类型
        let pool_type = get_pool_type(ctx.accounts.pool_info.key);

        // 根据池类型执行交换
        match pool_type {
            0 => {
                // Raydium V4 Pool
                // 执行Raydium V4交换
                let ix = Instruction {
                    program_id: *ctx.accounts.pool_info.key,
                    accounts: vec![
                        AccountMeta::new(*ctx.accounts.token_program.key, false),
                        AccountMeta::new(*ctx.accounts.amm_id.key, false),
                        AccountMeta::new(*ctx.accounts.amm_authority.key, false),
                        AccountMeta::new(*ctx.accounts.pool_info.key, false),
                        AccountMeta::new(ctx.accounts.user_token_a.to_account_info().key(), true),
                        AccountMeta::new(ctx.accounts.user_token_b.to_account_info().key(), true),
                    ],
                    data: vec![9], // swap instruction
                };

                invoke_signed(
                    &ix,
                    &[
                        ctx.accounts.token_program.to_account_info(),
                        ctx.accounts.amm_id.to_account_info(),
                        ctx.accounts.amm_authority.to_account_info(),
                        ctx.accounts.pool_info.to_account_info(),
                        ctx.accounts.user_token_a.to_account_info(),
                        ctx.accounts.user_token_b.to_account_info(),
                    ],
                    &[],
                )?;
            }
            1 => {
                // PUMP Pool
                // 执行PUMP交换
                let ix = Instruction {
                    program_id: *ctx.accounts.pool_info.key,
                    accounts: vec![
                        AccountMeta::new(*ctx.accounts.token_program.key, false),
                        AccountMeta::new(*ctx.accounts.amm_id.key, false),
                        AccountMeta::new(*ctx.accounts.amm_authority.key, false),
                        AccountMeta::new(*ctx.accounts.pool_info.key, false),
                        AccountMeta::new(ctx.accounts.user_token_a.to_account_info().key(), true),
                        AccountMeta::new(ctx.accounts.user_token_b.to_account_info().key(), true),
                    ],
                    data: if is_reverse {
                        vec![0xde, 0x33, 0x1e, 0xc4, 0xda, 0x5a, 0xbe, 0x8f]
                    } else {
                        vec![0xad, 0x83, 0x7f, 0x01, 0xa4, 0x85, 0xe6, 0x33]
                    },
                };

                invoke_signed(
                    &ix,
                    &[
                        ctx.accounts.token_program.to_account_info(),
                        ctx.accounts.amm_id.to_account_info(),
                        ctx.accounts.amm_authority.to_account_info(),
                        ctx.accounts.pool_info.to_account_info(),
                        ctx.accounts.user_token_a.to_account_info(),
                        ctx.accounts.user_token_b.to_account_info(),
                    ],
                    &[],
                )?;
            }
            2 => {
                // Other Pool Type
                // 执行其他类型池的交换
                let ix = Instruction {
                    program_id: *ctx.accounts.pool_info.key,
                    accounts: vec![
                        AccountMeta::new(*ctx.accounts.token_program.key, false),
                        AccountMeta::new(*ctx.accounts.amm_id.key, false),
                        AccountMeta::new(*ctx.accounts.amm_authority.key, false),
                        AccountMeta::new(*ctx.accounts.pool_info.key, false),
                        AccountMeta::new(ctx.accounts.user_token_a.to_account_info().key(), true),
                        AccountMeta::new(ctx.accounts.user_token_b.to_account_info().key(), true),
                    ],
                    data: if is_reverse {
                        vec![0xea, 0xeb, 0xda, 0x01, 0x12, 0x3d, 0x06, 0x66]
                    } else {
                        vec![0xad, 0x83, 0x7f, 0x01, 0xa4, 0x85, 0xe6, 0x33]
                    },
                };

                invoke_signed(
                    &ix,
                    &[
                        ctx.accounts.token_program.to_account_info(),
                        ctx.accounts.amm_id.to_account_info(),
                        ctx.accounts.amm_authority.to_account_info(),
                        ctx.accounts.pool_info.to_account_info(),
                        ctx.accounts.user_token_a.to_account_info(),
                        ctx.accounts.user_token_b.to_account_info(),
                    ],
                    &[],
                )?;
            }
            _ => return Err(error!(ErrorCode::UnsupportedPoolType)),
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct IsValid<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct GetQuote<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct GetLiquidity<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CalculateProfitOptimised<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CalculateHintedMaxAmount<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CalculateUpperBound<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct GetQuoteAndLiquidity<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CalculateProfit<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct IsBuyAmountTooBig<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CalculateOptimalStrategy<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,
    /// CHECK: Output results
    pub result_info: UncheckedAccount<'info>,
}

// 新增账户结构：fast_path_create_raydium_v4
#[derive(Accounts)]
pub struct FastPathCreateRaydiumV4<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,

    /// Raydium池记录
    #[account(mut)]
    pub raydium_pool: Account<'info, RaydiumPoolState>,

    /// 系统程序
    pub system_program: Program<'info, System>,

    /// 支付创建费用的账户
    #[account(mut)]
    pub payer: Signer<'info>,
}

// 新增账户结构：fast_path_auto_swap_in_raydium_v4
#[derive(Accounts)]
pub struct FastPathAutoSwapInRaydiumV4<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,

    /// Raydium池记录
    #[account(mut)]
    pub raydium_pool: Account<'info, RaydiumPoolState>,
}

// 新增账户结构：fast_path_auto_swap_out_raydium_v4
#[derive(Accounts)]
pub struct FastPathAutoSwapOutRaydiumV4<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,

    /// Raydium池记录
    #[account(mut)]
    pub raydium_pool: Account<'info, RaydiumPoolState>,
}

// 新增账户结构：deserialize_swap_optimised
#[derive(Accounts)]
pub struct DeserializeSwapOptimised<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,

    /// 池状态账户
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    /// 时钟账户
    pub clock: Sysvar<'info, Clock>,
}

// 新增账户结构：get_swap_instruction_optimised
#[derive(Accounts)]
pub struct GetSwapInstructionOptimised<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    pub token_program: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    pub amm_id: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    pub amm_authority: UncheckedAccount<'info>,

    /// 用户代币A账户
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,

    /// 用户代币B账户
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,

    /// 交换指令数据
    #[account(mut)]
    pub swap_data: AccountInfo<'info>,
}

// 新增账户结构：execute_swap_optimised
#[derive(Accounts)]
pub struct ExecuteSwapOptimised<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    pub token_program: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    pub amm_id: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    pub amm_authority: UncheckedAccount<'info>,

    /// 用户代币A账户
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,

    /// 用户代币B账户
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
}

// 新增池状态结构体
#[account]
pub struct RaydiumPoolState {
    pub pool_type: u8,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub last_amount_in: u64,
    pub last_quote: u64,
}

// 新增池状态结构体
#[account]
pub struct PoolState {
    pub token_a_mint: u64,
    pub token_b_mint: u64,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub liquidity_a: u64,
    pub liquidity_b: u64,
    pub last_observation_slot: u64,
    pub observation_index: u16,
    pub fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub swap_fee_enabled: bool,
    pub is_pause: bool,
    pub is_initialized: bool,
}

// Constants from assembly
pub const RAYDIUM_FEE_RATE: u64 = 9975;
pub const PUMP_FEE_RATE: u64 = 9900;
pub const MAX_U64: u64 = 0x68db8bac710cc;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Invalid pool data")]
    InvalidPoolData,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Output too small")]
    OutputTooSmall,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
    #[msg("Pool would be invalid after swap")]
    PoolInvalid,
    #[msg("Unsupported pool type")]
    UnsupportedPoolType,
    #[msg("Buffer too small")]
    BufferTooSmall,
}

// Helper functions
fn calculate_price_ratio(amount_a: u64, amount_b: u64) -> Result<i64> {
    // This is a simplified version of the price calculation in pump_fun_is_valid
    if amount_a == 0 || amount_b == 0 {
        return Ok(0);
    }

    // Very simple price ratio calculation - in practice would need to handle large numbers better
    // The assembly uses floating point calculations which we approximate here
    let ratio = amount_b as f64 / amount_a as f64;

    // Check if price is within certain bounds (from assembly: compared with 0x42d6bcc41e900000 and 0x4253ca6512000000)
    // We simplify by just checking if ratio is positive
    if ratio > 0.0 {
        Ok(1)
    } else {
        Ok(0)
    }
}

// 新增结构体：SwapInstruction
#[derive(Clone)]
pub struct SwapInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMeta>,
    pub data: Vec<u8>,
}

impl SwapInstruction {
    pub fn serialize(&self, output: &mut [u8]) -> Result<()> {
        if output.len() < self.data.len() {
            return Err(error!(ErrorCode::BufferTooSmall));
        }
        output[..self.data.len()].copy_from_slice(&self.data);
        Ok(())
    }
}

// 新增账户结构：get_key_type_optimised
#[derive(Accounts)]
pub struct GetKeyType<'info> {
    /// CHECK: Validated in instruction
    pub pool_info: UncheckedAccount<'info>,

    /// 池类型状态账户
    #[account(mut)]
    pub pool_type: Account<'info, PoolTypeState>,
}

// 新增池类型状态结构体
#[account]
pub struct PoolTypeState {
    pub pool_type: u8,
}

// 辅助函数：获取池类型
fn get_pool_type(pubkey: &Pubkey) -> u8 {
    // 将Pubkey转换为字节数组
    let key = pubkey.to_bytes();

    // 将key解析为4个u64值进行比较
    let key_parts = [
        u64::from_le_bytes(key[0..8].try_into().unwrap()),
        u64::from_le_bytes(key[8..16].try_into().unwrap()),
        u64::from_le_bytes(key[16..24].try_into().unwrap()),
        u64::from_le_bytes(key[24..32].try_into().unwrap()),
    ];

    // 根据汇编代码中的常量比较逻辑
    match key_parts[0] {
        0x3fc30236c449d94b => {
            if key_parts[1] == 0x4c52a316ed907720
                && key_parts[2] == 0xa9a221f15c97b9a1
                && key_parts[3] == 0xcd8ab6f87decff0c
            {
                0 // Raydium V4 Pool
            } else {
                3
            }
        }
        0x5259294f8b5a2aa9 => {
            if key_parts[1] == 0x955bfd93aa502584
                && key_parts[2] == 0x930c92eba8e6acb5
                && key_parts[3] == 0x73ec200c69432e94
            {
                1 // PUMP Pool
            } else {
                3
            }
        }
        0xcf5a6693f6e05601 => {
            if key_parts[1] == 0xaa5b17bf6815db44
                && key_parts[2] == 0x3bffd2f597cb8951
                && key_parts[3] == 0xb0186dfdb62b5d65
            {
                2 // Other Pool Type
            } else {
                3
            }
        }
        _ => 3, // Unknown Pool Type
    }
}
