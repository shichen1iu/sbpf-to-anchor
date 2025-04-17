use crate::utils::*;
use anchor_lang::prelude::*;

/// Pump DEX的报价计算函数
/// 这个函数实现了Pump的特殊报价计算算法
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r1)
/// * `quote_amount` - 报价金额 (对应汇编中的 r2)
/// * `swap_direction` - 交换方向 (对应汇编中的 r3)
pub fn pump_fun_get_quote(
    liquidity_data: &[u8],
    quote_amount: u64,
    swap_direction: bool,
) -> Result<u64> {
    if !swap_direction {
        // 处理 A->B 方向的交换 (jeq r3, 0, lbb_1365)

        // 创建临时缓冲区
        let mut temp_buffer1 = [0u8; 16];
        let mut temp_buffer2 = [0u8; 16];

        // 调用辅助函数 (call function_9839)
        function_9839(&mut temp_buffer1, 0, 100, 0)?;

        // 调用缓冲区处理函数 (call function_9883)
        function_9883(
            &mut temp_buffer2,
            u64::from_le_bytes(temp_buffer1[0..8].try_into().unwrap()),
            u64::from_le_bytes(temp_buffer1[8..16].try_into().unwrap()),
            101,
            0,
        )?;

        // 读取流动性数据
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析池数据
        let pool_a_lo = pool_a & 0xFFFFFFFF;
        let pool_a_hi = pool_a >> 32;
        let pool_b_lo = pool_b & 0xFFFFFFFF;
        let pool_b_hi = pool_b >> 32;

        // 计算交叉乘积
        let mut cross_mul1 = pool_a_hi * pool_b_lo;
        let cross_mul2 = pool_a_lo * pool_b_lo >> 32;
        let cross_sum = cross_mul1 + cross_mul2;

        // 计算更多的交叉乘积
        let cross_mul3 = pool_a_hi * pool_b_hi;
        let cross_hi = cross_sum >> 32;
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积和
        let low_mul = pool_a_lo * pool_b_hi;
        let cross_lo = cross_sum & 0xFFFFFFFF;
        let low_sum = low_mul + (cross_lo << 32 >> 32);
        let low_shift = low_sum >> 32;
        let total_sum = cross_sum2 + low_shift;

        // 获取调整后的数量
        let adjusted_amount = u64::from_le_bytes(temp_buffer2[0..8].try_into().unwrap());

        // 计算池乘积和调整后的数量
        let pool_product = pool_a * pool_b;
        let amount_with_b = pool_b + adjusted_amount;

        // 检查总和是否为0
        if total_sum == 0 {
            // 简单除法
            let result = pool_product / amount_with_b;

            // 执行最终调整 (xor64 r4, -1; add64 r0, r4)
            let inverted = !result;
            let final_result = pool_a + inverted;

            // 最后的调整计算 (div64 r1, 100; sub64 r0, r1)
            let adjustment = final_result / 100;
            return Ok(final_result - adjustment);
        }

        // 大数除法计算
        let division_result = complex_division(pool_product, amount_with_b, total_sum)?;

        // 执行最终调整
        let inverted = !division_result;
        let final_result = pool_a + inverted;

        // 最后的调整计算
        let adjustment = final_result / 100;
        Ok(final_result - adjustment)
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
        let cross_mul1 = pool_b_hi * pool_a_lo;
        let cross_mul2 = pool_b_lo * pool_a_lo >> 32;
        let cross_sum = cross_mul1 + cross_mul2;

        // 计算更多的交叉乘积
        let cross_mul3 = pool_b_hi * pool_a_hi;
        let cross_hi = cross_sum >> 32;
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积和
        let low_mul = pool_b_lo * pool_a_hi;
        let cross_lo = cross_sum & 0xFFFFFFFF;
        let low_sum = low_mul + (cross_lo << 32 >> 32);
        let low_shift = low_sum >> 32;
        let total_sum = cross_sum2 + low_shift;

        // 计算池乘积和调整后的数量
        let pool_product = pool_a * pool_b;
        let amount_with_a = pool_a + quote_amount;

        // 检查总和是否为0
        if total_sum == 0 {
            // 直接执行简单的除法
            return Ok(pool_product / amount_with_a);
        }

        // 执行复杂的大数除法
        let result = complex_division(pool_product, amount_with_a, total_sum)?;

        // 执行最终的取反操作
        Ok(!result + pool_a)
    }
}
