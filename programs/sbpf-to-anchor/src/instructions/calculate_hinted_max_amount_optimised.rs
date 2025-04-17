use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 计算优化后的最大交易金额
/// 该函数采用了多种优化技巧来处理大数计算和提高精度
/// 主要用于DEX交易中的金额计算，特别适用于大额交易场景
/// 计算在给定费用率和可用余额下的最大可交易金额
pub fn calculate_hinted_max_amount_optimised(
    max_input: u64,
    available: u64,
    fee_numerator: u64,
    fee_denominator: u64,
) -> Result<u64> {
    // 验证输入金额是否超过可用金额
    if max_input > available {
        return Ok(0);
    }

    // 计算实际可用金额，使用saturating_sub防止下溢
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
