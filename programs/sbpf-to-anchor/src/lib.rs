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
                if data.len() < 16 {
                    return Err(error!(ErrorCode::InvalidInput));
                }
                raydium_get_quote_and_liquidity(
                    &data[0..16],
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
    // 检查输入数据长度
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取reserves
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    let reserve_a = u64::from_le_bytes(bytes);
    output.reserve_a = reserve_a;

    bytes.copy_from_slice(&data[8..16]);
    let reserve_b = u64::from_le_bytes(bytes);
    output.reserve_b = reserve_b;

    // 常量定义
    const FEE_NUMERATOR: u64 = 25; // 0.25% fee
    const FEE_DENOMINATOR: u64 = 10000;

    // 根据side选择输入和输出reserve
    let (reserve_in, reserve_out) = if side {
        (reserve_a, reserve_b)
    } else {
        (reserve_b, reserve_a)
    };

    // 使用1000作为基准输入金额进行报价计算
    let amount_in: u64 = 1000;

    // 使用checked操作进行安全的大数计算
    let amount_in_with_fee = (amount_in as u128)
        .checked_mul((FEE_DENOMINATOR - FEE_NUMERATOR) as u128)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?;

    let numerator = amount_in_with_fee
        .checked_mul(reserve_out as u128)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?;

    let denominator = (reserve_in as u128)
        .checked_mul(FEE_DENOMINATOR as u128)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?
        .checked_add(amount_in_with_fee)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?;

    let amount_out = numerator
        .checked_div(denominator)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?
        .try_into()
        .map_err(|_| error!(ErrorCode::CalculationFailed))?;

    Ok(amount_out)
}

// PumpFun实现
fn pump_fun_get_quote(data: &[u8]) -> Result<u64> {
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取reserves
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    let reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&data[8..16]);
    let reserve_b = u64::from_le_bytes(bytes);

    // PumpFun使用1%的手续费
    const FEE_NUMERATOR: u64 = 100; // 1% fee
    const FEE_DENOMINATOR: u64 = 10000;
    const BASE_AMOUNT: u64 = 1000; // 基准输入金额

    // 使用优化的计算方法
    let amount_in_with_fee = (BASE_AMOUNT as u128)
        .checked_mul((FEE_DENOMINATOR - FEE_NUMERATOR) as u128)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?;

    let numerator = amount_in_with_fee
        .checked_mul(reserve_b as u128)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?;

    let denominator = (reserve_a as u128)
        .checked_mul(FEE_DENOMINATOR as u128)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?
        .checked_add(amount_in_with_fee)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?;

    // 使用优化的除法
    optimize_division(numerator, denominator)
}

fn pump_fun_get_liquidity(data: &[u8], output: &mut Liquidity) -> Result<()> {
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取reserves
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    let reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&data[8..16]);
    let reserve_b = u64::from_le_bytes(bytes);

    // 验证reserves
    if reserve_a == 0 || reserve_b == 0 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 使用优化的乘法来验证k值
    let k = calculate_price_high_precision(reserve_a, reserve_b)?;

    // 验证k值是否在合理范围内
    const MIN_K: u128 = 1_000_000; // 最小k值
    if k < MIN_K {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 设置输出
    output.reserve_a = reserve_a;
    output.reserve_b = reserve_b;

    Ok(())
}

fn pump_fun_get_quote_and_liquidity(
    data: &[u8],
    output: &mut Liquidity,
    side: bool,
) -> Result<u64> {
    // 先获取并验证流动性
    pump_fun_get_liquidity(data, output)?;

    // PumpFun使用1%的手续费
    const FEE_NUMERATOR: u64 = 100; // 1% fee
    const FEE_DENOMINATOR: u64 = 10000;
    const BASE_AMOUNT: u64 = 1000; // 基准输入金额

    // 根据side选择输入和输出reserve
    let (reserve_in, reserve_out) = if side {
        (output.reserve_a, output.reserve_b)
    } else {
        (output.reserve_b, output.reserve_a)
    };

    // 使用优化的价格计算
    calculate_optimal_price(
        reserve_in,
        reserve_out,
        BASE_AMOUNT,
        FEE_NUMERATOR,
        FEE_DENOMINATOR,
    )
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

// 添加辅助函数用于大数计算
fn safe_multiply_u64(a: u64, b: u64) -> Result<u64> {
    (a as u128)
        .checked_mul(b as u128)
        .and_then(|result| result.try_into().ok())
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))
}

fn safe_divide_u64(a: u64, b: u64) -> Result<u64> {
    if b == 0 {
        return Err(error!(ErrorCode::CalculationFailed));
    }
    Ok(a / b)
}

// 添加优化函数用于位操作和大数计算
fn optimize_division(numerator: u128, denominator: u128) -> Result<u64> {
    // 检查除数是否为0
    if denominator == 0 {
        return Err(error!(ErrorCode::CalculationFailed));
    }

    // 如果分子为0，直接返回0
    if numerator == 0 {
        return Ok(0);
    }

    // 计算前导零的数量
    let leading_zeros = numerator.leading_zeros().min(denominator.leading_zeros());

    // 左移以最大化精度
    let shifted_numerator = numerator << leading_zeros;
    let shifted_denominator = denominator << leading_zeros;

    // 执行除法
    let result = shifted_numerator
        .checked_div(shifted_denominator)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?;

    // 转换回u64
    result
        .try_into()
        .map_err(|_| error!(ErrorCode::CalculationFailed))
}

// 添加价格计算优化函数
fn calculate_price(reserve_a: u64, reserve_b: u64) -> Result<u64> {
    // 使用u128进行中间计算
    let reserve_a = reserve_a as u128;
    let reserve_b = reserve_b as u128;

    // 计算价格比率
    optimize_division(reserve_a, reserve_b)
}

// 修改 pump_fun_is_valid 函数实现，使用优化的价格计算
fn pump_fun_is_valid(data: &[u8]) -> Result<bool> {
    if data.len() < 16 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 读取reserves
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    let reserve_a = u64::from_le_bytes(bytes);

    bytes.copy_from_slice(&data[8..16]);
    let reserve_b = u64::from_le_bytes(bytes);

    // 检查reserves是否大于1001
    if reserve_a <= 1001 || reserve_b <= 1001 {
        return Ok(false);
    }

    // 使用优化的价格计算
    let price = calculate_price(reserve_a, reserve_b)?;

    // 检查价格是否在合理范围内
    const MAX_PRICE: u64 = 100000; // 1e5
    const MIN_PRICE: u64 = 1; // 1e-5 after division

    Ok(price >= MIN_PRICE && price <= MAX_PRICE)
}

// 添加更多优化函数
fn optimize_multiplication(a: u64, b: u64) -> Result<u64> {
    // 如果其中一个数为0，直接返回0
    if a == 0 || b == 0 {
        return Ok(0);
    }

    // 使用u128进行中间计算
    let result = (a as u128)
        .checked_mul(b as u128)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))?;

    // 转换回u64
    result
        .try_into()
        .map_err(|_| error!(ErrorCode::CalculationFailed))
}

fn optimize_addition(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))
}

// 添加价格计算的高精度版本
fn calculate_price_high_precision(reserve_a: u64, reserve_b: u64) -> Result<u128> {
    // 使用u128进行所有计算以保持高精度
    let reserve_a = reserve_a as u128;
    let reserve_b = reserve_b as u128;

    // 如果除数为0，返回错误
    if reserve_b == 0 {
        return Err(error!(ErrorCode::CalculationFailed));
    }

    // 计算前导零以最大化精度
    let leading_zeros = reserve_a.leading_zeros().min(reserve_b.leading_zeros());

    // 左移以最大化精度
    let shifted_reserve_a = reserve_a << leading_zeros;
    let shifted_reserve_b = reserve_b << leading_zeros;

    // 执行除法
    shifted_reserve_a
        .checked_div(shifted_reserve_b)
        .ok_or_else(|| error!(ErrorCode::CalculationFailed))
}

// 添加位操作优化函数
fn optimize_bit_shift(value: u64, shift_amount: u32, is_left: bool) -> Result<u64> {
    if shift_amount >= 64 {
        return Ok(0);
    }

    if is_left {
        value.checked_shl(shift_amount)
    } else {
        value.checked_shr(shift_amount)
    }
    .ok_or_else(|| error!(ErrorCode::CalculationFailed))
}

// 添加高精度乘法函数
fn high_precision_multiply(a: u64, b: u64) -> Result<(u64, u64)> {
    // 将输入分解为高32位和低32位
    let a_high = a >> 32;
    let a_low = a & 0xFFFFFFFF;
    let b_high = b >> 32;
    let b_low = b & 0xFFFFFFFF;

    // 计算四个部分的乘积
    let low_low = a_low * b_low;
    let high_low = a_high * b_low;
    let low_high = a_low * b_high;
    let high_high = a_high * b_high;

    // 组合结果
    let mid = high_low + low_high;
    let (low, carry) = low_low.overflowing_add(mid << 32);
    let high = high_high + (mid >> 32) + if carry { 1 } else { 0 };

    Ok((high, low))
}

// 添加优化的价格计算函数
fn calculate_optimal_price(
    reserve_a: u64,
    reserve_b: u64,
    amount_in: u64,
    fee_numerator: u64,
    fee_denominator: u64,
) -> Result<u64> {
    // 验证输入
    if reserve_a == 0 || reserve_b == 0 || amount_in == 0 {
        return Err(error!(ErrorCode::InvalidInput));
    }

    // 计算手续费调整后的输入金额
    let fee_adjusted_amount = optimize_multiplication(amount_in, fee_denominator - fee_numerator)?;

    // 使用高精度乘法计算分子
    let (numerator_high, numerator_low) = high_precision_multiply(fee_adjusted_amount, reserve_b)?;

    // 计算分母
    let denominator_base = optimize_multiplication(reserve_a, fee_denominator)?;
    let denominator = optimize_addition(denominator_base, fee_adjusted_amount)?;

    // 如果分子的高32位不为0，需要特殊处理
    if numerator_high > 0 {
        // 找到合适的位移量
        let shift = numerator_high.leading_zeros();

        // 调整分子和分母
        let adjusted_numerator = (numerator_high << shift) | (numerator_low >> (64 - shift));
        let adjusted_denominator = denominator >> (64 - shift);

        // 执行除法
        if adjusted_denominator == 0 {
            return Err(error!(ErrorCode::CalculationFailed));
        }

        Ok(adjusted_numerator / adjusted_denominator)
    } else {
        // 直接使用低64位进行除法
        if denominator == 0 {
            return Err(error!(ErrorCode::CalculationFailed));
        }

        Ok(numerator_low / denominator)
    }
}

// 添加常量定义
const MAX_CONSTANT: u64 = 0x68db8bac710cc;

// 添加计算利润的函数
fn calculate_profit(
    input_amount: u64,
    quote: u64,
    side: bool,
    dex_info: &DexInfo,
    output: &mut Liquidity,
) -> Result<i64> {
    // 获取报价和流动性
    let quote_amount = get_quote_and_liquidity_internal(dex_info, output, side)?;

    // 获取另一个方向的流动性
    get_liquidity_internal(dex_info, output, !side)?;

    // 获取反向报价
    let reverse_quote = get_quote_internal(output, quote_amount, !side)?;

    // 计算利润
    Ok(reverse_quote as i64 - input_amount as i64)
}

// 添加检查买入金额是否过大的函数
fn is_buy_amount_too_big(
    dex_info: &DexInfo,
    amount: u64,
    quote: u64,
    side: bool,
    output: &mut Liquidity,
) -> Result<bool> {
    // 获取流动性
    get_liquidity_internal(dex_info, output, side)?;

    // 获取报价
    let current_quote = get_quote_internal(output, amount, side)?;

    // 如果报价大于给定的quote，返回true
    if quote > current_quote {
        return Ok(true);
    }

    // 检查DEX是否有效
    let is_valid = is_valid_internal(dex_info)?;
    Ok(!is_valid)
}

// 添加计算最大金额的函数
fn calculate_hinted_max_amount(
    reserve_a: u64,
    reserve_b: u64,
    fee_numerator: u64,
    fee_multiplier: u64,
) -> Result<u64> {
    // 如果reserve_b大于reserve_a，返回0
    if reserve_b > reserve_a {
        return Ok(0);
    }

    // 计算可用余额
    let available = reserve_a - reserve_b;

    // 计算手续费因子
    let fee_denominator = 10000;
    let fee_factor = fee_denominator - fee_numerator;

    // 根据available的大小选择计算方式
    let mut result = if available > MAX_CONSTANT {
        // 先除后乘以fee_factor
        let temp = safe_divide_u64(available, fee_factor)?;
        optimize_multiplication(temp, fee_denominator)?
    } else {
        // 先乘后除以fee_factor
        let temp = optimize_multiplication(available, fee_denominator)?;
        safe_divide_u64(temp, fee_factor)?
    };

    // 处理结果与fee_multiplier的乘除操作
    result = if result > MAX_CONSTANT {
        // 先除后乘以fee_multiplier
        let temp = safe_divide_u64(result, fee_denominator)?;
        optimize_multiplication(temp, fee_multiplier)?
    } else {
        // 先乘后除以fee_multiplier
        let temp = optimize_multiplication(result, fee_multiplier)?;
        safe_divide_u64(temp, fee_denominator)?
    };

    Ok(result)
}

// 添加计算上限的函数
fn calculate_upper_bound(
    reserve_a: u64,
    reserve_info: &ReserveInfo,
    fee_multiplier: u64,
    is_token_a: bool,
) -> Result<u64> {
    // 初始化结果为0
    let mut result: u64 = 0;

    // 确定手续费
    let fee_numerator = if reserve_info.dex_type == 0 {
        9975 // Raydium fee
    } else {
        9900 // PumpFun fee
    };

    // 获取相应的reserve值
    let reserve_value = if is_token_a {
        reserve_info.reserve_a
    } else {
        reserve_info.reserve_b
    };

    // 如果reserve_value大于reserve_a，进行计算
    if reserve_value > reserve_a {
        // 计算可用余额
        let available = reserve_value - reserve_a;

        // 根据available的大小选择计算方式
        let mut temp = if available > MAX_CONSTANT {
            // 先除后乘
            let div_result = safe_divide_u64(available, fee_numerator)?;
            optimize_multiplication(div_result, 10000)?
        } else {
            // 先乘后除
            let mul_result = optimize_multiplication(available, 10000)?;
            safe_divide_u64(mul_result, fee_numerator)?
        };

        // 处理结果与fee_multiplier的乘除操作
        result = if temp > MAX_CONSTANT {
            // 先除后乘
            let div_result = safe_divide_u64(temp, 10000)?;
            optimize_multiplication(div_result, fee_multiplier)?
        } else {
            // 先乘后除
            let mul_result = optimize_multiplication(temp, fee_multiplier)?;
            safe_divide_u64(mul_result, 10000)?
        };
    }

    Ok(result)
}

// 添加 is_valid_internal 函数
fn is_valid_internal(dex_info: &DexInfo) -> Result<bool> {
    match dex_info.dex_type {
        0 => {
            // Raydium
            raydium_is_valid(&dex_info.pool_data)
        }
        1 => {
            // PumpFun
            pump_fun_is_valid(&dex_info.pool_data)
        }
        _ => Err(error!(ErrorCode::UnsupportedDex)),
    }
}
