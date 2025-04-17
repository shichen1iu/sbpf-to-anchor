use super::*;
use anchor_lang::prelude::*;

/// 获取流动性函数
/// 根据不同的DEX类型调用对应的流动性获取函数
///
/// # 参数说明
/// * `liquidity_info` - 流动性信息 (对应汇编中的 r1)
/// * `output` - 输出缓冲区 (对应汇编中的 r4)
pub fn get_liquidity(liquidity_info: &[u8], output: &mut [u8]) -> Result<()> {
    // 读取DEX类型 (ldxw r5, [r1+0x0])
    let dex_type = u32::from_le_bytes(liquidity_info[0..4].try_into().unwrap());

    // 根据DEX类型调用不同的流动性获取函数
    match dex_type {
        // Raydium DEX (jeq r5, 0)
        0 => {
            // 设置DEX类型为0 (mov64 r5, 0; stxw [r4+0x0], r5)
            output[0..4].copy_from_slice(&0u32.to_le_bytes());

            // 跳过类型字段，调用Raydium流动性获取函数 (add64 r1, 8; add64 r4, 8; call raydium_get_liquidity)
            raydium_get_liquidity(&liquidity_info[8..], &mut output[8..], false, false)
        }
        // Pump DEX (jeq r5, 1)
        1 => {
            // 设置DEX类型为1 (mov64 r5, 1; stxw [r4+0x0], r5)
            output[0..4].copy_from_slice(&1u32.to_le_bytes());

            // 跳过类型字段，调用Pump流动性获取函数 (add64 r1, 8; add64 r4, 8; call pump_fun_get_liquidity)
            pump_fun_get_liquidity(&liquidity_info[8..], &mut output[8..], false, false)
        }
        // 未知的DEX类型，直接返回 (jne r5, 0, lbb_30)
        _ => Ok(()),
    }
}

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
        // Raydium DEX (call raydium_get_quote)
        0 => {
            // 跳过类型字段，使用后续的流动性数据
            raydium_get_quote(&liquidity_info[8..], quote_amount, swap_direction)
        }
        // Pump DEX (call pump_fun_get_quote)
        1 => {
            // 跳过类型字段，使用后续的流动性数据
            pump_fun_get_quote(&liquidity_info[8..], quote_amount, swap_direction)
        }
        // 未知的DEX类型返回0
        _ => Ok(0),
    }
}

/// 获取报价和流动性函数
/// 根据不同的DEX类型调用对应的报价和流动性获取函数
///
/// # 参数说明
/// * `liquidity_info` - 流动性信息 (对应汇编中的 r1)
/// * `output` - 输出缓冲区 (对应汇编中的 r4)
pub fn get_quote_and_liquidity(liquidity_info: &[u8], output: &mut [u8]) -> Result<u64> {
    // 读取DEX类型 (ldxw r5, [r1+0x0])
    let dex_type = u32::from_le_bytes(liquidity_info[0..4].try_into().unwrap());

    // 检查是否为Pump DEX (jeq r5, 1, lbb_41)
    if dex_type == 1 {
        // 设置DEX类型为1 (mov64 r5, 1; stxw [r4+0x0], r5)
        output[0..4].copy_from_slice(&1u32.to_le_bytes());

        // 跳过类型字段，调用Pump报价和流动性获取函数 (add64 r1, 8; add64 r4, 8; call pump_fun_get_quote_and_liquidity)
        pump_fun_get_quote_and_liquidity(&liquidity_info[8..], &mut output[8..], false, false)
    }
    // 检查是否为Raydium DEX (jne r5, 0, lbb_46)
    else if dex_type == 0 {
        // 设置DEX类型为0 (mov64 r5, 0; stxw [r4+0x0], r5)
        output[0..4].copy_from_slice(&0u32.to_le_bytes());

        // 跳过类型字段，调用Raydium报价和流动性获取函数 (add64 r1, 8; add64 r4, 8; call raydium_get_quote_and_liquidity)
        raydium_get_quote_and_liquidity(&liquidity_info[8..], &mut output[8..], false, false)
    }
    // 未知的DEX类型返回0 (mov64 r0, 0)
    else {
        Ok(0)
    }
}
