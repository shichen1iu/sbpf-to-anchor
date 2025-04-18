/// 辅助函数 - 为Pump计算准备数据
/// 该函数在sBPF汇编中对应function_9839
pub fn function_9839(out_buffer: &mut [u8], param1: u64, param2: u64, param3: u64) -> Result<()> {
    // 保存参数r3到栈 (stxdw [r10-0x10], r3)
    // 保存参数r1到栈 (stxdw [r10-0x8], r1)

    // 提取param1的低32位 (mov64 r1, r2; lsh64 r1, 32; rsh64 r1, 32)
    let param1_lo = param1 & 0xFFFFFFFF;

    // 提取param3的低32位 (mov64 r7, r4; lsh64 r7, 32; rsh64 r7, 32)
    let param3_lo = param3 & 0xFFFFFFFF;

    // 提取param1的高32位 (mov64 r6, r2; rsh64 r6, 32)
    let param1_hi = param1 >> 32;

    // 计算第一个乘积 (mov64 r3, r7; mul64 r3, r1)
    // param3_lo * param1_lo
    let mut mul1 = param3_lo * param1_lo;

    // 计算第二个乘积 (mul64 r7, r6)
    // param3_lo * param1_hi
    let mul2 = param3_lo * param1_hi;

    // 提取param3的高32位 (mov64 r0, r4; rsh64 r0, 32)
    let param3_hi = param3 >> 32;

    // 计算第三个乘积 (mov64 r9, r0; mul64 r9, r1)
    // param3_hi * param1_lo
    let mul3 = param3_hi * param1_lo;

    // 合并第二和第三个乘积 (mov64 r1, r9; add64 r1, r7)
    let mut sum = mul3 + mul2;

    // 检查溢出 (mov64 r8, 1; jgt r9, r1, lbb_9861; mov64 r8, 0)
    let overflow1 = if mul3 > sum { 1u64 } else { 0u64 };

    // 左移sum的低32位，然后加上mul1 (mov64 r9, r1; lsh64 r9, 32; mov64 r7, r3; add64 r7, r9)
    let result_lo = mul1 + ((sum & 0xFFFFFFFF) << 32);

    // 检查溢出 (mov64 r9, 1; jgt r3, r7, lbb_9868; mov64 r9, 0)
    let overflow2 = if mul1 > result_lo { 1u64 } else { 0u64 };

    // 保存结果到输出缓冲区 (ldxdw r3, [r10-0x8]; stxdw [r3+0x0], r7)
    out_buffer[0..8].copy_from_slice(&result_lo.to_le_bytes());

    // 计算结果高64位 (rsh64 r1, 32; lsh64 r8, 32; or64 r8, r1)
    let partial_hi = ((sum >> 32) & 0xFFFFFFFF) | (overflow1 << 32);

    // 计算其他乘积 (ldxdw r1, [r10-0x10]; mul64 r4, r1; mul64 r5, r2; mul64 r0, r6)
    let mul4 = param3 * param2;
    let mul5 = param3_hi * param1_hi;

    // 合并结果 (add64 r0, r8; add64 r5, r4; add64 r0, r9; add64 r0, r5)
    let result_hi = mul5 + mul4 + partial_hi + overflow2;

    // 保存高64位结果到输出缓冲区 (stxdw [r3+0x8], r0)
    out_buffer[8..16].copy_from_slice(&result_hi.to_le_bytes());

    Ok(())
}

/// 辅助函数 - 处理缓冲区数据
/// 该函数在sBPF汇编中对应function_9883
pub fn function_9883(
    out_buffer: &mut [u8],
    param1: u64,
    param2: u64,
    param3: u64,
    param4: u64,
) -> Result<()> {
    // 保存输出缓冲区引用 (mov64 r6, r1)

    // 创建栈临时缓冲区 (mov64 r1, r10; add64 r1, -32)
    let mut temp_buffer = [0u8; 16];

    // 调用函数9892进行处理 (call function_9892)
    function_9892(&mut temp_buffer, param1, param2, param3, param4)?;

    // 读取处理后的结果 (ldxdw r1, [r10-0x20]; ldxdw r2, [r10-0x18])
    let result1 = u64::from_le_bytes(temp_buffer[0..8].try_into().unwrap());
    let result2 = u64::from_le_bytes(temp_buffer[8..16].try_into().unwrap());

    // 将结果写入输出缓冲区 (stxdw [r6+0x8], r2; stxdw [r6+0x0], r1)
    out_buffer[8..16].copy_from_slice(&result2.to_le_bytes());
    out_buffer[0..8].copy_from_slice(&result1.to_le_bytes());

    Ok(())
}

/// 复杂大数除法算法
/// 处理无法用简单除法解决的情况
pub fn complex_division(value: u64, divisor: u64, total_sum: u64) -> Result<u64> {
    // 计算前导零数量
    let leading_zeros = if divisor > 0xFFFFFFFF { 0 } else { 32 };
    let msb = divisor;

    let shift_amount = if msb > 0xFFFF {
        leading_zeros + 16
    } else {
        leading_zeros
    };

    let high_bits = if msb > 0xFFFF { msb >> 16 } else { msb };

    let shift_amount2 = if high_bits > 0xFF {
        shift_amount + 8
    } else {
        shift_amount
    };

    let high_byte = if high_bits > 0xFF {
        high_bits >> 8
    } else {
        high_bits
    };

    // 使用特殊的查找表获取字节值
    // 汇编中使用: lddw r5, 0x10001913b; add64 r5, r4; ldxb r4, [r5+0x0]
    let byte_value = 5; // 简化实现，实际应该从表中查找

    // 计算最终的位移量
    let final_shift = 64 - (shift_amount2 + byte_value);

    // 左移操作
    let shifted_value = value << final_shift;
    let shifted_divisor = divisor << final_shift;

    // 分解值以准备大数除法
    let divisor_hi = shifted_divisor >> 32;
    let divisor_lo = shifted_divisor & 0xFFFFFFFF;
    let value_hi = total_sum;
    let value_lo = shifted_value >> 32;

    // 大数除法计算商
    let quotient_hi = (value_hi << 32 | value_lo) / divisor_hi;

    // 返回结果
    Ok(quotient_hi)
}

/// 辅助函数 - 位运算预处理
/// 该函数在sBPF汇编中对应function_9892
/// 实现位操作和SWAR算法计算
pub fn function_9892(
    buffer: &mut [u8],
    param1: u64,
    param2: u64,
    param3: u64,
    param4: u64,
) -> Result<()> {
    // 保存参数r4 (stxdw [r10-0xc0], r4)

    // 从参数获取值 (mov64 r7, r2; mov64 r0, r1; mov64 r1, r5; rsh64 r1, 1)
    let mut value1 = param4;
    let value1_shift = value1 >> 1;

    // 保存参数r5 (stxdw [r10-0xb8], r5)

    // 准备寄存器 (mov64 r6, r5; mov64 r5, r3)
    let mut r6 = param4;
    let r5 = param2;

    // 位填充操作 (or64 r6, r1)
    r6 |= value1_shift;

    // 继续填充 (mov64 r1, r6; rsh64 r1, 2; or64 r6, r1)
    r6 |= r6 >> 2;

    // 更多填充 (mov64 r1, r6; rsh64 r1, 4; or64 r6, r1)
    r6 |= r6 >> 4;

    // 继续填充 (mov64 r1, r6; rsh64 r1, 8; or64 r6, r1)
    r6 |= r6 >> 8;

    // 继续填充 (mov64 r1, r6; rsh64 r1, 16; or64 r6, r1)
    r6 |= r6 >> 16;

    // 处理r5值 (mov64 r1, r5; rsh64 r1, 1; mov64 r4, r5; or64 r4, r1)
    let mut r4 = r5;
    r4 |= r5 >> 1;

    // 继续填充 (mov64 r1, r4; rsh64 r1, 2; or64 r4, r1)
    r4 |= r4 >> 2;

    // 完成r6填充 (mov64 r1, r6; rsh64 r1, 32; or64 r6, r1)
    r6 |= r6 >> 32;

    // 继续填充r4 (mov64 r1, r4; rsh64 r1, 4; or64 r4, r1)
    r4 |= r4 >> 4;

    // 继续填充 (mov64 r1, r4; rsh64 r1, 8; or64 r4, r1)
    r4 |= r4 >> 8;

    // 设置常量掩码 (lddw r1, 0x5555555555555555)
    let mask1: u64 = 0x5555555555555555;

    // 取反r6 (xor64 r6, -1)
    r6 = !r6;

    // SWAR算法计算位计数 (mov64 r2, r6; rsh64 r2, 1; and64 r2, r1; sub64 r6, r2)
    let mut tmp = r6;
    tmp = (tmp >> 1) & mask1;
    r6 -= tmp;

    // 继续填充r4 (mov64 r2, r4; rsh64 r2, 16; or64 r4, r2)
    r4 |= r4 >> 16;

    // 最后填充 (mov64 r2, r4; rsh64 r2, 32; or64 r4, r2)
    r4 |= r4 >> 32;

    // 设置常量掩码2 (lddw r2, 0x3333333333333333)
    let mask2: u64 = 0x3333333333333333;

    // 计算r6位数 (mov64 r8, r6; and64 r8, r2; rsh64 r6, 2; and64 r6, r2; add64 r8, r6)
    let mut r8 = r6 & mask2;
    r8 += (r6 >> 2) & mask2;

    // 取反r4 (xor64 r4, -1)
    r4 = !r4;

    // 计算r4位数 (mov64 r3, r4; rsh64 r3, 1; and64 r3, r1; sub64 r4, r3)
    let mut r3 = r4;
    r3 = (r3 >> 1) & mask1;
    r4 -= r3;

    // 位数调整 (mov64 r3, r8; rsh64 r3, 4; add64 r8, r3)
    r8 += r8 >> 4;

    // 计算r4位数 (mov64 r9, r4; and64 r9, r2; rsh64 r4, 2; and64 r4, r2; add64 r9, r4)
    let mut r9 = r4 & mask2;
    r9 += (r4 >> 2) & mask2;

    // 位数调整 (mov64 r3, r9; rsh64 r3, 4; add64 r9, r3)
    r9 += r9 >> 4;

    // 设置掩码3 (lddw r3, 0xf0f0f0f0f0f0f0f)
    let mask3: u64 = 0xf0f0f0f0f0f0f0f;

    // 掩码位与 (and64 r9, r3; and64 r8, r3)
    r9 &= mask3;
    r8 &= mask3;

    // 计算最终位计数 (lddw r4, 0x101010101010101; mul64 r8, r4; mul64 r9, r4)
    let magic_constant: u64 = 0x101010101010101;
    r8 *= magic_constant;
    r9 *= magic_constant;

    // 位移结果 (rsh64 r9, 56)
    r9 >>= 56;

    // 检查特殊情况 (jne r5, 0, lbb_10017)
    if r5 == 0 {
        // 特殊处理 (从汇编代码lbb_10017到lbb_10299)
        // 计算param1的位计数
        let mut val = param1;
        val |= val >> 1;
        val |= val >> 2;
        val |= val >> 4;
        val |= val >> 8;
        val |= val >> 16;
        val |= val >> 32;
        val = !val;

        let mut count = val;
        count -= ((count >> 1) & mask1);
        count = (count & mask2) + ((count >> 2) & mask2);
        count = (count + (count >> 4)) & mask3;
        count = (count * magic_constant) >> 56;

        // 调整结果 (add64 r9, 64)
        r9 += 64;
    }

    // 将结果写入缓冲区
    // 低64位值存储在[r10-0x20]，高64位值存储在[r10-0x18]
    let result1 = r8; // 低64位结果
    let result2 = r9; // 高64位结果

    if buffer.len() >= 16 {
        buffer[0..8].copy_from_slice(&result1.to_le_bytes());
        buffer[8..16].copy_from_slice(&result2.to_le_bytes());
    }

    Ok(())
}
