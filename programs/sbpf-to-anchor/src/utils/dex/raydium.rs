use crate::utils::*;
use anchor_lang::prelude::*;

/// Raydium DEX的报价计算函数
/// 这个函数实现了Raydium的报价计算算法
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r1)
/// * `quote_amount` - 报价金额 (对应汇编中的 r2)
/// * `swap_direction` - 交换方向 (对应汇编中的 r3)
pub fn raydium_get_quote(
    liquidity_data: &[u8],
    quote_amount: u64,
    swap_direction: bool,
) -> Result<u64> {
    // 计算基础费率调整 (div64 r8, 10000; mul64 r8, -25)
    let mut fee_adjusted = quote_amount / 10000 * 25;

    if !swap_direction {
        // 处理 A->B 方向的交换 (jeq r3, 0, lbb_436)

        // 读取流动性数据 (ldxdw r0, [r1+0x0]; ldxdw r3, [r1+0x8])
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析 A 池的高低位数据
        let pool_a_lo = pool_a & 0xFFFFFFFF;
        let pool_a_hi = pool_a >> 32;

        // 解析 B 池的高低位数据
        let pool_b_lo = pool_b & 0xFFFFFFFF;
        let pool_b_hi = pool_b >> 32;

        // 计算交叉乘积
        let mut cross_mul1 = pool_a_hi * pool_b_lo;
        let cross_mul2 = pool_a_lo * pool_b_lo >> 32;
        let cross_sum = cross_mul1 + cross_mul2;

        // 调整费率后的数量
        let adjusted_amount = quote_amount + fee_adjusted;

        // 计算第二组交叉乘积
        let cross_mul3 = pool_a_hi * pool_b_hi;
        let cross_hi = cross_sum >> 32;
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积和
        let low_mul = pool_a_lo * pool_b_hi;
        let cross_lo = cross_sum & 0xFFFFFFFF;
        let low_sum = low_mul + (cross_lo << 32 >> 32);
        let low_shift = low_sum >> 32;
        let total_sum = cross_sum2 + low_shift;

        // 计算最终数量
        let amount_adjusted = adjusted_amount + pool_b;
        let prod = pool_a * pool_b;

        // 如果总和为0，直接执行简单的除法 (jne r9, 0)
        if total_sum == 0 {
            return Ok(prod / amount_adjusted);
        }

        // 执行复杂的大数除法算法
        let result = complex_division(prod, amount_adjusted, total_sum)?;
        return Ok(result);
    } else {
        // 处理 B->A 方向的交换

        // 读取流动性数据
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析池数据
        let pool_a_lo = pool_a & 0xFFFFFFFF;
        let pool_a_hi = pool_a >> 32;
        let pool_b_lo = pool_b & 0xFFFFFFFF;
        let pool_b_hi = pool_b >> 32;

        // 计算交叉乘积
        let mut cross_mul1 = pool_b_hi * pool_a_lo;
        let cross_mul2 = pool_b_lo * pool_a_lo >> 32;
        let cross_sum = cross_mul1 + cross_mul2;

        // 调整费率后的数量
        let adjusted_amount = quote_amount + fee_adjusted;

        // 计算第二组交叉乘积
        let cross_mul3 = pool_b_hi * pool_a_hi;
        let cross_hi = cross_sum >> 32;
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积和
        let low_mul = pool_b_lo * pool_a_hi;
        let cross_lo = cross_sum & 0xFFFFFFFF;
        let low_sum = low_mul + (cross_lo << 32 >> 32);
        let low_shift = low_sum >> 32;
        let total_sum = cross_sum2 + low_shift;

        // 计算最终数量
        let amount_adjusted = adjusted_amount + pool_a;
        let prod = pool_a * pool_b;

        // 如果总和为0，直接执行简单的除法
        if total_sum == 0 {
            return Ok(prod / amount_adjusted);
        }

        // 执行复杂的大数除法算法
        let result = complex_division(prod, amount_adjusted, total_sum)?;
        return Ok(result);
    }
}
