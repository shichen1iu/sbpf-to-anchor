use crate::utils::*;
use anchor_lang::prelude::*;

/// 计算上限值函数
/// 根据不同的交换类型和参数计算交易的上限值
///
/// # 参数说明
/// * `initial_amount` - 初始金额 (对应汇编中的 r1)
/// * `swap_config` - 交换配置 (对应汇编中的 r4)
/// * `state` - 状态信息 (对应汇编中的 r5)
pub fn calculate_upper_bound(
    initial_amount: u64,
    swap_config: &AccountInfo,
    state: &AccountInfo,
) -> Result<u64> {
    // 从状态中读取参数
    let state_data = state.try_borrow_data()?;
    let liquidity_amount = u64::from_le_bytes(state_data[16..24].try_into().unwrap());
    let param1 = u64::from_le_bytes(state_data[8..16].try_into().unwrap());

    // 读取交换类型
    let swap_data = swap_config.try_borrow_data()?;
    let swap_type = u32::from_le_bytes(swap_data[0..4].try_into().unwrap());

    // 常量定义
    const THRESHOLD: u64 = 0x68db8bac710cc;
    const FEE_RATE_1: u64 = 9975;
    const FEE_RATE_2: u64 = 9900;

    // 根据交换类型处理
    let result = match swap_type {
        0 => {
            // 类型0的处理逻辑
            if liquidity_amount == 0 {
                let swap_amount = u64::from_le_bytes(swap_data[16..24].try_into().unwrap());
                if swap_amount > initial_amount {
                    return Ok(0);
                }
                let remaining = initial_amount.saturating_sub(swap_amount);
                if THRESHOLD > remaining {
                    (remaining * 10000) / FEE_RATE_1
                } else {
                    (remaining / FEE_RATE_1) * 10000
                }
            } else {
                let swap_amount = u64::from_le_bytes(swap_data[8..16].try_into().unwrap());
                if swap_amount > initial_amount {
                    return Ok(0);
                }
                let remaining = initial_amount.saturating_sub(swap_amount);
                if THRESHOLD > remaining {
                    (remaining * 10000) / FEE_RATE_1
                } else {
                    (remaining / FEE_RATE_1) * 10000
                }
            }
        }
        1 => {
            // 类型1的处理逻辑
            if liquidity_amount == 0 {
                let swap_amount = u64::from_le_bytes(swap_data[16..24].try_into().unwrap());
                if swap_amount > initial_amount {
                    return Ok(0);
                }
                let remaining = initial_amount.saturating_sub(swap_amount);
                if THRESHOLD > remaining {
                    (remaining * 10000) / FEE_RATE_2
                } else {
                    (remaining / FEE_RATE_2) * 10000
                }
            } else {
                let swap_amount = u64::from_le_bytes(swap_data[8..16].try_into().unwrap());
                if swap_amount > initial_amount {
                    return Ok(0);
                }
                let remaining = initial_amount.saturating_sub(swap_amount);
                if THRESHOLD > remaining {
                    (remaining * 10000) / FEE_RATE_2
                } else {
                    (remaining / FEE_RATE_2) * 10000
                }
            }
        }
        _ => return Ok(0),
    };

    // 最终结果计算
    let final_result = if THRESHOLD > result {
        (result * param1) / 10000
    } else {
        (result / 10000) * param1
    };

    Ok(final_result)
}

/// 计算上限值函数 - 优化版本
/// 相比原始版本，简化了参数处理和计算逻辑
///
/// # 参数说明
/// * `initial_amount` - 初始金额 (对应汇编中的 r1)
/// * `swap_config` - 交换配置 (对应汇编中的 r2)
/// * `param` - 计算参数 (对应汇编中的 r3)
/// * `liquidity_amount` - 流动性数量 (对应汇编中的 r4)
pub fn calculate_upper_bound_optimised(
    initial_amount: u64,
    swap_config: &AccountInfo,
    param: u64,
    liquidity_amount: u64,
) -> Result<u64> {
    // 初始化费率为9975
    let mut fee_rate = 9975u64;

    // 读取交换类型
    let swap_data = swap_config.try_borrow_data()?;
    let swap_type = u32::from_le_bytes(swap_data[0..4].try_into().unwrap());

    // 根据交换类型调整费率
    if swap_type == 1 {
        fee_rate = 9900;
    } else if swap_type != 0 {
        return Ok(0);
    }

    // 根据流动性状态读取不同位置的交换金额
    let swap_amount = if liquidity_amount == 0 {
        u64::from_le_bytes(swap_data[16..24].try_into().unwrap())
    } else {
        u64::from_le_bytes(swap_data[8..16].try_into().unwrap())
    };

    // 检查交换金额是否超过初始金额
    if swap_amount > initial_amount {
        return Ok(0);
    }

    // 计算剩余金额
    let remaining = initial_amount.saturating_sub(swap_amount);

    // 定义阈值常量
    const THRESHOLD: u64 = 0x68db8bac710cc;

    // 第一次计算
    let mut result = if THRESHOLD > remaining {
        (remaining * 10000) / fee_rate
    } else {
        (remaining / fee_rate) * 10000
    };

    // 第二次计算
    result = if THRESHOLD > result {
        (result * param) / 10000
    } else {
        (result / 10000) * param
    };

    Ok(result)
}

/// 计算提示的最大金额
/// 根据提供的参数计算最大可能的交易金额
///
/// # 参数说明
/// * `initial_amount` - 初始金额 (对应汇编中的 r1)
/// * `hint_amount` - 提示金额 (对应汇编中的 r2)
/// * `fee_rate` - 费率 (对应汇编中的 r3)
/// * `param` - 计算参数 (对应汇编中的 r4)
pub fn calculate_hinted_max_amount(
    initial_amount: u64,
    hint_amount: u64,
    fee_rate: u64,
    param: u64,
) -> Result<u64> {
    // 检查提示金额是否超过初始金额
    if hint_amount > initial_amount {
        return Ok(0);
    }

    // 计算剩余金额
    let remaining = initial_amount.saturating_sub(hint_amount);

    // 定义阈值常量
    const THRESHOLD: u64 = 0x68db8bac710cc;

    // 第一次计算
    // 使用 10000 - fee_rate 作为基准费率
    let base_rate = 10000u64.saturating_sub(fee_rate);
    let mut result = if THRESHOLD > remaining {
        // 当剩余金额小于阈值时，先乘后除
        (remaining * 10000) / base_rate
    } else {
        // 当剩余金额大于阈值时，先除后乘
        (remaining / base_rate) * 10000
    };

    // 第二次计算
    result = if THRESHOLD > result {
        // 当结果小于阈值时，先乘后除
        (result * param) / 10000
    } else {
        // 当结果大于阈值时，先除后乘
        (result / 10000) * param
    };

    Ok(result)
}

/// 计算提示的最大金额 - 优化版本
/// 相比原始版本，简化了计算流程，减少了中间变量的使用
///
/// # 参数说明
/// * `initial_amount` - 初始金额 (对应汇编中的 r1)
/// * `hint_amount` - 提示金额 (对应汇编中的 r2)
/// * `fee_rate` - 费率 (对应汇编中的 r3)
/// * `param` - 计算参数 (对应汇编中的 r4)
pub fn calculate_hinted_max_amount_optimised(
    initial_amount: u64,
    hint_amount: u64,
    fee_rate: u64,
    param: u64,
) -> Result<u64> {
    // 初始化返回值为0 (mov64 r0, 0)
    let mut result = 0u64;

    // 检查提示金额是否超过初始金额 (jgt r2, r1, lbb_94)
    if hint_amount <= initial_amount {
        // 计算剩余金额 (sub64 r1, r2)
        let mut amount = initial_amount.saturating_sub(hint_amount);

        // 定义阈值常量 (lddw r2, 0x68db8bac710cc)
        const THRESHOLD: u64 = 0x68db8bac710cc;

        // 第一次计算
        if THRESHOLD > amount {
            // 先乘后除 (mul64 r1, 10000; div64 r1, r2)
            amount = (amount * 10000) / (10000 - fee_rate);
        } else {
            // 先除后乘 (div64 r1, r2; mul64 r1, 10000)
            amount = (amount / (10000 - fee_rate)) * 10000;
        }

        // 第二次计算
        if THRESHOLD > amount {
            // 先乘后除 (mul64 r1, r4; div64 r1, 10000)
            result = (amount * param) / 10000;
        } else {
            // 先除后乘 (div64 r1, 10000; mul64 r1, r4)
            result = (amount / 10000) * param;
        }
    }

    Ok(result)
}

/// 检查购买金额是否过大
/// 通过流动性和报价计算来判断购买金额是否超出限制
///
/// # 参数说明
/// * `quote_amount` - 报价金额 (对应汇编中的 r1)
/// * `pool_state` - 池状态 (对应汇编中的 r2)
/// * `max_amount` - 最大金额 (对应汇编中的 r3)
/// * `swap_direction` - 交换方向 (对应汇编中的 r4)
/// * `liquidity_amount` - 流动性数量 (对应汇编中的 r5)
pub fn is_buy_amount_too_big(
    quote_amount: u64,
    pool_state: &AccountInfo,
    max_amount: u64,
    swap_direction: bool,
    liquidity_amount: u64,
) -> Result<bool> {
    // 创建24字节的缓冲区 (add64 r9, -24)
    let mut liquidity_info = [0u8; 24];

    // 获取流动性信息 (call get_liquidity)
    get_liquidity(
        &liquidity_info,
        pool_state,
        liquidity_amount,
        &mut liquidity_info,
    )?;

    // 获取报价 (call get_quote)
    let quote = get_quote(
        &liquidity_info,
        pool_state.key().to_bytes(),
        liquidity_amount,
    )?;

    // 如果最大金额大于报价，检查有效性 (jgt r7, r1)
    if max_amount > quote {
        return Ok(true);
    }

    // 检查有效性并返回相反结果 (call is_valid; xor64 r0, 1)
    Ok(!is_valid(&liquidity_info)?)
}

/// 检查流动性信息是否有效
fn is_valid(liquidity_info: &[u8]) -> Result<bool> {
    // TODO: 实现流动性验证逻辑
    Ok(true)
}

///todo 
/// get_liquidity