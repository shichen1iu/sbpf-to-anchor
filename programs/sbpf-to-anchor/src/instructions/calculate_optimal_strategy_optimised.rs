use super::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 优化版本的最优策略计算账户结构
/// 用于DEX交易中的高效策略优化计算
#[derive(Accounts)]
pub struct CalculateOptimalStrategyOptimised<'info> {
    /// 代币A的数量账户
    #[account(mut)]
    pub amount_a: Account<'info, AmountData>,

    /// 代币B的数量账户
    #[account(mut)]
    pub amount_b: Account<'info, AmountData>,

    /// 最小交易金额限制账户
    #[account(mut)]
    pub min_amount: Account<'info, AmountData>,

    /// 乘数数据账户，用于金额调整计算
    #[account(mut)]
    pub multiplier: Account<'info, MultiplierData>,

    /// 交换类型账户，影响费率设置
    #[account(mut)]
    pub swap_type: Account<'info, SwapType>,

    /// 是否反向交易的标志账户
    #[account(mut)]
    pub is_reverse: Account<'info, ReverseFlag>,

    /// 是否需要验证的标志账户
    #[account(mut)]
    pub need_verify: Account<'info, NeedVerify>,

    /// 计算结果存储账户
    #[account(mut)]
    pub result: Account<'info, ResultData>,

    /// DEX类型账户，用于区分不同的DEX平台
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,

    /// 流动性池数据账户
    /// CHECK: 池子数据账户
    pub pool_data: AccountInfo<'info>,
}

/// 计算优化版本的最优交易策略
/// 使用迭代优化方法寻找最佳交易金额和利润
///
/// # 功能特点
/// * 支持不同DEX平台（Raydium和PumpFun）
/// * 使用平方根策略优化搜索
/// * 考虑费率和流动性因素
/// * 实现了自适应的金额调整
///
/// # 返回值
/// * `Result<bool>` - 返回计算是否成功
///   - true: 找到可行策略
///   - false: 未找到满足条件的策略
pub fn calculate_optimal_strategy_optimised(
    ctx: Context<CalculateOptimalStrategyOptimised>,
) -> Result<bool> {
    let accounts = &ctx.accounts;

    // 初始化基础参数
    // fee_rate: 费率基数（9975 = 0.9975，9900 = 0.99）
    let mut fee_rate = 9975u64;
    let mut profit = 0u64;

    // 根据交换类型设置不同的费率
    // swap_type为1时使用更高的费率
    if accounts.swap_type.swap_type == 1 {
        fee_rate = 9900;
    }

    // 根据交易方向选择初始金额
    // reverse为true时使用B代币金额，否则使用A代币金额
    let mut amount = if accounts.is_reverse.reverse {
        accounts.amount_b.amount
    } else {
        accounts.amount_a.amount
    };

    // 验证金额是否满足最小交易限制
    if amount <= accounts.min_amount.amount {
        return Ok(false);
    }

    // 金额调整计算
    // threshold: 大数阈值（约1.8e14）用于优化计算路径
    let threshold = 0x68db8bac710cc;
    // 根据金额大小选择不同的计算路径，避免数值溢出
    amount = if amount > threshold {
        (amount / fee_rate) * 10000
    } else {
        (amount * 10000) / fee_rate
    };

    // 应用乘数调整并验证结果
    // 同样使用阈值优化计算路径
    if amount > threshold {
        amount = (amount / 10000) * accounts.multiplier.multiplier;
        if amount <= 999 {
            return Ok(false);
        }
    } else {
        amount = (amount * accounts.multiplier.multiplier) / 10000;
        if amount <= 999 {
            return Ok(false);
        }
    }

    // 如果不需要验证，直接保存结果并返回
    if !accounts.need_verify.need_verify {
        accounts.result.result = amount;
        return Ok(true);
    }

    // 开始迭代优化计算
    let mut best_amount = amount; // 记录最佳金额
    let mut best_profit = 0i64; // 记录最佳利润
    let base_amount = 1000u64; // 基础金额（最小交易单位）

    // 迭代搜索最优解
    // 最多进行15次迭代，或当金额调整小于10000时停止
    let mut iteration = 0u64;
    while iteration < 15 && (amount - base_amount) > 10000 {
        // 根据DEX类型获取当前报价和流动性数据
        let (quote1, reserve_a1, reserve_b1) = if accounts.dex_type.dex_type == 0 {
            // Raydium DEX的报价获取
            raydium_get_quote_and_liquidity(
                &accounts.pool_data,
                amount,
                accounts.is_reverse.reverse,
            )?
        } else {
            // PumpFun DEX的报价获取
            pump_fun_get_quote_and_liquidity(
                &accounts.pool_data,
                amount,
                accounts.is_reverse.reverse,
            )?
        };

        // 使用平方根策略计算调整系数
        // 0.1875和0.3125是经验优化系数
        let sqrt_val = ((quote1 as f64).sqrt() as u64);
        let adjust1 = (sqrt_val as f64 * 0.1875) as u64;
        let adjust2 = (sqrt_val as f64 * 0.3125) as u64;

        // 计算两个测试金额
        let test_amount1 = base_amount + adjust1;
        let test_amount2 = base_amount + adjust2;

        // 获取测试金额的报价
        let (quote2, _, _) = if accounts.dex_type.dex_type == 0 {
            raydium_get_quote_and_liquidity(
                &accounts.pool_data,
                test_amount1,
                accounts.is_reverse.reverse,
            )?
        } else {
            pump_fun_get_quote_and_liquidity(
                &accounts.pool_data,
                test_amount1,
                accounts.is_reverse.reverse,
            )?
        };

        let (quote3, _, _) = if accounts.dex_type.dex_type == 0 {
            raydium_get_quote_and_liquidity(
                &accounts.pool_data,
                test_amount2,
                accounts.is_reverse.reverse,
            )?
        } else {
            pump_fun_get_quote_and_liquidity(
                &accounts.pool_data,
                test_amount2,
                accounts.is_reverse.reverse,
            )?
        };

        // 计算两个测试方案的利润
        let profit1 = quote2 as i64 - test_amount1 as i64;
        let profit2 = quote3 as i64 - test_amount2 as i64;

        // 更新最优解
        // 如果找到更好的利润，更新最佳金额和利润
        if profit1 > best_profit {
            best_profit = profit1;
            best_amount = test_amount1;
        }

        if profit2 > best_profit {
            best_profit = profit2;
            best_amount = test_amount2;
        }

        // 使用当前最佳金额继续迭代
        amount = best_amount;
        iteration += 1;
    }

    // 保存最终的计算结果
    accounts.result.result = best_amount;
    accounts.result.profit = best_profit as u64;

    Ok(true)
}
