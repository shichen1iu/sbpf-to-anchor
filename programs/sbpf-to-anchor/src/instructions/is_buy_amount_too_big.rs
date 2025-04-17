use super::*;
use crate::states::*;
use anchor_lang::prelude::*;

/// 购买金额验证账户结构
/// 用于验证交易金额是否超过池子承受范围
#[derive(Accounts)]
pub struct IsBuyAmountTooBig<'info> {
    /// 输入数据账户
    /// 包含池子状态和流动性数据
    /// CHECK: 在指令处理中验证数据有效性
    pub input_data: AccountInfo<'info>,

    /// 交易金额账户
    /// 存储用户希望购买的代币数量
    #[account(mut)]
    pub amount: Account<'info, AmountData>,

    /// 阈值金额账户
    /// 存储可接受的最小交易金额
    #[account(mut)]
    pub threshold: Account<'info, AmountData>,

    /// 交易方向标志账户
    /// 指示是正向还是反向交易
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,

    /// DEX类型账户
    /// 指示使用的DEX（0: Raydium, 1: Pump.fun）
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
}

/// 验证购买金额是否过大的函数
///
/// # 功能特点
/// * 支持多个DEX的流动性检查
/// * 自动处理正反向交易
/// * 综合验证池子状态
/// * 阈值比较验证
///
/// # 验证步骤
/// 1. 获取池子流动性数据
/// 2. 计算交易报价
/// 3. 与阈值比较
/// 4. 验证池子有效性
///
/// # 返回值
/// * `Result<bool>` - true表示金额过大，false表示金额在可接受范围内
pub fn is_buy_amount_too_big(ctx: Context<IsBuyAmountTooBig>) -> Result<bool> {
    let accounts = &ctx.accounts;
    let input_data = accounts.input_data.to_account_info();
    let dex_type = accounts.dex_type.dex_type;
    let amount = accounts.amount.amount;
    let threshold = accounts.threshold.amount;
    let reverse = accounts.reverse.reverse;

    // 首先获取流动性数据
    // 根据DEX类型选择不同的流动性查询方法
    let (reserve_a, reserve_b) = if dex_type == 0 {
        // Raydium流动性查询
        raydium_get_liquidity(input_data.clone(), amount, reverse)?
    } else {
        // Pump.fun流动性查询
        pump_fun_get_liquidity(input_data.clone(), amount, reverse)?
    };

    // 获取交易报价
    // 根据DEX类型选择不同的报价计算方法
    let quote = if dex_type == 0 {
        // Raydium报价计算
        raydium_get_quote(input_data.clone(), amount, reverse)?
    } else {
        // Pump.fun报价计算
        pump_fun_get_quote(input_data.clone(), amount, reverse)?
    };

    // 检查报价是否低于阈值
    // 如果报价低于阈值，表示购买金额过大
    if threshold > quote {
        return Ok(true);
    }

    // 进一步验证池子状态是否有效
    // 根据DEX类型选择不同的验证方法
    let is_valid_result = if dex_type == 0 {
        // Raydium池子验证
        raydium_is_valid(input_data)?
    } else {
        // Pump.fun池子验证
        pump_fun_is_valid(input_data)?
    };

    // 如果池子无效，也视为金额过大
    // 返回池子验证结果的取反值
    Ok(!is_valid_result)
}
