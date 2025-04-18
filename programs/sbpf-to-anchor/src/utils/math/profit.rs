use crate::utils::*;
use anchor_lang::prelude::*;

pub fn calculate_profit(
    quote_amount: u64,
    liquidity_amount: u64,
    swap_direction: bool,
    pool_state: &mut AccountInfo,
) -> Result<u64> {
    let mut liquidity_info = [0u8; 24]; // Based on assembly using r9 which points to 24 bytes buffer

    // First get both quote and liquidity
    get_quote_and_liquidity(&liquidity_info, &mut liquidity_info)?;

    // Get liquidity with the info
    get_liquidity(&liquidity_info, &mut liquidity_info)?;

    // Get final quote with opposite direction
    let final_quote = get_quote(&liquidity_info, quote_amount, !swap_direction)?;

    // Calculate profit by subtracting initial quote amount
    Ok(final_quote.saturating_sub(quote_amount))
}

/// 优化版本的利润计算函数
/// 与原始版本相比，这个版本在参数传递和内存使用上进行了优化
///
/// # 参数说明
/// * `quote_amount` - 报价数量 (对应汇编中的 r1/r6)
/// * `pool_state` - 池状态账户 (对应汇编中的 r2，存储在栈上 [r10-0x20])
/// * `liquidity_amount` - 流动性数量 (对应汇编中的 r3)
/// * `swap_direction` - 交换方向 (对应汇编中的 r4/r7)
pub fn calculate_profit_optimised(
    quote_amount: u64,
    pool_state: &mut AccountInfo,
    liquidity_amount: u64,
    swap_direction: bool,
) -> Result<u64> {
    // 创建24字节的缓冲区用于存储流动性信息
    // 在汇编中对应 r9 = r10 - 24 的操作
    let mut liquidity_info = [0u8; 24];

    // 第一步：获取报价和流动性信息
    // 对应汇编中的 call get_quote_and_liquidity
    // 参数顺序：liquidity_amount(r3), quote_amount(r2), swap_direction(r7), liquidity_info(r9)
    let initial_quote = get_quote_and_liquidity(&liquidity_info, &mut liquidity_info)?;

    // 第二步：更新流动性信息
    // 对应汇编中的 call get_liquidity
    // 参数顺序：liquidity_info(r9), pool_state(r2), swap_direction(r7), liquidity_info(r9)
    get_liquidity(&liquidity_info, &mut liquidity_info)?;

    // 第三步：使用相反的交换方向获取最终报价
    // 对应汇编中的 xor64 r7, 1 和 call get_quote
    // 参数顺序：liquidity_info(r9), initial_quote(r8), !swap_direction(r7 xor 1)
    let final_quote = get_quote(&liquidity_info, initial_quote, !swap_direction)?;

    // 最后计算利润：最终报价 - 初始报价
    // 对应汇编中的 sub64 r0, r6
    Ok(final_quote.saturating_sub(quote_amount))
}

/// 计算最优策略的函数
/// 这个函数实现了一个复杂的优化算法来寻找最佳交易策略
///
/// # 参数说明
/// * `initial_state` - 初始状态 (对应汇编中的 r1)
/// * `pool_state` - 池状态账户 (对应汇编中的 r2，存储在 [r10-0x28])
/// * `liquidity_amount` - 流动性数量 (对应汇编中的 r3)
/// * `swap_direction` - 交换方向 (对应汇编中的 r4/r9)
/// * `config` - 配置信息 (对应汇编中的 r5，存储在 [r10-0x20])
pub fn calculate_optimal_strategy(
    initial_state: u64,
    pool_state: &mut AccountInfo,
    liquidity_amount: u64,
    swap_direction: bool,
    config: &mut AccountInfo,
) -> Result<bool> {
    // 从配置中读取关键参数
    let config_data = config.try_borrow_data()?;
    let upper_bound_param1 = u64::from_le_bytes(config_data[0..8].try_into().unwrap());
    let upper_bound_param2 = u64::from_le_bytes(config_data[8..16].try_into().unwrap());

    // 计算上限值
    let upper_bound = calculate_upper_bound(initial_state, config, pool_state)?;

    // 创建24字节的缓冲区用于存储流动性信息
    let mut liquidity_info = [0u8; 24];

    // 第一步：获取初始报价和流动性
    let initial_quote = get_quote_and_liquidity(&liquidity_info, &mut liquidity_info)?;

    // 更新流动性信息
    get_liquidity(&liquidity_info, &mut liquidity_info)?;

    // 计算反向报价
    let opposite_direction = !swap_direction;
    let quote_result = get_quote(&liquidity_info, initial_quote, opposite_direction)?;

    // 计算初始利润
    let initial_profit = quote_result.saturating_sub(upper_bound);

    // 读取配置参数
    let min_profit_threshold = u64::from_le_bytes(config_data[32..40].try_into().unwrap());
    let iteration_limit = u64::from_le_bytes(config_data[24..32].try_into().unwrap());

    // 如果没有迭代限制，直接返回
    if iteration_limit == 0 {
        return Ok(false);
    }

    // 检查上限值是否小于1000
    if upper_bound < 1000 {
        return Ok(false);
    }

    // 优化循环
    let mut current_amount = upper_bound;
    let mut best_profit = initial_profit;
    let mut best_amount = upper_bound;
    let mut iteration_count = 0u64;

    while current_amount > 1000 && iteration_count < 255 {
        // 尝试新的报价金额
        let test_amount = current_amount.saturating_sub(1000);

        // 计算新的报价
        let mut temp_liquidity_info = [0u8; 24];
        let test_quote = get_quote_and_liquidity(&liquidity_info, &mut temp_liquidity_info)?;

        // 更新流动性
        get_liquidity(&temp_liquidity_info, &mut temp_liquidity_info)?;

        // 计算新的利润
        let new_quote = get_quote(&temp_liquidity_info, test_quote, opposite_direction)?;

        let new_profit = new_quote.saturating_sub(test_amount);

        // 更新最佳结果
        if new_profit > best_profit {
            best_profit = new_profit;
            best_amount = test_amount;
        }

        current_amount = test_amount;
        iteration_count += 1;
    }

    // 更新结果到配置中
    let mut config_data = config.try_borrow_mut_data()?;
    let result_data = &mut config_data[min_profit_threshold as usize..];
    result_data[0..8].copy_from_slice(&best_amount.to_le_bytes());
    result_data[8..16].copy_from_slice(&best_profit.to_le_bytes());

    Ok(true)
}

pub fn calculate_optimal_strategy_optimised(
    initial_amount: u64,
    pool_state: &mut AccountInfo,
    swap_direction: bool,
    config: &AccountInfo,
    swap_config: &AccountInfo,
) -> Result<bool> {
    let mut fee_rate = 9975u64;
    let mut result_amount = 0u64;

    // 从配置中读取参数
    let config_data = config.try_borrow_data()?;
    let iteration_limit = u64::from_le_bytes(config_data[0..8].try_into().unwrap());
    let liquidity_amount = u64::from_le_bytes(config_data[8..16].try_into().unwrap());
    let param1 = u64::from_le_bytes(config_data[16..24].try_into().unwrap());
    let param2 = u64::from_le_bytes(config_data[24..32].try_into().unwrap());

    // 读取交换配置
    let swap_data = swap_config.try_borrow_data()?;
    let swap_type = u32::from_le_bytes(swap_data[0..4].try_into().unwrap());

    // 根据交换类型调整费率
    if swap_type == 0 {
        fee_rate = 9975;
    } else if swap_type == 1 {
        fee_rate = 9900;
    } else {
        return Ok(false);
    }

    // 读取额外配置
    let swap_amount = if swap_type == 0 {
        u64::from_le_bytes(swap_data[8..16].try_into().unwrap())
    } else {
        u64::from_le_bytes(swap_data[16..24].try_into().unwrap())
    };

    if swap_amount > initial_amount {
        return Ok(false);
    }

    // 计算结果金额
    let remaining = initial_amount.saturating_sub(swap_amount);
    let threshold = 0x68db8bac710cc;

    result_amount = if threshold > remaining {
        (remaining * 10000) / fee_rate
    } else {
        (remaining / fee_rate) * 10000
    };

    // 验证结果
    if threshold > result_amount {
        result_amount = (result_amount * param2) / 10000;
    } else {
        result_amount = (result_amount / 10000) * param2;
    }

    // 检查最小值限制
    if result_amount <= 999 {
        return Ok(false);
    }

    // 更新结果
    if liquidity_amount == 0 {
        let mut temp_info = [0u8; 24];

        // 第一次报价计算
        let quote1 = get_quote_and_liquidity(&temp_info, &mut temp_info)?;

        get_liquidity(&temp_info, &mut temp_info)?;

        let quote2 = get_quote(&temp_info, quote1, !param1)?;

        // 第二次报价计算
        let mut temp_info2 = [0u8; 24];
        let quote3 = get_quote_and_liquidity(&temp_info2, &mut temp_info2)?;

        get_liquidity(&temp_info2, &mut temp_info2)?;

        let quote4 = get_quote(&temp_info2, quote3, !param1)?;

        let profit = quote4.saturating_sub(result_amount);

        // 更新配置数据
        let mut config_mut = config.try_borrow_mut_data()?;
        config_mut[0..8].copy_from_slice(&result_amount.to_le_bytes());
        config_mut[8..16].copy_from_slice(&profit.to_le_bytes());
    } else {
        let mut config_mut = config.try_borrow_mut_data()?;
        config_mut[0..8].copy_from_slice(&result_amount.to_le_bytes());
    }

    Ok(true)
}

/// 数学计算辅助函数 - function_12023
/// 根据输入值计算浮点数结果
/// 该函数实现了位操作来计算leading zeros并进行浮点转换
fn function_12023(value: u64) -> u64 {
    // 初始化返回值为0 (mov64 r0, 0)
    let mut result = 0u64;

    // 如果输入值为0, 直接返回0 (jeq r1, 0, lbb_12085)
    if value == 0 {
        return result;
    }

    // 以下实现类似于计算value的前导零数量
    // 通过位操作填充

    // 开始填充bits (mov64 r3, r1; rsh64 r3, 1; mov64 r2, r1; or64 r2, r3)
    let mut r2 = value;
    r2 |= value >> 1;

    // 继续填充 (mov64 r3, r2; rsh64 r3, 2; or64 r2, r3)
    r2 |= r2 >> 2;

    // 继续填充 (mov64 r3, r2; rsh64 r3, 4; or64 r2, r3)
    r2 |= r2 >> 4;

    // 继续填充 (mov64 r3, r2; rsh64 r3, 8; or64 r2, r3)
    r2 |= r2 >> 8;

    // 继续填充 (mov64 r3, r2; rsh64 r3, 16; or64 r2, r3)
    r2 |= r2 >> 16;

    // 继续填充 (mov64 r3, r2; rsh64 r3, 32; or64 r2, r3)
    r2 |= r2 >> 32;

    // 取反 (xor64 r2, -1)
    r2 = !r2;

    // 使用SWAR算法计算前导零数量
    // 设置掩码 (lddw r3, 0x5555555555555555)
    let mask1: u64 = 0x5555555555555555;

    // SWAR算法第一步 (mov64 r4, r2; rsh64 r4, 1; and64 r4, r3; sub64 r2, r4)
    let mut r2_tmp = r2;
    r2_tmp = (r2_tmp >> 1) & mask1;
    r2 -= r2_tmp;

    // 设置掩码 (lddw r4, 0x3333333333333333)
    let mask2: u64 = 0x3333333333333333;

    // SWAR算法第二步 (mov64 r3, r2; and64 r3, r4; rsh64 r2, 2; and64 r2, r4; add64 r3, r2)
    let mut r3 = r2 & mask2;
    r3 += (r2 >> 2) & mask2;

    // 继续计算 (mov64 r2, r3; rsh64 r2, 4; add64 r3, r2)
    r3 += r3 >> 4;

    // 设置掩码 (lddw r2, 0xf0f0f0f0f0f0f0f)
    let mask3: u64 = 0xf0f0f0f0f0f0f0f;

    // 应用掩码 (and64 r3, r2)
    r3 &= mask3;

    // 设置乘法常量 (lddw r2, 0x101010101010101)
    let magic: u64 = 0x101010101010101;

    // 应用乘法并右移 (mul64 r3, r2; rsh64 r3, 56)
    r3 = (r3 * magic) >> 56;

    // 左移输入值 (lsh64 r1, r3)
    let shifted_value = value << r3;

    // 计算指数偏移 (lsh64 r3, 52)
    let exp_offset = r3 << 52;

    // 提取尾数 (mov64 r2, r1; rsh64 r2, 11)
    let mut mantissa = shifted_value >> 11;

    // 设置结果 (mov64 r0, r2; sub64 r0, r3)
    result = mantissa - exp_offset;

    // 计算尾数修正 (xor64 r2, -1)
    let r2_neg = !mantissa;

    // 计算符号位 (lsh64 r1, 53; mov64 r3, r1; rsh64 r3, 63; and64 r3, r2; sub64 r1, r3)
    let sign_bit = (shifted_value << 53) >> 63;
    let correction = sign_bit & r2_neg;
    let corrected_value = (shifted_value << 53) - correction;

    // 调整结果 (rsh64 r1, 63; add64 r0, r1)
    result += corrected_value >> 63;

    // 添加常量 (lddw r1, 0x43d0000000000000; add64 r0, r1)
    // 添加浮点数偏移量 (1085)
    result += 0x43d0000000000000;

    result
}

/// 数学计算辅助函数 - function_11552
fn function_11552(value: u64, multiplier: u64) -> u64 {
    // 根据sBPF汇编，这个函数只是简单地调用了function_11768然后退出
    // call function_11768
    // exit
    function_11768(value, multiplier)
}

/// 数学计算辅助函数 - function_11768
/// 该函数实现乘法操作并处理可能的溢出
fn function_11768(value: u64, multiplier: u64) -> u64 {
    // 检查乘数是否为0，如果是则返回0 (jeq r2, 0, lbb_11952)
    if multiplier == 0 {
        return 0;
    }

    // 检查value是否为0，如果是则返回0 (jeq r1, 0, lbb_11952)
    if value == 0 {
        return 0;
    }

    // 提取value的低32位 (mov64 r6, r1; lsh64 r6, 32; rsh64 r6, 32)
    let value_lo = value & 0xFFFFFFFF;

    // 提取multiplier的低32位 (mov64 r7, r2; lsh64 r7, 32; rsh64 r7, 32)
    let multiplier_lo = multiplier & 0xFFFFFFFF;

    // 提取value的高32位 (mov64 r8, r1; rsh64 r8, 32)
    let value_hi = value >> 32;

    // 提取multiplier的高32位 (mov64 r9, r2; rsh64 r9, 32)
    let multiplier_hi = multiplier >> 32;

    // 计算低32位相乘的结果 (mul64 r6, r7)
    let lo_mul = value_lo * multiplier_lo;

    // 计算高32位和低32位的交叉乘积 (mul64 r7, r8; mul64 r9, r6)
    let cross_mul1 = multiplier_lo * value_hi;
    let cross_mul2 = multiplier_hi * value_lo;

    // 计算高32位相乘的结果 (mul64 r8, r9)
    let mut hi_mul = value_hi * multiplier_hi;

    // 提取lo_mul的高32位 (mov64 r0, r6; rsh64 r0, 32)
    let lo_mul_hi = lo_mul >> 32;

    // 计算中间结果的低32位 (add64 r6, r7; add64 r6, r9)
    let mut middle_result = lo_mul_hi + cross_mul1 + cross_mul2;

    // 检查加法溢出 (jge r6, r7, lbb_11842; add64 r8, 1)
    if middle_result < cross_mul1 {
        hi_mul += 1;
    }

    // 继续检查溢出 (jge r6, r9, lbb_11848; add64 r8, 1)
    if middle_result < cross_mul2 {
        hi_mul += 1;
    }

    // 提取中间结果的高32位 (mov64 r7, r6; rsh64 r7, 32)
    let middle_result_hi = middle_result >> 32;

    // 计算最终的高64位结果 (add64 r8, r7)
    let mut result_hi = hi_mul + middle_result_hi;

    // 构建最终的64位结果
    // 低32位来自lo_mul (mov64 r7, r6; lsh64 r7, 32)
    let low_part = (middle_result & 0xFFFFFFFF) << 32;

    // 保留lo_mul的低32位 (lsh64 r6, 32; rsh64 r6, 32)
    let lowest_part = lo_mul & 0xFFFFFFFF;

    // 组合成完整的低64位结果 (or64 r6, r7)
    let result_lo = lowest_part | low_part;

    // 最终结果为高64位
    result_hi
}

/// 数学计算辅助函数 - function_9815
/// 该函数处理IEEE-754浮点数格式，进行特殊范围检查和位操作
fn function_9815(value: u64) -> u64 {
    // 初始化返回值为0 (mov64 r0, 0)
    let mut result = 0u64;

    // 设置下界常量 0x3ff0000000000000 (lddw r2, 0x3ff0000000000000)
    let lower_bound: u64 = 0x3ff0000000000000;

    // 如果value小于下界，直接返回0 (jgt r2, r1, lbb_9838)
    if lower_bound > value {
        return result;
    }

    // 设置上界常量 0x43f0000000000000 (lddw r2, 0x43f0000000000000)
    let upper_bound: u64 = 0x43f0000000000000;

    // 如果value大于等于上界，进行特殊处理 (jgt r2, r1, lbb_9828)
    if upper_bound <= value {
        // 设置结果为-1 (mov64 r0, -1)
        result = u64::MAX; // -1 的无符号表示

        // 设置另一个边界值 0x7ff0000000000001 (lddw r2, 0x7ff0000000000001)
        let max_bound: u64 = 0x7ff0000000000001;

        // 如果value小于max_bound，返回-1，否则返回0 (jgt r2, r1, lbb_9838; mov64 r0, 0)
        if max_bound <= value {
            result = 0;
        }

        // 返回结果 (ja lbb_9838)
        return result;
    }

    // 处理在正常范围内的值 (lbb_9828)
    // 左移value 11位 (mov64 r0, r1; lsh64 r0, 11)
    result = value << 11;

    // 设置最高位为1 (lddw r2, 0x8000000000000000; or64 r0, r2)
    let high_bit: u64 = 0x8000000000000000;
    result |= high_bit;

    // 提取指数部分 (rsh64 r1, 52)
    let exponent = value >> 52;

    // 计算右移量 (mov64 r2, 62; sub64 r2, r1; and64 r2, 63)
    let shift_amount = (62 - exponent) & 63;

    // 根据计算出的位移量右移结果 (rsh64 r0, r2)
    result >>= shift_amount;

    // 返回最终结果
    result
}
