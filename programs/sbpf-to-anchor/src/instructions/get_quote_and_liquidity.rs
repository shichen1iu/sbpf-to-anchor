use super::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 获取报价和流动性账户结构
/// 用于同时查询DEX池子的报价和流动性数据
#[derive(Accounts)]
pub struct GetQuoteAndLiquidity<'info> {
    /// 输入数据账户
    /// 包含池子状态和流动性数据
    pub input_data: AccountInfo<'info>,

    /// 金额账户
    /// 存储交易金额信息
    #[account(mut)]
    pub amount: Account<'info, AmountData>,

    /// 交易方向标志账户
    /// 指示交易的方向（正向/反向）
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
}

/// 获取DEX报价和流动性主函数
///
/// # 功能特点
/// * 多DEX支持
/// * 组合查询优化
/// * 原子性操作
///
/// # 参数
/// * `dex_type` - DEX类型(0: Raydium, 1: Pump.fun)
///
/// # 返回值
/// * `(u64, u64, u64)` - (quote, reserve_a, reserve_b)
///   - quote: 交易报价
///   - reserve_a: 代币A的储备量
///   - reserve_b: 代币B的储备量
///
/// # 错误处理
/// * 无效的DEX类型
/// * 数据读取错误
pub fn get_quote_and_liquidity(
    ctx: Context<GetQuoteAndLiquidity>,
    dex_type: u8,
) -> Result<(u64, u64, u64)> {
    let accounts = &ctx.accounts;

    if dex_type == 0 {
        // Raydium DEX查询
        // 获取报价和流动性数据
        raydium_get_quote_and_liquidity(
            accounts.input_data.to_account_info(),
            accounts.amount.amount,
            accounts.reverse.reverse,
        )
    } else if dex_type == 1 {
        // Pump.fun DEX查询
        // 获取报价和流动性数据
        pump_fun_get_quote_and_liquidity(
            accounts.input_data.to_account_info(),
            accounts.amount.amount,
            accounts.reverse.reverse,
        )
    } else {
        // 处理无效的DEX类型
        Err(ProgramError::InvalidArgument.into())
    }
}

/// Raydium报价和流动性组合查询函数
///
/// # 功能特点
/// * 优化的组合查询
/// * 单次账户读取
/// * 原子性保证
///
/// # 参数
/// * `input_data` - 池子数据账户
/// * `amount` - 交易金额
/// * `reverse` - 交易方向
///
/// # 实现说明
/// 通过单次调用获取两种数据，优化性能
pub fn raydium_get_quote_and_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64, u64)> {
    // 获取流动性数据
    // 克隆账户信息以便重复使用
    let (reserve_a, reserve_b) = raydium_get_liquidity(input_data.clone(), amount, reverse)?;

    // 获取报价数据
    // 复用已克隆的账户信息
    let quote = raydium_get_quote(input_data, amount, reverse)?;

    // 返回组合结果
    Ok((quote, reserve_a, reserve_b))
}

/// Pump.fun报价和流动性组合查询函数
///
/// # 功能特点
/// * 优化的组合查询
/// * 单次账户读取
/// * 原子性保证
///
/// # 参数
/// * `input_data` - 池子数据账户
/// * `amount` - 交易金额
/// * `reverse` - 交易方向
///
/// # 实现说明
/// 通过单次调用获取两种数据，优化性能
pub fn pump_fun_get_quote_and_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64, u64)> {
    // 获取流动性数据
    // 克隆账户信息以便重复使用
    let (reserve_a, reserve_b) = pump_fun_get_liquidity(input_data.clone(), amount, reverse)?;

    // 获取报价数据
    // 复用已克隆的账户信息
    let quote = pump_fun_get_quote(input_data, amount, reverse)?;

    // 返回组合结果
    Ok((quote, reserve_a, reserve_b))
}
