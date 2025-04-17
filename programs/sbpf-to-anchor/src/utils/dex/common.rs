use super::*;
use anchor_lang::prelude::*;

/// 获取报价函数
/// 根据不同的DEX类型调用对应的报价计算函数
///
/// # 参数说明
/// * `liquidity_info` - 流动性信息 (对应汇编中的 r1)
/// * `quote_amount` - 报价金额 (对应汇编中的 r2)
/// * `swap_direction` - 交换方向 (对应汇编中的 r3)
pub fn get_quote(liquidity_info: &[u8], quote_amount: u64, swap_direction: bool) -> Result<u64> {
    // 读取DEX类型 (ldxw r4, [r1+0x0])
    let dex_type = u32::from_le_bytes(liquidity_info[0..4].try_into().unwrap());

    // 根据DEX类型调用不同的报价计算函数
    match dex_type {
        0 => {
            // Raydium DEX (call raydium_get_quote)
            // 跳过类型字段，使用后续的流动性数据
            raydium_get_quote(&liquidity_info[8..], quote_amount, swap_direction)
        }
        1 => {
            // Pump DEX (call pump_fun_get_quote)
            // 跳过类型字段，使用后续的流动性数据
            pump_fun_get_quote(&liquidity_info[8..], quote_amount, swap_direction)
        }
        _ => Ok(0), // 未知的DEX类型返回0
    }
}
