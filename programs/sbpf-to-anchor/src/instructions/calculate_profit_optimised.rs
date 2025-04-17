use super::get_quote_and_liquidity::*;
use crate::states::*;
use crate::utils::*;
use super::*;
use anchor_lang::prelude::*;

/// 优化版本的利润计算账户结构
/// 用于计算DEX交易中的预期利润
#[derive(Accounts)]
pub struct CalculateProfitOptimised<'info> {
    /// 交易金额账户
    #[account(mut)]
    pub amount: Account<'info, AmountData>,

    /// 正向交易的DEX类型账户
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,

    /// 反向交易的DEX类型账户
    #[account(mut)]
    pub dex_type_reverse: Account<'info, DexType>,

    /// 正向交易报价上下文
    /// 用于获取正向交易的报价信息
    pub quote_ctx: AccountInfo<'info>,

    /// 流动性上下文
    /// 用于获取池子的流动性信息
    pub liquidity_ctx: AccountInfo<'info>,

    /// 反向交易报价上下文
    /// 用于获取反向交易的报价信息
    pub quote_ctx_reverse: AccountInfo<'info>,
}

/// 计算优化版本的交易利润
/// 通过比较正反向交易的报价差异来计算潜在利润
///
/// # 功能特点
/// * 支持跨DEX平台套利（Raydium和PumpFun）
/// * 分别计算正向和反向交易的报价
/// * 考虑流动性因素
/// * 使用饱和减法确保安全计算
///
/// # 返回值
/// * `Result<u64>` - 返回计算得到的利润值
///   - 如果交易可行，返回正向利润值
///   - 如果交易不可行，返回0
pub fn calculate_profit_optimised(ctx: Context<CalculateProfitOptimised>) -> Result<u64> {
    let accounts = &ctx.accounts;

    // 获取输入金额
    let amount = accounts.amount.amount;

    // 第一步：获取正向交易的报价和流动性数据
    // 根据DEX类型选择不同的接口
    let (quote1, reserve_a, reserve_b) = if accounts.dex_type.dex_type == 0 {
        // 使用Raydium DEX的接口
        raydium_get_quote_and_liquidity(accounts.quote_ctx.clone(), amount, false)?
    } else {
        // 使用PumpFun DEX的接口
        pump_fun_get_quote_and_liquidity(accounts.quote_ctx.clone(), amount, false)?
    };

    // 第二步：获取反向交易的流动性数据
    // 这一步检查反向交易的可行性
    let (reverse_reserve_a, reverse_reserve_b) = if accounts.dex_type_reverse.dex_type == 0 {
        // 使用Raydium DEX的流动性查询
        raydium_get_liquidity(accounts.liquidity_ctx.clone(), amount, true)?
    } else {
        // 使用PumpFun DEX的流动性查询
        pump_fun_get_liquidity(accounts.liquidity_ctx.clone(), amount, true)?
    };

    // 第三步：获取反向交易的最终报价
    // 这决定了最终的利润空间
    let quote2 = if accounts.dex_type_reverse.dex_type == 0 {
        // 获取Raydium DEX的反向报价
        raydium_get_quote(accounts.quote_ctx_reverse.clone(), amount, true)?
    } else {
        // 获取PumpFun DEX的反向报价
        pump_fun_get_quote(accounts.quote_ctx_reverse.clone(), amount, true)?
    };

    // 最后：计算实际利润
    // 使用饱和减法避免下溢
    // 利润 = 输出金额 - 输入金额
    Ok(quote2.saturating_sub(amount))
}
