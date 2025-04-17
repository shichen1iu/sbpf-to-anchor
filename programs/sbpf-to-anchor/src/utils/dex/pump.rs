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

/// Pump DEX的流动性解析函数
/// 该函数从Pump池中解析流动性数据
///
/// # 参数说明
/// * `pool_state` - 池状态账户 (对应汇编中的 r1)
/// * `output_buffer` - 输出缓冲区 (对应汇编中的 r2)
pub fn pump_fun_parse_liquidity(
    pool_state: &AccountInfo,
    output_buffer: &mut [u8],
) -> Result<bool> {
    // 获取池状态的数据长度 (ldxdw r3, [r1+0x10])
    let pool_data_len = pool_state.data_len() as u64;

    // 检查数据长度是否至少为24字节 (mov64 r4, 24; jgt r4, r3)
    if 24 > pool_data_len {
        // 如果数据长度不足，设置结果 (mov64 r0, 1; jgt r3, 23; mov64 r0, 0)
        // 在这里我们先计算结果值，但等到函数结束时再返回
        return Ok(false);
    }

    // 获取池状态的数据指针 (ldxdw r1, [r1+0x18])
    let pool_data = pool_state.try_borrow_data()?;

    // 读取流动性数据 (ldxdw r4, [r1+0x8]; ldxdw r1, [r1+0x10])
    let liquidity_a = if pool_data.len() >= 16 {
        u64::from_le_bytes(pool_data[8..16].try_into().unwrap())
    } else {
        return Ok(false);
    };

    let liquidity_b = if pool_data.len() >= 24 {
        u64::from_le_bytes(pool_data[16..24].try_into().unwrap())
    } else {
        return Ok(false);
    };

    // 将数据写入输出缓冲区 (stxdw [r2+0x10], r1; stxdw [r2+0x8], r4)
    output_buffer[16..24].copy_from_slice(&liquidity_b.to_le_bytes());
    output_buffer[8..16].copy_from_slice(&liquidity_a.to_le_bytes());

    // 设置类型为1 (mov64 r1, 1; stxw [r2+0x0], r1)
    output_buffer[0..4].copy_from_slice(&1u32.to_le_bytes());

    // 返回结果 (mov64 r0, 1)
    Ok(true)
}

/// Pump DEX的K值计算函数
/// 该函数计算Pump池的K值（常数乘积）
///
/// # 参数说明
/// * `output_buffer` - 输出缓冲区 (对应汇编中的 r6/r1)
/// * `input_buffer` - 输入缓冲区 (对应汇编中的 r2)
pub fn pump_fun_k(output_buffer: &mut [u8], input_buffer: &[u8]) -> Result<()> {
    // 从输入缓冲区读取数据 (ldxdw r3, [r2+0x8]; ldxdw r4, [r2+0x0])
    let value_b = u64::from_le_bytes(input_buffer[8..16].try_into().unwrap());
    let value_a = u64::from_le_bytes(input_buffer[0..8].try_into().unwrap());

    // 创建临时缓冲区 (mov64 r1, r10; add64 r1, -16)
    let mut temp_buffer = [0u8; 16];

    // 调用函数9839处理数据 (call function_9839)
    // 注意参数顺序：r1=temp_buffer, r2=value_b, r3=0, r5=0
    function_9839(&mut temp_buffer, value_b, 0, 0)?;

    // 从临时缓冲区读取结果并写入输出缓冲区
    // (ldxdw r1, [r10-0x8]; stxdw [r6+0x8], r1; ldxdw r1, [r10-0x10]; stxdw [r6+0x0], r1)
    let result_b = u64::from_le_bytes(temp_buffer[8..16].try_into().unwrap());
    let result_a = u64::from_le_bytes(temp_buffer[0..8].try_into().unwrap());

    output_buffer[8..16].copy_from_slice(&result_b.to_le_bytes());
    output_buffer[0..8].copy_from_slice(&result_a.to_le_bytes());

    Ok(())
}

/// Pump DEX的价格计算函数
/// 该函数根据方向计算Pump池的价格
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r1)
/// * `direction` - 方向标志 (对应汇编中的 r2/r7)
pub fn pump_fun_price(liquidity_data: &[u8], direction: bool) -> Result<u64> {
    // 从流动性数据中读取池中的代币数量 (ldxdw r6, [r1+0x8]; ldxdw r9, [r1+0x0])
    let token_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());
    let token_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

    // 根据方向选择要处理的代币 (mov64 r1, r9; jne r7, 0, lbb_1291; mov64 r1, r6)
    let selected_token = if direction { token_a } else { token_b };

    // 调用价格处理函数 (call function_12023)
    // 这里我们假设function_12023是一个已存在的函数，在实际实现中需要替换为真实函数
    let price_result = process_price(selected_token)?;

    // 根据方向可能需要进一步处理结果 (jne r7, 0, lbb_1295; mov64 r6, r9)
    let final_result = if !direction {
        token_a // 如果方向为false，使用token_a作为结果的一部分
    } else {
        token_b // 保持原值
    };

    // 这里汇编代码似乎没有完成，我们假设最终结果需要结合price_result和final_result
    // 由于没有完整的汇编代码，我们返回price_result作为最终结果
    Ok(price_result)
}

/// 处理价格的辅助函数
/// 这个函数在sBPF汇编中对应function_12023
/// todo
fn process_price(token_amount: u64) -> Result<u64> {
    // 这里应该实现真实的价格处理逻辑
    // 由于我们没有function_12023的具体实现，暂时返回token_amount作为示例
    Ok(token_amount)
}

/// Pump DEX的池有效性验证函数
/// 该函数检查流动性池是否有效
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r1)
pub fn pump_fun_is_valid(liquidity_data: &[u8]) -> Result<bool> {
    // 初始化返回值为false (mov64 r0, 0)
    let mut result = false;

    // 从流动性数据读取代币A数量 (ldxdw r6, [r1+0x0])
    let token_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

    // 检查代币A数量是否大于1001 (mov64 r2, 1001; jgt r2, r6, lbb_1327)
    if 1001 > token_a {
        return Ok(false);
    }

    // 读取代币B数量 (ldxdw r1, [r1+0x8])
    let token_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

    // 检查代币B数量是否大于1001 (jgt r2, r1, lbb_1327)
    if 1001 > token_b {
        return Ok(false);
    }

    // 处理代币B数量 (call function_12023)
    let processed_b = process_price(token_b)?;

    // 保存处理结果 (mov64 r7, r0)
    let saved_b = processed_b;

    // 处理代币A数量 (mov64 r1, r6; call function_12023)
    let processed_a = process_price(token_a)?;

    // 计算比率或关系 (mov64 r1, r7; mov64 r2, r0; call function_12129)
    let ratio = calculate_ratio(saved_b, processed_a)?;

    // 比较结果与第一个常数 (mov64 r1, r0; lddw r2, 0x42d6bcc41e900000; call function_11552)
    let compare1 = compare_values(ratio, 0x42d6bcc41e900000)?;

    // 比较结果与第二个常数 (mov64 r1, r0; lddw r2, 0x4253ca6512000000; call function_11519)
    let compare2 = compare_values(compare1, 0x4253ca6512000000)?;

    // 设置返回结果 (mov64 r0, 1; mov64 r2, 0; jsgt r2, r1, lbb_1327; mov64 r0, 0)
    if 0 > compare2 as i64 {
        result = false;
    } else {
        result = true;
    }

    // 确保返回值是布尔值 (and64 r0, 1)
    Ok(result)
}

/// 计算比率的辅助函数
/// 这个函数在sBPF汇编中对应function_12129
/// todo
fn calculate_ratio(value1: u64, value2: u64) -> Result<u64> {
    // 由于没有function_12129的具体实现，这里提供一个简单示例
    // 例如返回两个值的比率或关系
    Ok(value1.wrapping_mul(value2))
}

/// 比较值的辅助函数
/// 这个函数在sBPF汇编中对应function_11552和function_11519
fn compare_values(value: u64, constant: u64) -> Result<u64> {
    // 由于没有function_11552和function_11519的具体实现
    // 这里提供一个简单的比较逻辑作为示例
    if value > constant {
        Ok(1)
    } else if value < constant {
        Ok(-1i64 as u64)
    } else {
        Ok(0)
    }
}

/// Pump DEX的获取流动性函数
/// 该函数根据当前池状态和报价金额计算流动性
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r7/r1)
/// * `quote_amount` - 报价金额 (对应汇编中的 r8/r2)
/// * `swap_direction` - 交换方向 (对应汇编中的 r3)
/// * `output_buffer` - 输出缓冲区 (对应汇编中的 r9/r4)
pub fn pump_fun_get_liquidity(
    liquidity_data: &[u8],
    quote_amount: u64,
    swap_direction: bool,
    output_buffer: &mut [u8],
) -> Result<()> {
    // 初始化寄存器 (mov64 r9, r4; mov64 r8, r2; mov64 r7, r1)
    if swap_direction {
        // 处理 B->A 方向的交换 (jeq r3, 0, lbb_1662)

        // 读取流动性数据 (ldxdw r1, [r7+0x0])
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

        // 解析 A 池的高低位数据 (mov64 r0, r1; lsh64 r0, 32; rsh64 r0, 32)
        let pool_a_lo = pool_a & 0xFFFFFFFF;

        // 读取第二部分流动性数据 (ldxdw r3, [r7+0x8])
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析 B 池的高低位数据 (mov64 r5, r3; lsh64 r5, 32; rsh64 r5, 32)
        let pool_b_lo = pool_b & 0xFFFFFFFF;

        // 获取 A 池的高32位 (mov64 r4, r1; rsh64 r4, 32)
        let pool_a_hi = pool_a >> 32;

        // 计算交叉乘积 (mov64 r6, r5; mul64 r6, r4)
        let mut cross_mul1 = pool_b_lo * pool_a_hi;

        // 计算低位乘积并右移32位 (mul64 r5, r0; rsh64 r5, 32)
        let cross_mul2 = (pool_b_lo * pool_a_lo) >> 32;

        // 合并交叉乘积 (add64 r5, r6)
        let cross_sum = cross_mul1 + cross_mul2;

        // 获取 B 池的高32位 (mov64 r6, r3; rsh64 r6, 32)
        let pool_b_hi = pool_b >> 32;

        // 计算高位的交叉乘积 (mov64 r7, r6; mul64 r7, r4)
        let cross_mul3 = pool_b_hi * pool_a_hi;

        // 获取交叉乘积的高32位 (mov64 r2, r5; rsh64 r2, 32)
        let cross_hi = cross_sum >> 32;

        // 合并高位结果 (add64 r2, r7)
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算池乘积 (mov64 r7, r3; mul64 r7, r1)
        let prod = pool_a * pool_b;

        // 计算最终数量 (add64 r1, r8)
        let amount_adjusted = pool_a + quote_amount;

        // 计算低位的乘积 (mul64 r6, r0)
        let low_mul = pool_b_hi * pool_a_lo;

        // 获取交叉乘积的低32位 (lsh64 r5, 32; rsh64 r5, 32)
        let cross_lo = cross_sum & 0xFFFFFFFF;

        // 合并低位结果 (add64 r5, r6)
        let low_sum = low_mul + cross_lo;

        // 获取低位溢出 (rsh64 r5, 32)
        let low_shift = low_sum >> 32;

        // 合并最终结果 (add64 r2, r5)
        let total_sum = cross_sum2 + low_shift;

        // 如果总和为0，直接执行简单的除法 (jne r2, 0)
        if total_sum == 0 {
            // 简单除法 (div64 r7, r1)
            let result = prod / amount_adjusted;

            // 存储结果到输出缓冲区 (stxdw [r9+0x8], r3; stxdw [r9+0x0], r1)
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&result.to_le_bytes());

            return Ok(());
        }

        // 以下是复杂大数除法的实现
        // 检查被除数是否大于32位 (mov64 r3, 1; lddw r0, 0xffffffff; jgt r1, r0)
        let mut shift = 0;
        if amount_adjusted > 0xFFFFFFFF {
            shift = 32;
        }

        // 确定被除数的高32位或完整数值 (mov64 r5, r1; rsh64 r5, 32; jgt r1, r0)
        let mut numerator_high = amount_adjusted;
        if amount_adjusted > 0xFFFFFFFF {
            numerator_high = amount_adjusted >> 32;
        }

        // 计算需要的移位 - 检查是否大于16位值 (lsh64 r3, 5; mov64 r0, r3; or64 r0, 16; jgt r5, 65535)
        if numerator_high > 0xFFFF {
            shift |= 16;
        }

        // 根据前一个检查调整numerator_high (mov64 r6, r5; rsh64 r6, 16; jgt r5, 65535)
        let mut temp_high = numerator_high;
        if numerator_high > 0xFFFF {
            temp_high = numerator_high >> 16;
        }

        // 检查是否大于8位值 (mov64 r3, r0; or64 r3, 8; jgt r6, 255)
        if temp_high > 0xFF {
            shift |= 8;
        }

        // 根据前一个检查再次调整temp_high (mov64 r5, r6; rsh64 r5, 8; jgt r6, 255)
        let mut lookup_index = temp_high;
        if temp_high > 0xFF {
            lookup_index = temp_high >> 8;
        }

        // 保存寄存器状态 (stxdw [r10-0x38], r9)
        // 在Rust中不需要显式保存

        // 查表获取最后的移位 (lddw r0, 0x10001913b; add64 r0, r5; ldxb r5, [r0+0x0])
        let final_shift = shift + lookup_bits(lookup_index as u8);

        // 计算补充移位 (mov64 r5, 64; sub64 r5, r3)
        let complement_shift = 64 - final_shift;

        // 准备被除数和除数 (rsh64 r0, r3; lsh64 r2, r5; or64 r2, r0)
        let dividend_high = (prod >> final_shift) | (0u64 << complement_shift);

        // 左移被除数 (mov64 r6, r1; lsh64 r6, r5)
        let dividend_shifted = amount_adjusted << complement_shift;

        // 左移乘积 (lsh64 r7, r5)
        let prod_shifted = prod << complement_shift;

        // 分解被除数的高低位
        let dividend_shifted_high = dividend_shifted >> 32;
        let prod_shifted_high = prod_shifted >> 32;
        let prod_shifted_low = prod_shifted & 0xFFFFFFFF;

        // 第一步除法 (mov64 r5, r2; div64 r5, r0)
        let mut quotient_high = dividend_high / dividend_shifted_high;

        // 乘法检查 (mov64 r3, r0; mul64 r3, r5; mov64 r4, r2; sub64 r4, r3)
        let mut remainder = dividend_high - (dividend_shifted_high * quotient_high);

        // 循环调整商和余数
        loop {
            if quotient_high <= 0xFFFFFFFF {
                let product = quotient_high * prod_shifted_low;
                let compare_value = (remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r4, r0; add64 r5, -1)
            remainder += dividend_shifted_high;
            quotient_high -= 1;

            // 防止无限循环 (jgt r8, r4)
            if 0x100000000 > remainder {
                continue;
            }
            break;
        }

        // 计算下一步的被除数 (mul64 r3, r4; lsh64 r2, 32; or64 r2, r4; sub64 r2, r3)
        let next_dividend =
            ((dividend_high << 32) | prod_shifted_high) - (quotient_high * dividend_shifted);

        // 计算低位商 (mov64 r3, r2; div64 r3, r0)
        let mut quotient_low = next_dividend / dividend_shifted_high;

        // 计算新的余数 (mov64 r4, r3; mul64 r4, r0; sub64 r2, r4)
        let mut low_remainder = next_dividend - (quotient_low * dividend_shifted_high);

        // 调整低位商
        loop {
            if quotient_low <= 0xFFFFFFFF {
                let product = quotient_low * prod_shifted_low;
                let compare_value = (low_remainder << 32) | prod_shifted_low;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r2, r0; add64 r3, -1)
            low_remainder += dividend_shifted_high;
            quotient_low -= 1;

            // 防止无限循环 (jgt r8, r2)
            if 0x100000000 > low_remainder {
                continue;
            }
            break;
        }

        // 合并最终结果 (lsh64 r5, 32; add64 r3, r5)
        let final_result = (quotient_high << 32) | quotient_low;

        // 存储结果到输出缓冲区 (stxdw [r9+0x8], r3; stxdw [r9+0x0], r1)
        output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
        output_buffer[0..8].copy_from_slice(&final_result.to_le_bytes());
    } else {
        // 处理 A->B 方向的交换

        // 创建临时缓冲区 (mov64 r1, r10; add64 r1, -16)
        let mut temp_buffer1 = [0u8; 16];

        // 调用辅助函数9839处理数据 (call function_9839)
        // r2=quote_amount, r3=0, r4=100, r5=0
        function_9839(&mut temp_buffer1, quote_amount, 0, 100)?;

        // 创建第二个临时缓冲区 (mov64 r1, r10; add64 r1, -32)
        let mut temp_buffer2 = [0u8; 16];

        // 从第一个临时缓冲区读取结果 (ldxdw r2, [r10-0x10]; ldxdw r3, [r10-0x8])
        let value1 = u64::from_le_bytes(temp_buffer1[0..8].try_into().unwrap());
        let value2 = u64::from_le_bytes(temp_buffer1[8..16].try_into().unwrap());

        // 调用辅助函数9883处理数据 (call function_9883)
        // r2=value1, r3=value2, r4=101, r5=0
        function_9883(&mut temp_buffer2, value1, value2, 101, 0)?;

        // 读取流动性数据 (ldxdw r8, [r7+0x0])
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

        // 解析 A 池的高低位数据 (mov64 r6, r8; lsh64 r6, 32; rsh64 r6, 32)
        let pool_a_lo = pool_a & 0xFFFFFFFF;

        // 读取第二部分流动性数据 (ldxdw r4, [r7+0x8])
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析 B 池的高低位数据 (mov64 r3, r4; lsh64 r3, 32; rsh64 r3, 32)
        let pool_b_lo = pool_b & 0xFFFFFFFF;

        // 获取 A 池的高32位 (mov64 r5, r8; rsh64 r5, 32)
        let pool_a_hi = pool_a >> 32;

        // 计算交叉乘积 (mov64 r0, r5; mul64 r0, r3)
        let mut cross_mul1 = pool_a_hi * pool_b_lo;

        // 计算低位乘积并右移32位 (mov64 r2, r6; mul64 r2, r3; rsh64 r2, 32)
        let cross_mul2 = (pool_a_lo * pool_b_lo) >> 32;

        // 合并交叉乘积 (add64 r2, r0)
        let cross_sum = cross_mul1 + cross_mul2;

        // 获取 B 池的高32位 (mov64 r0, r4; rsh64 r0, 32)
        let pool_b_hi = pool_b >> 32;

        // 计算更多的交叉乘积 (mul64 r5, r0)
        let cross_mul3 = pool_a_hi * pool_b_hi;

        // 获取交叉乘积的高32位 (mov64 r1, r2; rsh64 r1, 32)
        let cross_hi = cross_sum >> 32;

        // 合并高位结果 (add64 r1, r5)
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积 (mul64 r6, r0)
        let low_mul = pool_a_lo * pool_b_hi;

        // 获取交叉乘积的低32位 (lsh64 r2, 32; rsh64 r2, 32)
        let cross_lo = cross_sum & 0xFFFFFFFF;

        // 合并低位结果 (add64 r2, r6)
        let low_sum = low_mul + cross_lo;

        // 获取低位溢出 (rsh64 r2, 32)
        let low_shift = low_sum >> 32;

        // 合并最终结果 (add64 r1, r2)
        let total_sum = cross_sum2 + low_shift;

        // 从第二个临时缓冲区读取调整后的金额 (ldxdw r2, [r10-0x20])
        let adjusted_value = u64::from_le_bytes(temp_buffer2[0..8].try_into().unwrap());

        // 计算池乘积和调整后的数量 (mov64 r6, r8; mul64 r6, r4; add64 r4, r2)
        let pool_product = pool_a * pool_b;
        let amount_adjusted = pool_b + adjusted_value;

        // 如果总和为0，执行简单除法 (jne r1, 0, lbb_2134)
        if total_sum == 0 {
            // 简单除法 (div64 r6, r4)
            let result = pool_product / amount_adjusted;

            // 存储结果到输出缓冲区 (stxdw [r9+0x8], r4; stxdw [r9+0x0], r8)
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&result.to_le_bytes());

            return Ok(());
        }

        // 以下是复杂大数除法的实现 - 与B->A方向类似的逻辑
        // 检查被除数是否大于32位 (mov64 r3, 1; lddw r5, 0xffffffff; jgt r4, r5)
        let mut shift = 0;
        if amount_adjusted > 0xFFFFFFFF {
            shift = 32;
        }

        // 确定被除数的高32位或完整数值 (mov64 r2, r4; rsh64 r2, 32; jgt r4, r5)
        let mut numerator_high = amount_adjusted;
        if amount_adjusted > 0xFFFFFFFF {
            numerator_high = amount_adjusted >> 32;
        }

        // 计算需要的移位 - 检查是否大于16位值 (lsh64 r3, 5; mov64 r5, r3; or64 r5, 16)
        if numerator_high > 0xFFFF {
            shift |= 16;
        }

        // 根据前一个检查调整numerator_high (mov64 r0, r2; rsh64 r0, 16)
        let mut temp_high = numerator_high;
        if numerator_high > 0xFFFF {
            temp_high = numerator_high >> 16;
        }

        // 检查是否大于8位值 (mov64 r3, r5; or64 r3, 8)
        if temp_high > 0xFF {
            shift |= 8;
        }

        // 根据前一个检查再次调整temp_high (mov64 r2, r0; rsh64 r2, 8)
        let mut lookup_index = temp_high;
        if temp_high > 0xFF {
            lookup_index = temp_high >> 8;
        }

        // 查表获取最后的移位 (lddw r5, 0x10001913b; add64 r5, r2; ldxb r2, [r5+0x0])
        let final_shift = shift + lookup_bits(lookup_index as u8);

        // 计算补充移位 (mov64 r2, 64; sub64 r2, r3)
        let complement_shift = 64 - final_shift;

        // 准备被除数和除数 (mov64 r5, r6; rsh64 r5, r3; lsh64 r1, r2; or64 r1, r5)
        let dividend_high = (pool_product >> final_shift) | (0u64 << complement_shift);

        // 左移被除数 (mov64 r9, r4; lsh64 r9, r2)
        let dividend_shifted = amount_adjusted << complement_shift;

        // 左移乘积 (lsh64 r6, r2)
        let prod_shifted = pool_product << complement_shift;

        // 分解被除数的高低位
        let dividend_shifted_high = dividend_shifted >> 32;
        let prod_shifted_high = prod_shifted >> 32;
        let prod_shifted_low = prod_shifted & 0xFFFFFFFF;

        // 第一步除法 (mov64 r5, r1; div64 r5, r7)
        let mut quotient_high = dividend_high / dividend_shifted_high;

        // 乘法检查 (mov64 r2, r7; mul64 r2, r5; mov64 r3, r1; sub64 r3, r2)
        let mut remainder = dividend_high - (dividend_shifted_high * quotient_high);

        // 循环调整商和余数
        loop {
            if quotient_high <= 0xFFFFFFFF {
                let product = quotient_high * prod_shifted_low;
                let compare_value = (remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r3, r7; add64 r5, -1)
            remainder += dividend_shifted_high;
            quotient_high -= 1;

            // 防止无限循环 (jgt r0, r3)
            if 0x100000000 > remainder {
                continue;
            }
            break;
        }

        // 计算下一步的被除数 (mul64 r2, r9; lsh64 r1, 32; or64 r1, r8; sub64 r1, r2)
        let next_dividend =
            ((dividend_high << 32) | prod_shifted_high) - (quotient_high * dividend_shifted);

        // 计算低位商 (mov64 r8, r1; div64 r8, r7)
        let mut quotient_low = next_dividend / dividend_shifted_high;

        // 计算新的余数 (mov64 r2, r8; mul64 r2, r7; sub64 r1, r2)
        let mut low_remainder = next_dividend - (quotient_low * dividend_shifted_high);

        // 调整低位商
        loop {
            if quotient_low <= 0xFFFFFFFF {
                let product = quotient_low * prod_shifted_low;
                let compare_value = (low_remainder << 32) | prod_shifted_low;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r1, r7; add64 r8, -1)
            low_remainder += dividend_shifted_high;
            quotient_low -= 1;

            // 防止无限循环 (jgt r0, r1)
            if 0x100000000 > low_remainder {
                continue;
            }
            break;
        }

        // 合并最终结果 (lsh64 r5, 32; add64 r8, r5)
        let final_result = (quotient_high << 32) | quotient_low;

        // 执行最终调整 (mov64 r1, r8; xor64 r1, -1; ldxdw r0, [r10-0x38]; add64 r0, r1)
        let inverted = !final_result;
        let result = pool_a + inverted;

        // 存储结果到输出缓冲区 (stxdw [r9+0x8], r4; stxdw [r9+0x0], r8)
        output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
        output_buffer[0..8].copy_from_slice(&final_result.to_le_bytes());
    }

    Ok(())
}

/// 用于查找位计数的辅助函数
/// 这在sBPF汇编中通常通过查表实现 (对应汇编中 0x10001913b 的表查找)
fn lookup_bits(value: u8) -> u8 {
    // 这是一个典型的计算前导零的函数
    // 在汇编中通常是一个硬编码的查找表
    if value == 0 {
        return 0;
    }

    let mut count = 0;
    let mut v = value;
    while v > 0 {
        count += 1;
        v >>= 1;
    }

    count
}

/// Pump DEX的报价和流动性计算函数
/// 该函数同时计算报价和流动性
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r7/r1)
/// * `quote_amount` - 报价金额 (对应汇编中的 r5/r2)
/// * `swap_direction` - 交换方向 (对应汇编中的 r3)
/// * `output_buffer` - 输出缓冲区 (对应汇编中的 r9/r4)
pub fn pump_fun_get_quote_and_liquidity(
    liquidity_data: &[u8],
    quote_amount: u64,
    swap_direction: bool,
    output_buffer: &mut [u8],
) -> Result<()> {
    // 初始化寄存器 (mov64 r9, r4; mov64 r5, r2; mov64 r7, r1)
    if swap_direction {
        // 处理 B->A 方向的交换 (jeq r3, 0, lbb_1967)

        // 读取流动性数据 (ldxdw r8, [r7+0x0])
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

        // 解析 A 池的高低位数据 (mov64 r4, r8; lsh64 r4, 32; rsh64 r4, 32)
        let pool_a_lo = pool_a & 0xFFFFFFFF;

        // 读取第二部分流动性数据 (ldxdw r0, [r7+0x8])
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析 B 池的高低位数据 (mov64 r3, r0; lsh64 r3, 32; rsh64 r3, 32)
        let pool_b_lo = pool_b & 0xFFFFFFFF;

        // 获取 A 池的高32位 (mov64 r2, r8; rsh64 r2, 32)
        let pool_a_hi = pool_a >> 32;

        // 计算交叉乘积 (mov64 r1, r3; mul64 r1, r2)
        let mut cross_mul1 = pool_b_lo * pool_a_hi;

        // 计算低位乘积并右移32位 (mul64 r3, r4; rsh64 r3, 32)
        let cross_mul2 = (pool_b_lo * pool_a_lo) >> 32;

        // 合并交叉乘积 (add64 r3, r1)
        let cross_sum = cross_mul1 + cross_mul2;

        // 获取 B 池的高32位 (mov64 r1, r0; rsh64 r1, 32)
        let pool_b_hi = pool_b >> 32;

        // 计算更多的交叉乘积 (mov64 r7, r1; mul64 r7, r2)
        let cross_mul3 = pool_b_hi * pool_a_hi;

        // 获取交叉乘积的高32位 (mov64 r6, r3; rsh64 r6, 32)
        let cross_hi = cross_sum >> 32;

        // 合并高位结果 (add64 r6, r7)
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积 (mul64 r1, r4)
        let low_mul = pool_b_hi * pool_a_lo;

        // 获取交叉乘积的低32位 (lsh64 r3, 32; rsh64 r3, 32)
        let cross_lo = cross_sum & 0xFFFFFFFF;

        // 合并低位结果 (add64 r3, r1)
        let low_sum = low_mul + cross_lo;

        // 获取低位溢出 (rsh64 r3, 32)
        let low_shift = low_sum >> 32;

        // 合并最终结果 (add64 r6, r3)
        let total_sum = cross_sum2 + low_shift;

        // 计算池乘积 (mov64 r4, r0; mul64 r4, r8)
        let pool_product = pool_b * pool_a;

        // 计算调整后的总量 (add64 r8, r5)
        let amount_adjusted = pool_a + quote_amount;

        // 如果总和为0，执行简单除法 (jne r6, 0, lbb_2018)
        if total_sum == 0 {
            // 简单除法 (div64 r4, r8)
            let result = pool_product / amount_adjusted;

            // 执行最终调整 (mov64 r2, r4; xor64 r2, -1; add64 r0, r2)
            let inverted = !result;
            let final_result = pool_b + inverted;

            // 最后的调整计算 (mov64 r2, r0; div64 r2, 100; sub64 r0, r2)
            let adjustment = final_result / 100;
            let adjusted_result = final_result - adjustment;

            // 存储结果到输出缓冲区 (stxdw [r9+0x8], r4; stxdw [r9+0x0], r8)
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&result.to_le_bytes());
        } else {
            // 以下是复杂大数除法的实现
            // 检查被除数是否大于32位 (mov64 r3, 1; lddw r5, 0xffffffff; jgt r4, r5)
            let mut shift = 0;
            if amount_adjusted > 0xFFFFFFFF {
                shift = 32;
            }

            // 确定被除数的高32位或完整数值 (mov64 r2, r4; rsh64 r2, 32; jgt r4, r5)
            let mut numerator_high = amount_adjusted;
            if amount_adjusted > 0xFFFFFFFF {
                numerator_high = amount_adjusted >> 32;
            }

            // 计算需要的移位 - 检查是否大于16位值 (lsh64 r3, 5; mov64 r5, r3; or64 r5, 16)
            if numerator_high > 0xFFFF {
                shift |= 16;
            }

            // 根据前一个检查调整numerator_high (mov64 r0, r2; rsh64 r0, 16)
            let mut temp_high = numerator_high;
            if numerator_high > 0xFFFF {
                temp_high = numerator_high >> 16;
            }

            // 检查是否大于8位值 (mov64 r3, r5; or64 r3, 8)
            if temp_high > 0xFF {
                shift |= 8;
            }

            // 根据前一个检查再次调整temp_high (mov64 r2, r0; rsh64 r2, 8)
            let mut lookup_index = temp_high;
            if temp_high > 0xFF {
                lookup_index = temp_high >> 8;
            }

            // 查表获取最后的移位 (lddw r5, 0x10001913b; add64 r5, r2; ldxb r2, [r5+0x0])
            let final_shift = shift + lookup_bits(lookup_index as u8);

            // 计算补充移位 (mov64 r2, 64; sub64 r2, r3)
            let complement_shift = 64 - final_shift;

            // 准备被除数和除数 (mov64 r5, r6; rsh64 r5, r3; lsh64 r1, r2; or64 r1, r5)
            let dividend_high = (pool_product >> final_shift) | (0u64 << complement_shift);

            // 左移被除数 (mov64 r9, r4; lsh64 r9, r2)
            let dividend_shifted = amount_adjusted << complement_shift;

            // 左移乘积 (lsh64 r6, r2)
            let prod_shifted = pool_product << complement_shift;

            // 分解被除数的高低位
            let dividend_shifted_high = dividend_shifted >> 32;
            let prod_shifted_high = prod_shifted >> 32;
            let prod_shifted_low = prod_shifted & 0xFFFFFFFF;

            // 第一步除法 (mov64 r5, r1; div64 r5, r7)
            let mut quotient_high = dividend_high / dividend_shifted_high;

            // 乘法检查 (mov64 r2, r7; mul64 r2, r5; mov64 r3, r1; sub64 r3, r2)
            let mut remainder = dividend_high - (dividend_shifted_high * quotient_high);

            // 循环调整商和余数
            loop {
                if quotient_high <= 0xFFFFFFFF {
                    let product = quotient_high * prod_shifted_low;
                    let compare_value = (remainder << 32) | prod_shifted_high;

                    if compare_value >= product {
                        break;
                    }
                }

                // 调整商和余数 (add64 r3, r7; add64 r5, -1)
                remainder += dividend_shifted_high;
                quotient_high -= 1;

                // 防止无限循环 (jgt r0, r3)
                if 0x100000000 > remainder {
                    continue;
                }
                break;
            }

            // 计算下一步的被除数 (mul64 r2, r9; lsh64 r1, 32; or64 r1, r8; sub64 r1, r2)
            let next_dividend =
                ((dividend_high << 32) | prod_shifted_high) - (quotient_high * dividend_shifted);

            // 计算低位商 (mov64 r8, r1; div64 r8, r7)
            let mut quotient_low = next_dividend / dividend_shifted_high;

            // 计算新的余数 (mov64 r2, r8; mul64 r2, r7; sub64 r1, r2)
            let mut low_remainder = next_dividend - (quotient_low * dividend_shifted_high);

            // 调整低位商
            loop {
                if quotient_low <= 0xFFFFFFFF {
                    let product = quotient_low * prod_shifted_low;
                    let compare_value = (low_remainder << 32) | prod_shifted_low;

                    if compare_value >= product {
                        break;
                    }
                }

                // 调整商和余数 (add64 r1, r7; add64 r8, -1)
                low_remainder += dividend_shifted_high;
                quotient_low -= 1;

                // 防止无限循环 (jgt r0, r1)
                if 0x100000000 > low_remainder {
                    continue;
                }
                break;
            }

            // 合并最终结果 (lsh64 r5, 32; add64 r8, r5)
            let final_result = (quotient_high << 32) | quotient_low;

            // 执行最终调整 (mov64 r1, r8; xor64 r1, -1; ldxdw r0, [r10-0x38]; add64 r0, r1)
            let inverted = !final_result;
            let result = pool_a + inverted;

            // 存储结果到输出缓冲区 (stxdw [r9+0x8], r4; stxdw [r9+0x0], r8)
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&final_result.to_le_bytes());
        }
    } else {
        // 处理 A->B 方向的交换

        // 创建临时缓冲区 (mov64 r1, r10; add64 r1, -16)
        let mut temp_buffer1 = [0u8; 16];

        // 调用辅助函数9839处理数据 (call function_9839)
        // r2=quote_amount, r3=0, r4=100, r5=0
        function_9839(&mut temp_buffer1, quote_amount, 0, 100)?;

        // 创建第二个临时缓冲区 (mov64 r1, r10; add64 r1, -32)
        let mut temp_buffer2 = [0u8; 16];

        // 从第一个临时缓冲区读取结果 (ldxdw r2, [r10-0x10]; ldxdw r3, [r10-0x8])
        let value1 = u64::from_le_bytes(temp_buffer1[0..8].try_into().unwrap());
        let value2 = u64::from_le_bytes(temp_buffer1[8..16].try_into().unwrap());

        // 调用辅助函数9883处理数据 (call function_9883)
        // r2=value1, r3=value2, r4=101, r5=0
        function_9883(&mut temp_buffer2, value1, value2, 101, 0)?;

        // 读取流动性数据 (ldxdw r8, [r7+0x0])
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

        // 解析 A 池的高低位数据 (mov64 r6, r8; lsh64 r6, 32; rsh64 r6, 32)
        let pool_a_lo = pool_a & 0xFFFFFFFF;

        // 读取第二部分流动性数据 (ldxdw r4, [r7+0x8])
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析 B 池的高低位数据 (mov64 r3, r4; lsh64 r3, 32; rsh64 r3, 32)
        let pool_b_lo = pool_b & 0xFFFFFFFF;

        // 获取 A 池的高32位 (mov64 r5, r8; rsh64 r5, 32)
        let pool_a_hi = pool_a >> 32;

        // 计算交叉乘积 (mov64 r0, r5; mul64 r0, r3)
        let mut cross_mul1 = pool_a_hi * pool_b_lo;

        // 计算低位乘积并右移32位 (mov64 r2, r6; mul64 r2, r3; rsh64 r2, 32)
        let cross_mul2 = (pool_a_lo * pool_b_lo) >> 32;

        // 合并交叉乘积 (add64 r2, r0)
        let cross_sum = cross_mul1 + cross_mul2;

        // 获取 B 池的高32位 (mov64 r0, r4; rsh64 r0, 32)
        let pool_b_hi = pool_b >> 32;

        // 计算更多的交叉乘积 (mul64 r5, r0)
        let cross_mul3 = pool_a_hi * pool_b_hi;

        // 获取交叉乘积的高32位 (mov64 r1, r2; rsh64 r1, 32)
        let cross_hi = cross_sum >> 32;

        // 合并高位结果 (add64 r1, r5)
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积 (mul64 r6, r0)
        let low_mul = pool_a_lo * pool_b_hi;

        // 获取交叉乘积的低32位 (lsh64 r2, 32; rsh64 r2, 32)
        let cross_lo = cross_sum & 0xFFFFFFFF;

        // 合并低位结果 (add64 r2, r6)
        let low_sum = low_mul + cross_lo;

        // 获取低位溢出 (rsh64 r2, 32)
        let low_shift = low_sum >> 32;

        // 合并最终结果 (add64 r1, r2)
        let total_sum = cross_sum2 + low_shift;

        // 从第二个临时缓冲区读取调整后的金额 (ldxdw r2, [r10-0x20])
        let adjusted_value = u64::from_le_bytes(temp_buffer2[0..8].try_into().unwrap());

        // 计算池乘积和调整后的数量 (mov64 r6, r8; mul64 r6, r4; add64 r4, r2)
        let pool_product = pool_a * pool_b;
        let amount_adjusted = pool_b + adjusted_value;

        // 如果总和为0，执行简单除法 (jne r1, 0, lbb_2134)
        if total_sum == 0 {
            // 简单除法 (div64 r6, r4)
            let result = pool_product / amount_adjusted;

            // 存储结果到输出缓冲区 (stxdw [r9+0x8], r4; stxdw [r9+0x0], r8)
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&result.to_le_bytes());

            return Ok(());
        }

        // 以下是复杂大数除法的实现 - 与B->A方向类似的逻辑
        // 检查被除数是否大于32位 (mov64 r3, 1; lddw r5, 0xffffffff; jgt r4, r5)
        let mut shift = 0;
        if amount_adjusted > 0xFFFFFFFF {
            shift = 32;
        }

        // 确定被除数的高32位或完整数值 (mov64 r2, r4; rsh64 r2, 32; jgt r4, r5)
        let mut numerator_high = amount_adjusted;
        if amount_adjusted > 0xFFFFFFFF {
            numerator_high = amount_adjusted >> 32;
        }

        // 计算需要的移位 - 检查是否大于16位值 (lsh64 r3, 5; mov64 r5, r3; or64 r5, 16)
        if numerator_high > 0xFFFF {
            shift |= 16;
        }

        // 根据前一个检查调整numerator_high (mov64 r0, r2; rsh64 r0, 16)
        let mut temp_high = numerator_high;
        if numerator_high > 0xFFFF {
            temp_high = numerator_high >> 16;
        }

        // 检查是否大于8位值 (mov64 r3, r5; or64 r3, 8)
        if temp_high > 0xFF {
            shift |= 8;
        }

        // 根据前一个检查再次调整temp_high (mov64 r2, r0; rsh64 r2, 8)
        let mut lookup_index = temp_high;
        if temp_high > 0xFF {
            lookup_index = temp_high >> 8;
        }

        // 查表获取最后的移位 (lddw r5, 0x10001913b; add64 r5, r2; ldxb r2, [r5+0x0])
        let final_shift = shift + lookup_bits(lookup_index as u8);

        // 计算补充移位 (mov64 r2, 64; sub64 r2, r3)
        let complement_shift = 64 - final_shift;

        // 准备被除数和除数 (mov64 r5, r6; rsh64 r5, r3; lsh64 r1, r2; or64 r1, r5)
        let dividend_high = (pool_product >> final_shift) | (0u64 << complement_shift);

        // 左移被除数 (mov64 r9, r4; lsh64 r9, r2)
        let dividend_shifted = amount_adjusted << complement_shift;

        // 左移乘积 (lsh64 r6, r2)
        let prod_shifted = pool_product << complement_shift;

        // 分解被除数的高低位
        let dividend_shifted_high = dividend_shifted >> 32;
        let prod_shifted_high = prod_shifted >> 32;
        let prod_shifted_low = prod_shifted & 0xFFFFFFFF;

        // 第一步除法 (mov64 r5, r1; div64 r5, r7)
        let mut quotient_high = dividend_high / dividend_shifted_high;

        // 乘法检查 (mov64 r2, r7; mul64 r2, r5; mov64 r3, r1; sub64 r3, r2)
        let mut remainder = dividend_high - (dividend_shifted_high * quotient_high);

        // 循环调整商和余数
        loop {
            if quotient_high <= 0xFFFFFFFF {
                let product = quotient_high * prod_shifted_low;
                let compare_value = (remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r3, r7; add64 r5, -1)
            remainder += dividend_shifted_high;
            quotient_high -= 1;

            // 防止无限循环 (jgt r0, r3)
            if 0x100000000 > remainder {
                continue;
            }
            break;
        }

        // 计算下一步的被除数 (mul64 r2, r9; lsh64 r1, 32; or64 r1, r8; sub64 r1, r2)
        let next_dividend =
            ((dividend_high << 32) | prod_shifted_high) - (quotient_high * dividend_shifted);

        // 计算低位商 (mov64 r8, r1; div64 r8, r7)
        let mut quotient_low = next_dividend / dividend_shifted_high;

        // 计算新的余数 (mov64 r2, r8; mul64 r2, r7; sub64 r1, r2)
        let mut low_remainder = next_dividend - (quotient_low * dividend_shifted_high);

        // 调整低位商
        loop {
            if quotient_low <= 0xFFFFFFFF {
                let product = quotient_low * prod_shifted_low;
                let compare_value = (low_remainder << 32) | prod_shifted_low;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r1, r7; add64 r8, -1)
            low_remainder += dividend_shifted_high;
            quotient_low -= 1;

            // 防止无限循环 (jgt r0, r1)
            if 0x100000000 > low_remainder {
                continue;
            }
            break;
        }

        // 合并最终结果 (lsh64 r5, 32; add64 r8, r5)
        let final_result = (quotient_high << 32) | quotient_low;

        // 执行最终调整 (mov64 r1, r8; xor64 r1, -1; ldxdw r0, [r10-0x38]; add64 r0, r1)
        let inverted = !final_result;
        let result = pool_a + inverted;

        // 存储结果到输出缓冲区 (stxdw [r9+0x8], r4; stxdw [r9+0x0], r8)
        output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
        output_buffer[0..8].copy_from_slice(&final_result.to_le_bytes());
    }

    Ok(())
}
