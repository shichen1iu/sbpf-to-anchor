use crate::utils::*;
use anchor_lang::prelude::*;

/// 池子有效性验证账户结构
/// 用于验证DEX流动性池的状态是否有效
#[derive(Accounts)]
pub struct IsValid<'info> {
    /// 输入数据账户
    /// 包含池子状态和流动性数据
    /// CHECK: 在指令处理中验证数据有效性
    pub input_data: AccountInfo<'info>,
}

/// 池子有效性验证主函数
///
/// # 功能特点
/// * 支持多个DEX的池子验证
/// * 根据DEX类型调用不同的验证逻辑
/// * 对未知DEX类型返回false
///
/// # 参数
/// * `ctx` - 包含输入数据账户的上下文
/// * `dex_type` - DEX类型(0: Raydium, 1: Pump.fun)
///
/// # 返回值
/// * `Result<bool>` - true表示池子有效，false表示池子无效
pub fn is_valid(ctx: Context<IsValid>, dex_type: u8) -> Result<bool> {
    if dex_type == 0 {
        // Raydium池子验证
        let accounts = &ctx.accounts;
        raydium_is_valid(accounts.input_data.to_account_info())
    } else if dex_type == 1 {
        // Pump.fun池子验证
        let accounts = &ctx.accounts;
        pump_fun_is_valid(accounts.input_data.to_account_info())
    } else {
        // 未知DEX类型默认返回false
        Ok(false)
    }
}

/// Raydium池子有效性验证函数
///
/// # 功能特点
/// * 验证池子双边流动性
/// * 检查最小储备金额要求
///
/// # 验证标准
/// * 代币A储备量 > 1000
/// * 代币B储备量 > 1000
///
/// # 参数
/// * `input_data` - 包含池子储备量的账户数据
pub fn raydium_is_valid(input_data: AccountInfo) -> Result<bool> {
    // 读取池子储备量数据
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // 验证两个代币的储备量是否都大于1000
    Ok(amount_a > 1000 && amount_b > 1000)
}

/// Pump.fun池子有效性验证函数
///
/// # 功能特点
/// * 验证池子双边流动性
/// * 计算并验证价格范围
/// * 使用数学函数进行复杂验证
///
/// # 验证步骤
/// 1. 检查最小储备金额
/// 2. 计算储备量的平方根
/// 3. 计算价格
/// 4. 验证价格是否在有效范围内
///
/// # 参数
/// * `input_data` - 包含池子储备量的账户数据
pub fn pump_fun_is_valid(input_data: AccountInfo) -> Result<bool> {
    // 读取池子数据
    let input_bytes = input_data.try_borrow_data()?;

    // 提取储备量数据
    let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
    let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

    // 验证最小储备金额要求
    if reserve_a <= 1000 || reserve_b <= 1000 {
        return Ok(false);
    }

    // 计算储备量的平方根
    let sqrt_a = function_12023(reserve_a);
    let sqrt_b = function_12023(reserve_b);

    // 计算池子价格
    let price = function_12129(sqrt_a, sqrt_b)?;

    // 验证价格是否在有效范围内
    // 0x42d6bcc41e900000 约等于 100,000
    // 0x4253ca6512000000 约等于 1,000,000
    let check1 = function_11552(price, 0x42d6bcc41e900000);
    let check2 = function_11519(check1, 0x4253ca6512000000);

    // 返回价格验证结果
    Ok(check2 > 0)
}
