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

/// Raydium V4池的流动性解析函数
/// 这个函数从Raydium V4池中解析流动性数据
///
/// # 参数说明
/// * `pool_state` - 池状态账户 (对应汇编中的 r1)
/// * `token_a_account` - 代币A账户 (对应汇编中的 r2)
/// * `token_b_account` - 代币B账户 (对应汇编中的 r3)
/// * `out_buffer` - 输出缓冲区 (对应汇编中的 r4)
pub fn raydium_v4_parse_liquidity(
    pool_state: &AccountInfo,
    token_a_account: &AccountInfo,
    token_b_account: &AccountInfo,
    out_buffer: &mut [u8],
) -> Result<bool> {
    // 初始化返回值为false (mov64 r0, 0)
    let mut result = false;

    // 读取代币A账户中的金额 (ldxdw r2, [r2+0x18]; ldxdw r2, [r2+0x40])
    let token_a_data = token_a_account.try_borrow_data()?;
    let token_a_amount = if token_a_data.len() >= 96 {
        u64::from_le_bytes(token_a_data[88..96].try_into().unwrap())
    } else {
        return Ok(false);
    };

    // 如果代币A金额为0，直接返回false (jeq r2, 0, lbb_350)
    if token_a_amount == 0 {
        return Ok(false);
    }

    // 读取代币B账户中的金额 (ldxdw r3, [r3+0x18]; ldxdw r3, [r3+0x40])
    let token_b_data = token_b_account.try_borrow_data()?;
    let token_b_amount = if token_b_data.len() >= 96 {
        u64::from_le_bytes(token_b_data[88..96].try_into().unwrap())
    } else {
        return Ok(false);
    };

    // 如果代币B金额为0，直接返回false (jeq r3, 0, lbb_350)
    if token_b_amount == 0 {
        return Ok(false);
    }

    // 读取池状态的数据长度 (ldxdw r5, [r1+0x10])
    let pool_data_len = pool_state.data_len() as u64;

    // 检查池状态数据长度是否足够 (mov64 r6, 208; jgt r6, r5, lbb_350)
    if pool_data_len < 208 {
        return Ok(false);
    }

    // 读取池状态数据
    let pool_data = pool_state.try_borrow_data()?;

    // 从池状态中读取流动性值 (ldxdw r5, [r1+0xc0]; ldxdw r1, [r1+0xc8])
    let liquidity_a = if pool_data.len() >= 200 {
        u64::from_le_bytes(pool_data[192..200].try_into().unwrap())
    } else {
        return Ok(false);
    };

    let liquidity_b = if pool_data.len() >= 208 {
        u64::from_le_bytes(pool_data[200..208].try_into().unwrap())
    } else {
        return Ok(false);
    };

    // 设置输出缓冲区的类型为0 (stxw [r4+0x0], r0)
    out_buffer[0..4].copy_from_slice(&0u32.to_le_bytes());

    // 计算并存储流动性差值 (sub64 r3, r1; stxdw [r4+0x10], r3)
    let diff_b = token_b_amount.saturating_sub(liquidity_b);
    out_buffer[16..24].copy_from_slice(&diff_b.to_le_bytes());

    // 计算并存储流动性差值 (sub64 r2, r5; stxdw [r4+0x8], r2)
    let diff_a = token_a_amount.saturating_sub(liquidity_a);
    out_buffer[8..16].copy_from_slice(&diff_a.to_le_bytes());

    // 设置结果为true (mov64 r0, 1)
    result = true;

    Ok(result)
}

/// Raydium CP池(常量乘积)的流动性解析函数
/// 这个函数从Raydium CP池中解析流动性数据
///
/// # 参数说明
/// * `pool_state` - 池状态账户 (对应汇编中的 r1)
/// * `token_a_account` - 代币A账户 (对应汇编中的 r2)
/// * `token_b_account` - 代币B账户 (对应汇编中的 r3)
/// * `out_buffer` - 输出缓冲区 (对应汇编中的 r4)
pub fn raydium_cp_parse_liquidity(
    pool_state: &AccountInfo,
    token_a_account: &AccountInfo,
    token_b_account: &AccountInfo,
    out_buffer: &mut [u8],
) -> Result<bool> {
    // 记录公钥以便调试 (syscall sol_log_pubkey)
    msg!("Pool: {}", pool_state.key());
    msg!("Token A: {}", token_a_account.key());
    msg!("Token B: {}", token_b_account.key());

    // 初始化返回值为false (mov64 r0, 0)
    let mut result = false;

    // 读取代币A账户中的金额 (ldxdw r1, [r9+0x18]; ldxdw r1, [r1+0x40])
    let token_a_data = token_a_account.try_borrow_data()?;
    let token_a_amount = if token_a_data.len() >= 96 {
        u64::from_le_bytes(token_a_data[88..96].try_into().unwrap())
    } else {
        return Ok(false);
    };

    // 如果代币A金额为0，直接返回false (jeq r1, 0, lbb_385)
    if token_a_amount == 0 {
        return Ok(false);
    }

    // 读取代币B账户中的金额 (ldxdw r2, [r8+0x18]; ldxdw r2, [r2+0x40])
    let token_b_data = token_b_account.try_borrow_data()?;
    let token_b_amount = if token_b_data.len() >= 96 {
        u64::from_le_bytes(token_b_data[88..96].try_into().unwrap())
    } else {
        return Ok(false);
    };

    // 如果代币B金额为0，直接返回false (jeq r2, 0, lbb_385)
    if token_b_amount == 0 {
        return Ok(false);
    }

    // 读取池状态的数据长度 (ldxdw r3, [r7+0x10])
    let pool_data_len = pool_state.data_len() as u64;

    // 检查池状态数据长度是否足够 (mov64 r4, 408; jgt r4, r3, lbb_385)
    if pool_data_len < 408 {
        return Ok(false);
    }

    // 读取池状态数据
    let pool_data = pool_state.try_borrow_data()?;

    // 从池状态中读取流动性值 - CP池的偏移量与V4不同
    // (ldxdw r4, [r3+0x16d]; ldxdw r5, [r3+0x15d])
    let liquidity_b1 = if pool_data.len() >= 373 {
        u64::from_le_bytes(pool_data[365..373].try_into().unwrap())
    } else {
        return Ok(false);
    };

    let liquidity_b2 = if pool_data.len() >= 381 {
        u64::from_le_bytes(pool_data[373..381].try_into().unwrap())
    } else {
        return Ok(false);
    };

    // 计算总流动性B (add64 r5, r4)
    let total_liquidity_b = liquidity_b1 + liquidity_b2;

    // 读取流动性A值 (ldxdw r4, [r3+0x165]; ldxdw r3, [r3+0x155])
    let liquidity_a1 = if pool_data.len() >= 357 {
        u64::from_le_bytes(pool_data[349..357].try_into().unwrap())
    } else {
        return Ok(false);
    };

    let liquidity_a2 = if pool_data.len() >= 365 {
        u64::from_le_bytes(pool_data[357..365].try_into().unwrap())
    } else {
        return Ok(false);
    };

    // 计算总流动性A (add64 r3, r4)
    let total_liquidity_a = liquidity_a1 + liquidity_a2;

    // 设置输出缓冲区的类型为0 (mov64 r5, 0; stxw [r6+0x0], r5)
    out_buffer[0..4].copy_from_slice(&0u32.to_le_bytes());

    // 计算并存储流动性B差值 (sub64 r2, r5; stxdw [r6+0x10], r2)
    let diff_b = token_b_amount.saturating_sub(total_liquidity_b);
    out_buffer[16..24].copy_from_slice(&diff_b.to_le_bytes());

    // 计算并存储流动性A差值 (sub64 r1, r3; stxdw [r6+0x8], r1)
    let diff_a = token_a_amount.saturating_sub(total_liquidity_a);
    out_buffer[8..16].copy_from_slice(&diff_a.to_le_bytes());

    // 设置结果为true (mov64 r0, 1)
    result = true;

    Ok(result)
}

/// Raydium获取流动性函数
/// 根据当前池内的流动性数据和报价金额计算最终流动性结果
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r1)
/// * `quote_amount` - 报价金额 (对应汇编中的 r2)
/// * `swap_direction` - 交换方向 (对应汇编中的 r3)
/// * `output_buffer` - 输出缓冲区 (对应汇编中的 r4)
pub fn raydium_get_liquidity(
    liquidity_data: &[u8],
    quote_amount: u64,
    swap_direction: bool,
    output_buffer: &mut [u8],
) -> Result<()> {
    // 计算基础费率调整 0.25% (div64 r8, 10000; mul64 r8, -25)
    let mut fee_adjusted = quote_amount / 10000 * 25;

    if swap_direction {
        // 处理 B->A 方向的交换

        // 读取流动性数据并拆分为高低位
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析 A 池的高低位数据
        let pool_a_lo = pool_a & 0xFFFFFFFF;
        let pool_a_hi = pool_a >> 32;

        // 解析 B 池的高低位数据
        let pool_b_lo = pool_b & 0xFFFFFFFF;
        let pool_b_hi = pool_b >> 32;

        // 计算交叉乘积 (r7 = r5 * r1)
        let mut cross_mul1 = pool_b_hi * pool_a_lo;

        // 计算低位乘积并右移32位 (r3 = (r0 * r1) >> 32)
        let cross_mul2 = (pool_a_lo * pool_b_lo) >> 32;

        // 合并交叉乘积 (r3 = r7 + r3)
        let cross_sum = cross_mul1 + cross_mul2;

        // 调整费率后的数量 (r8 = r8 + r2)
        let adjusted_amount = quote_amount + fee_adjusted;

        // 获取B池高位 (r1 = r9 >> 32)
        let cross_hi = cross_sum >> 32;

        // 计算高位的交叉乘积 (r5 = r5 * r1)
        let cross_mul3 = pool_a_hi * pool_b_hi;

        // 合并高位结果 (r6 = r6 + r5)
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积 (r0 = r0 * r1)
        let low_mul = pool_a_lo * pool_b_hi;

        // 获取交叉乘积的低32位 (r3 = r3 & 0xFFFFFFFF)
        let cross_lo = cross_sum & 0xFFFFFFFF;

        // 合并低位结果 (r3 = r3 + r0)
        let low_sum = low_mul + cross_lo;

        // 获取低位溢出 (r3 = r3 >> 32)
        let low_shift = low_sum >> 32;

        // 合并最终结果 (r6 = r6 + r3)
        let total_sum = cross_sum2 + low_shift;

        // 计算最终数量 (r5 = r8 + r9)
        let amount_adjusted = adjusted_amount + pool_b;

        // 计算池乘积 (r7 = r7 * r9)
        let prod = pool_a * pool_b;

        // 如果总和为0，直接执行简单的除法 (jne r6, 0)
        if total_sum == 0 {
            // 简单除法 (r7 = r7 / r5)
            let result = prod / amount_adjusted;

            // 写入输出缓冲区 (stxdw [r4+0x8], r5; stxdw [r4+0x0], r8)
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&result.to_le_bytes());

            return Ok(());
        }

        // 复杂大数除法开始
        // 检查被除数是否大于32位 (lddw r0, 0xffffffff; jgt r5, r0)
        let mut shift = 0;
        if amount_adjusted > 0xFFFFFFFF {
            shift = 32;
        }

        // 确定被除数的高32位或完整数值 (jgt r5, r0)
        let mut numerator_high = amount_adjusted;
        if amount_adjusted > 0xFFFFFFFF {
            numerator_high = amount_adjusted >> 32;
        }

        // 计算需要的移位 - 检查是否大于16位值
        if numerator_high > 0xFFFF {
            shift |= 16;
        }

        // 根据前一个检查调整numerator_high
        let mut temp_high = numerator_high;
        if numerator_high > 0xFFFF {
            temp_high = numerator_high >> 16;
        }

        // 检查是否大于8位值
        if temp_high > 0xFF {
            shift |= 8;
        }

        // 根据前一个检查再次调整temp_high
        let mut lookup_index = temp_high;
        if temp_high > 0xFF {
            lookup_index = temp_high >> 8;
        }

        // 使用查找表获取最后的移位 (lddw r0, 0x10001903b; add64 r0, r1; ldxb r1, [r0+0x0])
        // 这里简化为对数查找，实际应该使用表格
        let final_shift = shift + lookup_bits(lookup_index as u8);

        // 计算补充移位 (r1 = 64 - r3)
        let complement_shift = 64 - final_shift;

        // 准备被除数和除数 (rsh64 r0, r3; lsh64 r6, r1; or64 r6, r0)
        let dividend_high = (prod >> final_shift) | (0u64 << complement_shift);

        // 左移被除数 (lsh64 r4, r1)
        let dividend_shifted = amount_adjusted << complement_shift;

        // 左移乘积 (lsh64 r7, r1)
        let prod_shifted = prod << complement_shift;

        // 分解被除数的高低位
        let dividend_shifted_high = dividend_shifted >> 32;
        let prod_shifted_high = prod_shifted >> 32;
        let prod_shifted_low = prod_shifted & 0xFFFFFFFF;

        // 第一步除法 (div64 r0, r1)
        let mut quotient_high = dividend_high / dividend_shifted_high;

        // 乘法检查 (mul64 r3, r0; sub64 r2, r3)
        let mut remainder = dividend_high - (dividend_shifted_high * quotient_high);

        // 调整商直到满足条件
        loop {
            if quotient_high <= 0xFFFFFFFF {
                let product = quotient_high * prod_shifted_low;
                let compare_value = (remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r2, r1; add64 r0, -1)
            remainder += dividend_shifted_high;
            quotient_high -= 1;

            // 防止无限循环
            if remainder > 0x100000000 {
                break;
            }
        }

        // 计算下一步的被除数 (mul64 r2, r3; sub64 r1, r2)
        let next_dividend =
            ((dividend_high << 32) | prod_shifted_high) - (quotient_high * dividend_shifted);

        // 计算低位商 (div64 r8, r1)
        let mut quotient_low = next_dividend / dividend_shifted_high;

        // 计算新的余数 (mul64 r2, r1; sub64 r6, r2)
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

            // 调整商和余数
            low_remainder += dividend_shifted_high;
            quotient_low -= 1;

            // 防止无限循环
            if low_remainder > 0x100000000 {
                break;
            }
        }

        // 合并最终结果 (lsh64 r0, 32; add64 r8, r0)
        let final_result = (quotient_high << 32) | quotient_low;

        // 写入输出缓冲区 (stxdw [r4+0x8], r5; stxdw [r4+0x0], r8)
        output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
        output_buffer[0..8].copy_from_slice(&final_result.to_le_bytes());
    } else {
        // 处理 A->B 方向的交换

        // 读取流动性数据
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

        // 计算低位乘积并右移32位
        let cross_mul2 = (pool_a_lo * pool_b_lo) >> 32;

        // 合并交叉乘积
        let cross_sum = cross_mul1 + cross_mul2;

        // 调整费率后的数量
        let adjusted_amount = quote_amount + fee_adjusted;

        // 计算高位的交叉乘积
        let cross_mul3 = pool_a_hi * pool_b_hi;

        // 获取交叉乘积的高32位
        let cross_hi = cross_sum >> 32;

        // 合并高位结果
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积
        let low_mul = pool_a_lo * pool_b_hi;

        // 获取交叉乘积的低32位
        let cross_lo = cross_sum & 0xFFFFFFFF;

        // 合并低位结果
        let low_sum = low_mul + cross_lo;

        // 获取低位溢出
        let low_shift = low_sum >> 32;

        // 合并最终结果
        let total_sum = cross_sum2 + low_shift;

        // 计算最终数量
        let amount_adjusted = adjusted_amount + pool_a;

        // 计算池乘积
        let prod = pool_a * pool_b;

        // 如果总和为0，直接执行简单的除法
        if total_sum == 0 {
            // 简单除法
            let result = prod / amount_adjusted;

            // 写入输出缓冲区
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&result.to_le_bytes());

            return Ok(());
        }

        // 复杂大数除法开始 - 与B->A方向类似的逻辑
        // 检查被除数是否大于32位
        let mut shift = 0;
        if amount_adjusted > 0xFFFFFFFF {
            shift = 32;
        }

        // 确定被除数的高32位或完整数值
        let mut numerator_high = amount_adjusted;
        if amount_adjusted > 0xFFFFFFFF {
            numerator_high = amount_adjusted >> 32;
        }

        // 计算需要的移位 - 检查是否大于16位值
        if numerator_high > 0xFFFF {
            shift |= 16;
        }

        // 根据前一个检查调整numerator_high
        let mut temp_high = numerator_high;
        if numerator_high > 0xFFFF {
            temp_high = numerator_high >> 16;
        }

        // 检查是否大于8位值
        if temp_high > 0xFF {
            shift |= 8;
        }

        // 根据前一个检查再次调整temp_high
        let mut lookup_index = temp_high;
        if temp_high > 0xFF {
            lookup_index = temp_high >> 8;
        }

        // (lddw r0, 0x10001903b; add64 r0, r3; ldxb r4, [r0+0x0])
        let final_shift = shift + lookup_bits(lookup_index as u8);

        // 计算补充移位
        let complement_shift = 64 - final_shift;

        // 准备被除数和除数
        let dividend_high = (prod >> final_shift) | (0u64 << complement_shift);

        // 左移被除数
        let dividend_shifted = amount_adjusted << complement_shift;

        // 左移乘积
        let prod_shifted = prod << complement_shift;

        // 分解被除数的高低位
        let dividend_shifted_high = dividend_shifted >> 32;
        let prod_shifted_high = prod_shifted >> 32;
        let prod_shifted_low = prod_shifted & 0xFFFFFFFF;

        // 第一步除法
        let mut quotient_high = dividend_high / dividend_shifted_high;

        // 乘法检查
        let mut remainder = dividend_high - (dividend_shifted_high * quotient_high);

        // 调整商直到满足条件
        loop {
            if quotient_high <= 0xFFFFFFFF {
                let product = quotient_high * prod_shifted_low;
                let compare_value = (remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数
            remainder += dividend_shifted_high;
            quotient_high -= 1;

            // 防止无限循环
            if remainder > 0x100000000 {
                break;
            }
        }

        // 计算下一步的被除数
        let next_dividend =
            ((dividend_high << 32) | prod_shifted_high) - (quotient_high * dividend_shifted);

        // 计算低位商
        let mut quotient_low = next_dividend / dividend_shifted_high;

        // 计算新的余数
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

            // 调整商和余数
            low_remainder += dividend_shifted_high;
            quotient_low -= 1;

            // 防止无限循环
            if low_remainder > 0x100000000 {
                break;
            }
        }

        // 合并最终结果
        let final_result = (quotient_high << 32) | quotient_low;

        // 写入输出缓冲区
        output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
        output_buffer[0..8].copy_from_slice(&final_result.to_le_bytes());
    }

    Ok(())
}

/// 辅助函数：获取数值的最高有效位位置
/// 这个函数模拟汇编代码中的查找表操作
fn lookup_bits(value: u8) -> u64 {
    // 这个函数模拟汇编中的位查找表
    // 返回最高有效位的位置
    if value == 0 {
        return 0;
    }

    let mut bits = 0;
    let mut v = value;

    while v > 0 {
        v >>= 1;
        bits += 1;
    }

    bits
}

/// Raydium流动性有效性验证函数
/// 验证流动性数据是否有效
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r1)
pub fn raydium_is_valid(liquidity_data: &[u8]) -> bool {
    // 读取流动性B值 (ldxdw r3, [r1+0x8])
    let liquidity_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

    // 初始化结果为true (mov64 r0, 1)
    let mut result_a = true;

    // 初始化B检查结果为true (mov64 r2, 1)
    let mut result_b = true;

    // 检查流动性B是否大于1000 (jgt r3, 1000)
    if !(liquidity_b > 1000) {
        // 如果不大于，设置B检查结果为false (mov64 r2, 0)
        result_b = false;
    }

    // 读取流动性A值 (ldxdw r1, [r1+0x0])
    let liquidity_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

    // 检查流动性A是否大于1000 (jgt r1, 1000)
    if !(liquidity_a > 1000) {
        // 如果不大于，设置A检查结果为false (mov64 r0, 0)
        result_a = false;
    }

    // 计算最终结果 - A和B的结果进行与运算 (and64 r0, r2)
    result_a && result_b
}

/// Raydium DEX的报价和流动性计算函数
/// 这个函数计算报价并返回流动性信息
///
/// # 参数说明
/// * `liquidity_data` - 流动性数据 (对应汇编中的 r1)
/// * `quote_amount` - 报价金额 (对应汇编中的 r2)
/// * `swap_direction` - 交换方向 (对应汇编中的 r3)
/// * `output_buffer` - 输出缓冲区 (对应汇编中的 r4)
pub fn raydium_get_quote_and_liquidity(
    liquidity_data: &[u8],
    quote_amount: u64,
    swap_direction: bool,
    output_buffer: &mut [u8],
) -> Result<u64> {
    // 计算基础费率调整 0.25% (mov64 r9, r2; div64 r9, 10000; mul64 r9, -25)
    let mut fee_adjusted = quote_amount / 10000 * 25;

    if swap_direction {
        // 处理 B->A 方向的交换 (jeq r3, 0, lbb_1002)

        // 读取流动性数据 (ldxdw r3, [r1+0x0])
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

        // 解析 A 池的高低位数据 (mov64 r6, r3; lsh64 r6, 32; rsh64 r6, 32)
        let pool_a_lo = pool_a & 0xFFFFFFFF;

        // 读取第二部分流动性数据 (ldxdw r0, [r1+0x8])
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析 B 池的高低位数据 (mov64 r8, r0; lsh64 r8, 32; rsh64 r8, 32)
        let pool_b_lo = pool_b & 0xFFFFFFFF;

        // 获取 A 池的高32位 (mov64 r1, r3; rsh64 r1, 32)
        let pool_a_hi = pool_a >> 32;

        // 计算交叉乘积 (mov64 r5, r8; mul64 r5, r1)
        let mut cross_mul1 = pool_b_lo * pool_a_hi;

        // 计算低位乘积并右移32位 (mul64 r8, r6; rsh64 r8, 32)
        let cross_mul2 = (pool_b_lo * pool_a_lo) >> 32;

        // 合并交叉乘积 (add64 r8, r5)
        let cross_sum = cross_mul1 + cross_mul2;

        // 调整费率后的数量 (add64 r9, r2)
        let adjusted_amount = quote_amount + fee_adjusted;

        // 获取 B 池的高32位 (mov64 r2, r0; rsh64 r2, 32)
        let pool_b_hi = pool_b >> 32;

        // 计算高位的交叉乘积 (mov64 r7, r2; mul64 r7, r1)
        let cross_mul3 = pool_b_hi * pool_a_hi;

        // 获取交叉乘积的高32位 (mov64 r5, r8; rsh64 r5, 32)
        let cross_hi = cross_sum >> 32;

        // 合并高位结果 (add64 r5, r7)
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积 (mul64 r2, r6)
        let low_mul = pool_b_hi * pool_a_lo;

        // 获取交叉乘积的低32位 (lsh64 r8, 32; rsh64 r8, 32)
        let cross_lo = cross_sum & 0xFFFFFFFF;

        // 合并低位结果 (add64 r8, r2)
        let low_sum = low_mul + cross_lo;

        // 获取低位溢出 (rsh64 r8, 32)
        let low_shift = low_sum >> 32;

        // 合并最终结果 (add64 r5, r8)
        let total_sum = cross_sum2 + low_shift;

        // 计算最终数量 (add64 r9, r3)
        let amount_adjusted = adjusted_amount + pool_a;

        // 计算池乘积 (mov64 r7, r0; mul64 r7, r3)
        let prod = pool_a * pool_b;

        // 如果总和为0，直接执行简单的除法 (jne r5, 0)
        if total_sum == 0 {
            // 简单除法 (div64 r7, r9)
            let result = prod / amount_adjusted;

            // 存储新流动性值到输出缓冲区 (stxdw [r4+0x8], r7; stxdw [r4+0x0], r9)
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&result.to_le_bytes());

            // 返回结果的补码 (xor64 r3, -1; add64 r0, r3; exit)
            return Ok(!result + 1);
        }

        // 复杂大数除法开始
        // 检查被除数是否大于32位 (mov64 r2, 1; lddw r1, 0xffffffff; jgt r9, r1)
        let mut shift = 0;
        if amount_adjusted > 0xFFFFFFFF {
            shift = 32;
        }

        // 确定被除数的高32位或完整数值 (mov64 r3, r9; rsh64 r3, 32; jgt r9, r1)
        let mut numerator_high = amount_adjusted;
        if amount_adjusted > 0xFFFFFFFF {
            numerator_high = amount_adjusted >> 32;
        }

        // 计算需要的移位 - 检查是否大于16位值 (lsh64 r2, 5; mov64 r1, r2; or64 r1, 16; jgt r3, 65535)
        if numerator_high > 0xFFFF {
            shift |= 16;
        }

        // 根据前一个检查调整numerator_high (mov64 r6, r3; rsh64 r6, 16; jgt r3, 65535)
        let mut temp_high = numerator_high;
        if numerator_high > 0xFFFF {
            temp_high = numerator_high >> 16;
        }

        // 检查是否大于8位值 (mov64 r2, r1; or64 r2, 8; jgt r6, 255)
        if temp_high > 0xFF {
            shift |= 8;
        }

        // 根据前一个检查再次调整temp_high (mov64 r3, r6; rsh64 r3, 8; jgt r6, 255)
        let mut lookup_index = temp_high;
        if temp_high > 0xFF {
            lookup_index = temp_high >> 8;
        }

        // 保存寄存器状态 (stxdw [r10-0x18], r0; stxdw [r10-0x10], r4)
        // Rust不需要显式保存变量

        // 查表获取最后的移位 (lddw r4, 0x10001903b; add64 r4, r3; ldxb r3, [r4+0x0])
        let final_shift = shift + lookup_bits(lookup_index as u8);

        // 计算补充移位 (mov64 r3, 64; sub64 r3, r2)
        let complement_shift = 64 - final_shift;

        // 准备被除数和除数 (mov64 r0, r7; mov64 r4, r0; rsh64 r4, r2; lsh64 r5, r3; or64 r5, r4)
        let dividend_high = (prod >> final_shift) | (0u64 << complement_shift);

        // 保存调整后的金额 (stxdw [r10-0x20], r9)
        // Rust不需要显式保存变量

        // 左移被除数 (mov64 r8, r9; lsh64 r8, r3)
        let dividend_shifted = amount_adjusted << complement_shift;

        // 左移乘积 (lsh64 r0, r3)
        let prod_shifted = prod << complement_shift;

        // 分解被除数的高低位 (mov64 r9, r0; rsh64 r9, 32; mov64 r7, r8; rsh64 r7, 32)
        let dividend_shifted_high = dividend_shifted >> 32;
        let prod_shifted_high = prod_shifted >> 32;
        let prod_shifted_low = prod_shifted & 0xFFFFFFFF;

        // 第一步除法 (mov64 r6, r5; div64 r6, r7)
        let mut quotient_high = dividend_high / dividend_shifted_high;

        // 乘法检查 (mov64 r3, r7; mul64 r3, r6; mov64 r1, r5; sub64 r1, r3)
        let mut remainder = dividend_high - (dividend_shifted_high * quotient_high);

        // 保存低位结果 (lsh64 r0, 32; rsh64 r0, 32; stxdw [r10-0x8], r0; stxdw [r10-0x28], r8)
        // Rust不需要显式保存变量

        // 获取低位移位值 (lsh64 r8, 32; rsh64 r8, 32)
        // 循环调整商和余数
        loop {
            if quotient_high <= 0xFFFFFFFF {
                let product = quotient_high * prod_shifted_low;
                let compare_value = (remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r1, r7; add64 r6, -1)
            remainder += dividend_shifted_high;
            quotient_high -= 1;

            // 防止无限循环 (jgt r3, r1)
            if 0x100000000 > remainder {
                continue;
            }
            break;
        }

        // 计算下一步的被除数 (mov64 r1, r6; mul64 r1, r2; lsh64 r5, 32; or64 r5, r9; sub64 r5, r1)
        let next_dividend =
            ((dividend_high << 32) | prod_shifted_high) - (quotient_high * dividend_shifted);

        // 计算低位商 (mov64 r3, r5; div64 r3, r7)
        let mut quotient_low = next_dividend / dividend_shifted_high;

        // 计算新的余数 (mov64 r1, r3; mul64 r1, r7; sub64 r5, r1)
        let mut low_remainder = next_dividend - (quotient_low * dividend_shifted_high);

        // 读取保存的低位值 (ldxdw r9, [r10-0x8])
        // 调整低位商
        loop {
            if quotient_low <= 0xFFFFFFFF {
                let product = quotient_low * prod_shifted_low;
                let compare_value = (low_remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r2, r7; add64 r3, -1)
            low_remainder += dividend_shifted_high;
            quotient_low -= 1;

            // 防止无限循环 (jgt r4, r5)
            if 0x100000000 > low_remainder {
                continue;
            }
            break;
        }

        // 合并最终结果 (lsh64 r6, 32; add64 r3, r6)
        let final_result = (quotient_high << 32) | quotient_low;

        // 恢复保存的寄存器值 (mov64 r9, r3; ldxdw r4, [r10-0x10]; ldxdw r0, [r10-0x18]; ldxdw r7, [r10-0x8])
        // Rust不需要显式恢复变量

        // 存储新流动性值到输出缓冲区 (stxdw [r4+0x8], r7; stxdw [r4+0x0], r9)
        output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
        output_buffer[0..8].copy_from_slice(&final_result.to_le_bytes());

        // 返回结果的补码 (xor64 r3, -1; add64 r0, r3; exit)
        return Ok(!final_result + 1);
    } else {
        // 处理 A->B 方向的交换

        // 读取流动性数据 (ldxdw r0, [r1+0x0])
        let pool_a = u64::from_le_bytes(liquidity_data[0..8].try_into().unwrap());

        // 解析 A 池的高低位数据 (mov64 r8, r0; lsh64 r8, 32; rsh64 r8, 32)
        let pool_a_lo = pool_a & 0xFFFFFFFF;

        // 读取第二部分流动性数据 (ldxdw r3, [r1+0x8])
        let pool_b = u64::from_le_bytes(liquidity_data[8..16].try_into().unwrap());

        // 解析 B 池的高低位数据 (mov64 r7, r3; lsh64 r7, 32; rsh64 r7, 32)
        let pool_b_lo = pool_b & 0xFFFFFFFF;

        // 获取 A 池的高32位 (mov64 r5, r0; rsh64 r5, 32)
        let pool_a_hi = pool_a >> 32;

        // 计算交叉乘积 (mov64 r6, r5; mul64 r6, r7)
        let mut cross_mul1 = pool_a_hi * pool_b_lo;

        // 计算低位乘积并右移32位 (mov64 r1, r8; mul64 r1, r7; rsh64 r1, 32)
        let cross_mul2 = (pool_a_lo * pool_b_lo) >> 32;

        // 合并交叉乘积 (add64 r1, r6)
        let cross_sum = cross_mul1 + cross_mul2;

        // 调整费率后的数量 (add64 r9, r2)
        let adjusted_amount = quote_amount + fee_adjusted;

        // 获取 B 池的高32位 (mov64 r6, r3; rsh64 r6, 32)
        let pool_b_hi = pool_b >> 32;

        // 计算高位的交叉乘积 (mul64 r5, r6)
        let cross_mul3 = pool_a_hi * pool_b_hi;

        // 获取交叉乘积的高32位 (mov64 r2, r1; rsh64 r2, 32)
        let cross_hi = cross_sum >> 32;

        // 合并高位结果 (add64 r2, r5)
        let cross_sum2 = cross_hi + cross_mul3;

        // 计算低位的乘积 (mul64 r8, r6)
        let low_mul = pool_a_lo * pool_b_hi;

        // 获取交叉乘积的低32位 (lsh64 r1, 32; rsh64 r1, 32)
        let cross_lo = cross_sum & 0xFFFFFFFF;

        // 合并低位结果 (add64 r1, r8)
        let low_sum = low_mul + cross_lo;

        // 获取低位溢出 (rsh64 r1, 32)
        let low_shift = low_sum >> 32;

        // 合并最终结果 (add64 r2, r1)
        let total_sum = cross_sum2 + low_shift;

        // 计算最终数量 (mov64 r7, r9; add64 r7, r3)
        let amount_adjusted = adjusted_amount + pool_b;

        // 计算池乘积 (mov64 r9, r0; mul64 r9, r3)
        let prod = pool_a * pool_b;

        // 如果总和为0，直接执行简单的除法 (jne r2, 0)
        if total_sum == 0 {
            // 简单除法 (div64 r9, r7)
            let result = prod / amount_adjusted;

            // 存储新流动性值到输出缓冲区 (stxdw [r4+0x8], r7; stxdw [r4+0x0], r9)
            output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
            output_buffer[0..8].copy_from_slice(&result.to_le_bytes());

            // 返回结果的补码 (xor64 r3, -1; add64 r0, r3; exit)
            return Ok(!result + 1);
        }

        // 复杂大数除法开始 - 与B->A方向类似的逻辑
        // 检查被除数是否大于32位 (mov64 r1, 1; lddw r5, 0xffffffff; stxdw [r10-0x8], r7; jgt r7, r5)
        let mut shift = 0;
        if amount_adjusted > 0xFFFFFFFF {
            shift = 32;
        }

        // 确定被除数的高32位或完整数值 (ldxdw r6, [r10-0x8]; mov64 r3, r6; rsh64 r3, 32; jgt r6, r5)
        let mut numerator_high = amount_adjusted;
        if amount_adjusted > 0xFFFFFFFF {
            numerator_high = amount_adjusted >> 32;
        }

        // 计算需要的移位 - 检查是否大于16位值 (lsh64 r1, 5; mov64 r5, r1; or64 r5, 16; jgt r3, 65535)
        if numerator_high > 0xFFFF {
            shift |= 16;
        }

        // 根据前一个检查调整numerator_high (mov64 r6, r3; rsh64 r6, 16; jgt r3, 65535)
        let mut temp_high = numerator_high;
        if numerator_high > 0xFFFF {
            temp_high = numerator_high >> 16;
        }

        // 检查是否大于8位值 (mov64 r1, r5; or64 r1, 8; jgt r6, 255)
        if temp_high > 0xFF {
            shift |= 8;
        }

        // 根据前一个检查再次调整temp_high (mov64 r3, r6; rsh64 r3, 8; jgt r6, 255)
        let mut lookup_index = temp_high;
        if temp_high > 0xFF {
            lookup_index = temp_high >> 8;
        }

        // 保存寄存器状态 (stxdw [r10-0x18], r0; stxdw [r10-0x10], r4)
        // Rust不需要显式保存变量

        // 查表获取最后的移位 (lddw r4, 0x10001903b; add64 r4, r3; ldxb r3, [r4+0x0])
        let final_shift = shift + lookup_bits(lookup_index as u8);

        // 计算补充移位 (mov64 r3, 64; sub64 r3, r1)
        let complement_shift = 64 - final_shift;

        // 准备被除数和除数 (mov64 r4, r9; rsh64 r4, r1; lsh64 r2, r3; or64 r2, r4)
        let dividend_high = (prod >> final_shift) | (0u64 << complement_shift);

        // 左移被除数 (ldxdw r8, [r10-0x8]; lsh64 r8, r3)
        let dividend_shifted = amount_adjusted << complement_shift;

        // 左移乘积 (lsh64 r9, r3)
        let prod_shifted = prod << complement_shift;

        // 分解被除数的高低位 (mov64 r4, r9; rsh64 r9, 32; mov64 r7, r8; rsh64 r7, 32)
        let dividend_shifted_high = dividend_shifted >> 32;
        let prod_shifted_high = prod_shifted >> 32;
        let prod_shifted_low = prod_shifted & 0xFFFFFFFF;

        // 第一步除法 (mov64 r6, r2; div64 r6, r7)
        let mut quotient_high = dividend_high / dividend_shifted_high;

        // 乘法检查 (mov64 r3, r7; mul64 r3, r6; mov64 r1, r2; sub64 r1, r3)
        let mut remainder = dividend_high - (dividend_shifted_high * quotient_high);

        // 保存低位结果 (lsh64 r4, 32; rsh64 r4, 32; stxdw [r10-0x20], r4; stxdw [r10-0x28], r8)
        // Rust不需要显式保存变量

        // 获取低位移位值 (lsh64 r8, 32; rsh64 r8, 32)
        // 循环调整商和余数
        loop {
            if quotient_high <= 0xFFFFFFFF {
                let product = quotient_high * prod_shifted_low;
                let compare_value = (remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r1, r7; add64 r6, -1)
            remainder += dividend_shifted_high;
            quotient_high -= 1;

            // 防止无限循环 (jgt r3, r1)
            if 0x100000000 > remainder {
                continue;
            }
            break;
        }

        // 计算下一步的被除数 (mov64 r1, r6; ldxdw r3, [r10-0x28]; mul64 r1, r3; lsh64 r2, 32; or64 r2, r9; sub64 r2, r1)
        let next_dividend =
            ((dividend_high << 32) | prod_shifted_high) - (quotient_high * dividend_shifted);

        // 计算低位商 (mov64 r3, r2; div64 r3, r7)
        let mut quotient_low = next_dividend / dividend_shifted_high;

        // 计算新的余数 (mov64 r1, r3; mul64 r1, r7; sub64 r2, r1)
        let mut low_remainder = next_dividend - (quotient_low * dividend_shifted_high);

        // 读取保存的低位值 (ldxdw r9, [r10-0x20])
        // 调整低位商
        loop {
            if quotient_low <= 0xFFFFFFFF {
                let product = quotient_low * prod_shifted_low;
                let compare_value = (low_remainder << 32) | prod_shifted_high;

                if compare_value >= product {
                    break;
                }
            }

            // 调整商和余数 (add64 r2, r7; add64 r3, -1)
            low_remainder += dividend_shifted_high;
            quotient_low -= 1;

            // 防止无限循环 (jgt r4, r2)
            if 0x100000000 > low_remainder {
                continue;
            }
            break;
        }

        // 合并最终结果 (lsh64 r6, 32; add64 r3, r6)
        let final_result = (quotient_high << 32) | quotient_low;

        // 恢复保存的寄存器值 (mov64 r9, r3; ldxdw r4, [r10-0x10]; ldxdw r0, [r10-0x18]; ldxdw r7, [r10-0x8])
        // Rust不需要显式恢复变量

        // 存储新流动性值到输出缓冲区 (stxdw [r4+0x8], r7; stxdw [r4+0x0], r9)
        output_buffer[8..16].copy_from_slice(&amount_adjusted.to_le_bytes());
        output_buffer[0..8].copy_from_slice(&final_result.to_le_bytes());

        // 返回结果的补码 (xor64 r3, -1; add64 r0, r3; exit)
        return Ok(!final_result + 1);
    }
}
