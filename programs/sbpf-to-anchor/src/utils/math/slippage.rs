/// 帮助函数：计算调整后的金额
pub fn calculate_adjusted_amount(amount: u64, is_large_amount: bool) -> u64 {
    if is_large_amount {
        amount / 9975 * 10000
    } else {
        amount * 10000 / 9975
    }
}

/// 帮助函数：计算滑点调整
pub fn apply_slippage(amount: u64, slippage_bps: u16, is_large_amount: bool) -> u64 {
    if is_large_amount {
        amount / 10000 * slippage_bps as u64
    } else {
        amount * slippage_bps as u64 / 10000
    }
}
