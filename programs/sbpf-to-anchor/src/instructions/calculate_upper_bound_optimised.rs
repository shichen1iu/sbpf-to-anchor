use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 优化版本的上界计算账户结构
/// 用于计算交易金额的理论上限
#[derive(Accounts)]
pub struct CalculateUpperBoundOptimised<'info> {
    /// 交易金额账户
    #[account(mut)]
    pub amount: Account<'info, AmountData>,

    /// DEX类型账户
    /// 用于确定使用的费率
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,

    /// 代币数量账户
    /// 存储代币A和B的可用数量
    #[account(mut)]
    pub amounts: Account<'info, TokenAmounts>,

    /// 代币类型标志账户
    /// 用于确定使用哪个代币的数量
    #[account(mut)]
    pub is_token_a: Account<'info, IsTokenA>,

    /// 乘数账户
    /// 用于调整计算结果
    #[account(mut)]
    pub multiplier: Account<'info, AmountData>,
}

/// 计算优化版本的交易金额上界
/// 考虑费率、可用数量和乘数因素
///
/// # 功能特点
/// * 支持不同DEX的费率设置
/// * 优化的大数计算处理
/// * 考虑代币可用性
/// * 使用乘数进行结果调整
///
/// # 计算流程
/// 1. 确定使用的费率
/// 2. 选择合适的代币数量
/// 3. 计算输出金额
/// 4. 应用乘数调整
///
/// # 返回值
/// * `Result<u64>` - 返回计算得到的上界值
///   - 如果可行，返回计算结果
///   - 如果不可行，返回0
pub fn calculate_upper_bound_optimised(ctx: Context<CalculateUpperBoundOptimised>) -> Result<u64> {
    let accounts = &ctx.accounts;
    let dex_type = accounts.dex_type.dex_type;
    let amount = accounts.amount.amount;

    // 设置费率
    // 默认费率为0.25%（9975 = 99.75%）
    let mut fee_rate = 9975u64;

    // 如果是DEX类型1，使用1%的费率（9900 = 99%）
    if dex_type == 1 {
        fee_rate = 9900;
    }

    // 根据代币类型标志选择可用数量
    // is_token_a为0使用代币A的数量，否则使用代币B的数量
    let available = if accounts.is_token_a.is_token_a == 0 {
        accounts.amounts.token_a_amount
    } else {
        accounts.amounts.token_b_amount
    };

    // 检查可用数量是否足够
    if available > amount {
        // 计算剩余可用数量
        let remaining = amount.saturating_sub(available);
        let output_amount;

        // 使用阈值优化计算路径
        // 0x68db8bac710cc ≈ 1.8e14 作为阈值
        if remaining > 0x68db8bac710cc {
            // 大数处理：先除后乘
            output_amount = remaining / fee_rate * 10000;
        } else {
            // 小数处理：先乘后除
            output_amount = remaining * 10000 / fee_rate;
        }

        // 应用乘数调整
        let result;
        let multiplier = accounts.multiplier.amount;

        // 同样使用阈值优化乘数调整的计算
        if output_amount > 0x68db8bac710cc {
            // 大数处理：先除后乘
            result = output_amount / 10000 * multiplier;
        } else {
            // 小数处理：先乘后除
            result = output_amount * multiplier / 10000;
        }

        Ok(result)
    } else {
        // 如果可用数量不足，返回0
        Ok(0)
    }
}
