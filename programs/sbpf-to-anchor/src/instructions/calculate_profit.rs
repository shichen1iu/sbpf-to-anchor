use super::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 原始版本的利润计算账户结构
/// 用于在单个DEX内计算交易利润
#[derive(Accounts)]
pub struct CalculateProfit<'info> {
    /// 输入数据账户
    /// 包含交易所需的基础数据
    pub input_data: AccountInfo<'info>,

    /// 交易金额账户
    #[account(mut)]
    pub amount: Account<'info, AmountData>,

    /// 交易方向标志账户
    /// 用于标识是正向还是反向交易
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,

    /// DEX类型账户
    /// 用于区分不同的DEX平台（Raydium/PumpFun）
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
}

/// 计算交易利润的函数
/// 在同一个DEX内计算正反向交易的利润
///
/// # 功能特点
/// * 支持单一DEX内的交易利润计算
/// * 考虑正反向交易方向
/// * 包含流动性检查
/// * 使用饱和减法保证计算安全
///
/// # 计算流程
/// 1. 获取初始报价和流动性
/// 2. 检查同池子的流动性状态
/// 3. 计算反向交易的报价
/// 4. 计算最终利润
///
/// # 返回值
/// * `Result<u64>` - 返回计算得到的利润值
pub fn calculate_profit(ctx: Context<CalculateProfit>) -> Result<u64> {
    let accounts = &ctx.accounts;

    // 获取基础参数
    let amount = accounts.amount.amount; // 交易金额
    let reverse_flag = accounts.reverse.reverse; // 交易方向标志

    // 第一步：获取初始报价和流动性数据
    // 根据DEX类型选择对应的接口
    let (quote1, reserve_a, reserve_b) = if accounts.dex_type.dex_type == 0 {
        // Raydium DEX的报价和流动性查询
        raydium_get_quote_and_liquidity(
            accounts.input_data.to_account_info(),
            amount,
            reverse_flag,
        )?
    } else {
        // PumpFun DEX的报价和流动性查询
        pump_fun_get_quote_and_liquidity(
            accounts.input_data.to_account_info(),
            amount,
            reverse_flag,
        )?
    };

    // 第二步：获取当前池子的流动性状态
    // 验证池子是否有足够的流动性支持交易
    let (liquidity_a, liquidity_b) = if accounts.dex_type.dex_type == 0 {
        // 查询Raydium池子的流动性
        raydium_get_liquidity(accounts.input_data.to_account_info(), amount, reverse_flag)?
    } else {
        // 查询PumpFun池子的流动性
        pump_fun_get_liquidity(accounts.input_data.to_account_info(), amount, reverse_flag)?
    };

    // 第三步：获取反向交易的报价
    // 注意这里使用!reverse_flag来反转交易方向
    let reverse_quote = if accounts.dex_type.dex_type == 0 {
        // 获取Raydium反向交易报价
        raydium_get_quote(accounts.input_data.to_account_info(), quote1, !reverse_flag)?
    } else {
        // 获取PumpFun反向交易报价
        pump_fun_get_quote(accounts.input_data.to_account_info(), quote1, !reverse_flag)?
    };

    // 最后：计算最终利润
    // 使用饱和减法避免数值下溢
    // 利润 = 反向交易报价 - 原始金额
    Ok(reverse_quote.saturating_sub(amount))
}
