use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sbpf_to_anchor {
    use super::*;

    // Entry point to check if a DEX is valid
    pub fn is_valid(ctx: Context<IsValid>, data: Vec<u8>) -> Result<bool> {
        // 根据sBPF分析，这个函数首先检查第一个参数是否为0
        let dex_type = if data.len() >= 4 {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&data[0..4]);
            u32::from_le_bytes(bytes)
        } else {
            return Err(error!(ErrorCode::InvalidInput));
        };

        // 调用不同的实现
        if dex_type == 0 {
            // 跳过8字节，调用raydium的实现
            if data.len() < 8 {
                return Err(error!(ErrorCode::InvalidInput));
            }
            raydium_is_valid(&data[8..])
        } else {
            // 其他类型的DEX暂未实现
            Ok(false)
        }
    }

    // 获取报价信息
    pub fn get_quote(ctx: Context<GetQuote>, data: Vec<u8>) -> Result<u64> {
        // 解析DEX类型
        let dex_type = if data.len() >= 4 {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&data[0..4]);
            u32::from_le_bytes(bytes)
        } else {
            return Err(error!(ErrorCode::InvalidInput));
        };

        // 调用相应的实现
        match dex_type {
            0 => {
                // Raydium
                if data.len() < 8 {
                    return Err(error!(ErrorCode::InvalidInput));
                }
                raydium_get_quote(&data[8..], ctx.accounts.side.side)
            }
            1 => {
                // PumpFun
                if data.len() < 8 {
                    return Err(error!(ErrorCode::InvalidInput));
                }
                pump_fun_get_quote(&data[8..])
            }
            _ => Err(error!(ErrorCode::UnsupportedDex)),
        }
    }

    // 获取流动性信息
    pub fn get_liquidity(ctx: Context<GetLiquidity>, data: Vec<u8>) -> Result<()> {
        // 解析DEX类型
        let dex_type = if data.len() >= 4 {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&data[0..4]);
            u32::from_le_bytes(bytes)
        } else {
            return Err(error!(ErrorCode::InvalidInput));
        };

        // 写入DEX类型到输出
        ctx.accounts.output.dex_type = dex_type;

        // 调用相应实现
        match dex_type {
            0 => {
                // Raydium
                if data.len() < 8 {
                    return Err(error!(ErrorCode::InvalidInput));
                }
                raydium_get_liquidity(&data[8..], &mut ctx.accounts.output.liquidity)
            }
            1 => {
                // PumpFun
                if data.len() < 8 {
                    return Err(error!(ErrorCode::InvalidInput));
                }
                pump_fun_get_liquidity(&data[8..], &mut ctx.accounts.output.liquidity)
            }
            _ => Err(error!(ErrorCode::UnsupportedDex)),
        }
    }

    // 同时获取报价和流动性
    pub fn get_quote_and_liquidity(
        ctx: Context<GetQuoteAndLiquidity>,
        data: Vec<u8>,
    ) -> Result<u64> {
        // 解析DEX类型
        let dex_type = if data.len() >= 4 {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&data[0..4]);
            u32::from_le_bytes(bytes)
        } else {
            return Err(error!(ErrorCode::InvalidInput));
        };

        // 写入DEX类型到输出
        ctx.accounts.output.dex_type = dex_type;

        // 调用相应实现
        match dex_type {
            0 => {
                // Raydium
                if data.len() < 8 {
                    return Err(error!(ErrorCode::InvalidInput));
                }
                raydium_get_quote_and_liquidity(
                    &data[8..],
                    &mut ctx.accounts.output.liquidity,
                    ctx.accounts.side.side,
                )
            }
            1 => {
                // PumpFun
                if data.len() < 8 {
                    return Err(error!(ErrorCode::InvalidInput));
                }
                pump_fun_get_quote_and_liquidity(
                    &data[8..],
                    &mut ctx.accounts.output.liquidity,
                    ctx.accounts.side.side,
                )
            }
            _ => Err(error!(ErrorCode::UnsupportedDex)),
        }
    }

    // 计算优化的利润
    pub fn calculate_profit_optimised(
        ctx: Context<CalculateProfit>,
        input_amount: u64,
        side: bool,
        dex_info: DexInfo,
    ) -> Result<i64> {
        // 创建输出
        let mut output = LiquidityOutput::default();

        // 获取报价和流动性
        let quote = get_quote_and_liquidity_internal(&dex_info, &mut output.liquidity, side)?;

        // 获取另一个方向的流动性
        get_liquidity_internal(&dex_info, &mut output.liquidity, !side)?;

        // 获取另一个方向的报价
        let reverse_quote = get_quote_internal(&output.liquidity, quote, !side)?;

        // 计算利润
        Ok((reverse_quote as i64) - (input_amount as i64))
    }

    // 计算最大交易量
    pub fn calculate_hinted_max_amount_optimised(
        ctx: Context<CalculateMaxAmount>,
        reserve_a: u64,
        reserve_b: u64,
        fee_numerator: u64,
        fee_multiplier: u64,
    ) -> Result<u64> {
        // 如果 reserve_b 大于 reserve_a，直接返回 0
        if reserve_b > reserve_a {
            return Ok(0);
        }

        // 计算可用余额
        let available = reserve_a - reserve_b;

        let max_constant = 0x68db8bac710cc; // 从sBPF代码中提取的常量

        let fee_denominator = 10000;
        let fee_factor = fee_denominator - fee_numerator;

        let mut result: u64;

        // 根据sBPF代码中的逻辑分支
        if available > max_constant {
            // 先除后乘以fee_factor
            result = available / fee_factor;
            result = result * fee_denominator;
        } else {
            // 先乘后除以fee_factor
            result = available * fee_denominator;
            result = result / fee_factor;
        }

        // 处理结果与fee_multiplier的乘除操作
        if result > max_constant {
            // 先除后乘以fee_multiplier
            result = result / fee_denominator;
            result = result * fee_multiplier;
        } else {
            // 先乘后除以fee_multiplier
            result = result * fee_multiplier;
            result = result / fee_denominator;
        }

        Ok(result)
    }

    // 计算上限
    pub fn calculate_upper_bound_optimised(
        ctx: Context<CalculateUpperBound>,
        reserve_a: u64,
        reserve_info: ReserveInfo,
        fee_multiplier: u64,
        is_token_a: bool,
    ) -> Result<u64> {
        // 初始化结果为0
        let mut result: u64 = 0;

        // 确定手续费
        let fee_numerator = if reserve_info.dex_type == 0 {
            9975
        } else {
            9900
        };

        // 获取相应的reserve值
        let reserve_value = if is_token_a {
            reserve_info.reserve_a
        } else {
            reserve_info.reserve_b
        };

        // 如果reserve_value大于reserve_a，进行计算
        if reserve_value > reserve_a {
            // 计算可用
            let available = reserve_value - reserve_a;

            let max_constant = 0x68db8bac710cc; // 从sBPF代码中提取的常量

            // 根据available的大小选择计算方式
            if available > max_constant {
                // 先除后乘
                result = available / fee_numerator;
                result = result * 10000;
            } else {
                // 先乘后除
                result = available * 10000;
                result = result / fee_numerator;
            }

            // 处理结果与fee_multiplier的乘除操作
            if result > max_constant {
                // 先除后乘
                result = result / 10000;
                result = result * fee_multiplier;
            } else {
                // 先乘后除
                result = result * fee_multiplier;
                result = result / 10000;
            }
        }

        Ok(result)
    }

    // 计算最优策略
    pub fn calculate_optimal_strategy_optimised(
        ctx: Context<CalculateStrategy>,
        input_amount: u64,
        dex_info: DexInfo,
        side: bool,
    ) -> Result<OptimalStrategyResult> {
        // 常量从sBPF代码提取
        let max_constant = 0x68db8bac710cc;
        let fee_numerator = if dex_info.dex_type == 0 { 9975 } else { 9900 };

        // 初始化结果
        let mut result = OptimalStrategyResult {
            is_profitable: false,
            amount_in: 0,
            expected_profit: 0,
        };

        // 从dex_info提取reserves
        let (reserve_a, reserve_b) = extract_reserves_from_dex(&dex_info, side)?;

        // 验证reserves
        if reserve_b > input_amount {
            // 计算可用量
            let available = input_amount - reserve_b;

            // 使用相似于sBPF的计算逻辑
            let mut amount_in: u64;
            if available > max_constant {
                amount_in = available / fee_numerator;
                amount_in = amount_in * 10000;
            } else {
                amount_in = available * 10000;
                amount_in = amount_in / fee_numerator;
            }

            // 计算预期收益
            let mut expected_profit =
                calculate_expected_profit(amount_in, reserve_a, reserve_b, dex_info.dex_type)?;

            // 验证策略是否可行
            if amount_in > 999 && expected_profit > 0 {
                result.is_profitable = true;
                result.amount_in = amount_in;
                result.expected_profit = expected_profit;
            } else {
                // 输出调试信息
                msg!(
                    "Strategy not profitable: amount_in={}, profit={}",
                    amount_in,
                    expected_profit
                );
            }
        }

        Ok(result)
    }

    // 其他函数实现...
}

// 账户结构定义
#[derive(Accounts)]
pub struct IsValid {
    // 根据需要添加必要的账户
}

#[derive(Accounts)]
pub struct GetQuote<'info> {
    // 包含Side结构的账户
    pub side: Account<'info, Side>,
}

#[derive(Accounts)]
pub struct GetLiquidity<'info> {
    // 输出账户
    #[account(mut)]
    pub output: Account<'info, LiquidityOutput>,
}

#[derive(Accounts)]
pub struct GetQuoteAndLiquidity<'info> {
    // 输出账户
    #[account(mut)]
    pub output: Account<'info, LiquidityOutput>,

    // 包含Side的账户
    pub side: Account<'info, Side>,
}

#[derive(Accounts)]
pub struct CalculateProfit {
    // 可能需要的账户
}

#[derive(Accounts)]
pub struct CalculateMaxAmount {
    // 相关账户结构
}

#[derive(Accounts)]
pub struct CalculateUpperBound {
    // 相关账户结构
}

#[derive(Accounts)]
pub struct CalculateStrategy {
    // 相关账户结构
}

// 定义数据结构
#[account]
#[derive(Default)]
pub struct Side {
    pub side: bool, // false = buy (0), true = sell (1)
}

#[account]
#[derive(Default)]
pub struct LiquidityOutput {
    pub dex_type: u32,
    pub liquidity: Liquidity,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug)]
pub struct Liquidity {
    pub reserve_a: u64,
    pub reserve_b: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct DexInfo {
    pub dex_type: u32,        // 0 = Raydium, 1 = PumpFun
    pub pool_data: [u8; 100], // 池数据，根据实际情况调整大小
}

impl Default for DexInfo {
    fn default() -> Self {
        Self {
            dex_type: 0,
            pool_data: [0; 100],
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug)]
pub struct ReserveInfo {
    pub dex_type: u32, // 0 = Raydium, 1 = PumpFun
    pub reserve_a: u64,
    pub reserve_b: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug)]
pub struct OptimalStrategyResult {
    pub is_profitable: bool,
    pub amount_in: u64,
    pub expected_profit: u64,
}

// 错误码定义
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid input data")]
    InvalidInput,
    #[msg("Unsupported DEX type")]
    UnsupportedDex,
    #[msg("Calculation failed")]
    CalculationFailed,
}

// 实现各个DEX的功能
fn raydium_is_valid(data: &[u8]) -> Result<bool> {
    // 根据sBPF代码实现逻辑
    // 这里简化实现
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取两个u64
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    let reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&data[8..16]);
    let reserve_b = u64::from_le_bytes(bytes);

    // 检查reserves是否大于1000
    Ok(reserve_a > 1000 && reserve_b > 1000)
}

fn raydium_get_quote(data: &[u8], side: bool) -> Result<u64> {
    // 简化实现
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取reserves
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    let reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&data[8..16]);
    let reserve_b = u64::from_le_bytes(bytes);

    // 根据side计算报价
    // 这里简化计算，实际应根据sBPF代码的精确算法
    let fee_numerator = 25; // 0.25% fee = 25/10000
    let fee_denominator = 10000;

    if side {
        // Sell token A for B
        let amount_out = (reserve_b * 1000) / (reserve_a + 1000)
            * (fee_denominator - fee_numerator)
            / fee_denominator;
        Ok(amount_out)
    } else {
        // Buy token A with B
        let amount_out = (reserve_a * 1000) / (reserve_b + 1000)
            * (fee_denominator - fee_numerator)
            / fee_denominator;
        Ok(amount_out)
    }
}

fn raydium_get_liquidity(data: &[u8], output: &mut Liquidity) -> Result<()> {
    // 简化实现
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取reserves
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    output.reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&data[8..16]);
    output.reserve_b = u64::from_le_bytes(bytes);

    Ok(())
}

fn raydium_get_quote_and_liquidity(data: &[u8], output: &mut Liquidity, side: bool) -> Result<u64> {
    // 先获取流动性
    raydium_get_liquidity(data, output)?;

    // 然后计算报价
    let reserve_a = output.reserve_a;
    let reserve_b = output.reserve_b;

    // 根据side计算报价
    let fee_numerator = 25; // 0.25% fee
    let fee_denominator = 10000;

    if side {
        // Sell token A for B
        let amount_out = (reserve_b * 1000) / (reserve_a + 1000)
            * (fee_denominator - fee_numerator)
            / fee_denominator;
        Ok(amount_out)
    } else {
        // Buy token A with B
        let amount_out = (reserve_a * 1000) / (reserve_b + 1000)
            * (fee_denominator - fee_numerator)
            / fee_denominator;
        Ok(amount_out)
    }
}

// PumpFun实现
fn pump_fun_get_quote(data: &[u8]) -> Result<u64> {
    // 简化实现
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取reserves
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    let reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&data[8..16]);
    let reserve_b = u64::from_le_bytes(bytes);

    // 使用不同的fee
    let fee_numerator = 100; // 1% fee
    let fee_denominator = 10000;

    // 简化计算
    let amount_out = (reserve_b * 1000) / (reserve_a + 1000) * (fee_denominator - fee_numerator)
        / fee_denominator;
    Ok(amount_out)
}

fn pump_fun_get_liquidity(data: &[u8], output: &mut Liquidity) -> Result<()> {
    // 简化实现，与Raydium类似但字段位置可能不同
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取reserves
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    output.reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&data[8..16]);
    output.reserve_b = u64::from_le_bytes(bytes);

    Ok(())
}

fn pump_fun_get_quote_and_liquidity(
    data: &[u8],
    output: &mut Liquidity,
    side: bool,
) -> Result<u64> {
    // 先获取流动性
    pump_fun_get_liquidity(data, output)?;

    // 然后计算报价
    let reserve_a = output.reserve_a;
    let reserve_b = output.reserve_b;

    // 根据side计算报价，使用不同的fee
    let fee_numerator = 100; // 1% fee
    let fee_denominator = 10000;

    if side {
        // Sell token A for B
        let amount_out = (reserve_b * 1000) / (reserve_a + 1000)
            * (fee_denominator - fee_numerator)
            / fee_denominator;
        Ok(amount_out)
    } else {
        // Buy token A with B
        let amount_out = (reserve_a * 1000) / (reserve_b + 1000)
            * (fee_denominator - fee_numerator)
            / fee_denominator;
        Ok(amount_out)
    }
}

// 内部函数，用于组合调用
fn get_quote_and_liquidity_internal(
    dex_info: &DexInfo,
    output: &mut Liquidity,
    side: bool,
) -> Result<u64> {
    match dex_info.dex_type {
        0 => raydium_get_quote_and_liquidity(&dex_info.pool_data, output, side),
        1 => pump_fun_get_quote_and_liquidity(&dex_info.pool_data, output, side),
        _ => Err(error!(ErrorCode::UnsupportedDex)),
    }
}

fn get_liquidity_internal(dex_info: &DexInfo, output: &mut Liquidity, side: bool) -> Result<()> {
    match dex_info.dex_type {
        0 => raydium_get_liquidity(&dex_info.pool_data, output),
        1 => pump_fun_get_liquidity(&dex_info.pool_data, output),
        _ => Err(error!(ErrorCode::UnsupportedDex)),
    }
}

fn get_quote_internal(liquidity: &Liquidity, amount: u64, side: bool) -> Result<u64> {
    // 简化实现，根据流动性和金额计算报价
    // 在实际情况下，应该使用完整的计算逻辑

    let reserve_a = liquidity.reserve_a;
    let reserve_b = liquidity.reserve_b;

    // 根据side计算报价
    let fee_numerator = if side { 100 } else { 25 }; // 根据DEX类型选择fee
    let fee_denominator = 10000;

    if side {
        // Sell token A for B
        let amount_out = (reserve_b * amount) / (reserve_a + amount)
            * (fee_denominator - fee_numerator)
            / fee_denominator;
        Ok(amount_out)
    } else {
        // Buy token A with B
        let amount_out = (reserve_a * amount) / (reserve_b + amount)
            * (fee_denominator - fee_numerator)
            / fee_denominator;
        Ok(amount_out)
    }
}

// 从DexInfo中提取reserve值
fn extract_reserves_from_dex(dex_info: &DexInfo, side: bool) -> Result<(u64, u64)> {
    if dex_info.pool_data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&dex_info.pool_data[0..8]);
    let reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&dex_info.pool_data[8..16]);
    let reserve_b = u64::from_le_bytes(bytes);

    if side {
        Ok((reserve_b, reserve_a))
    } else {
        Ok((reserve_a, reserve_b))
    }
}

// 计算预期利润
fn calculate_expected_profit(
    amount_in: u64,
    reserve_a: u64,
    reserve_b: u64,
    dex_type: u32,
) -> Result<u64> {
    // 确定手续费
    let fee_numerator = if dex_type == 0 { 25 } else { 100 };
    let fee_denominator = 10000;

    // 计算输出金额
    let amount_out = (reserve_b * amount_in) / (reserve_a + amount_in)
        * (fee_denominator - fee_numerator)
        / fee_denominator;

    // 计算利润
    if amount_out > amount_in {
        Ok(amount_out - amount_in)
    } else {
        Ok(0)
    }
}
