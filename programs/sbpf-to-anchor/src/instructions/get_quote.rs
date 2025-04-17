use crate::utils::*;
use anchor_lang::prelude::*;

/// 获取报价主函数
/// 根据不同的DEX类型获取交易报价
///
/// # 参数
/// * `ctx` - 包含所需账户的上下文
/// * `dex_type` - DEX类型(0: Raydium, 1: Pump.fun)
///
/// # 返回值
/// * `Result<u64>` - 计算得到的报价金额
///
/// # 错误处理
/// * 无效的DEX类型将返回InvalidArgument错误
pub fn get_quote(ctx: Context<GetQuote>, dex_type: u8) -> Result<u64> {
    let accounts = &ctx.accounts;

    if dex_type == 0 {
        // Raydium DEX报价查询
        raydium_get_quote(
            accounts.input_data.to_account_info(),
            accounts.amount.amount,
            accounts.reverse.reverse,
        )
    } else if dex_type == 1 {
        // Pump.fun DEX报价查询
        pump_fun_get_quote(
            accounts.input_data.to_account_info(),
            accounts.amount.amount,
            accounts.reverse.reverse,
        )
    } else {
        Err(ProgramError::InvalidArgument.into())
    }
}

/// Raydium DEX报价计算函数
///
/// # 功能特点
/// * 基于池子储备量计算报价
/// * 包含0.25%的交易手续费
/// * 支持双向交易路径
///
/// # 参数
/// * `input_data` - 包含池子储备量的账户数据
/// * `amount` - 输入金额
/// * `reverse` - 交易方向标志
pub fn raydium_get_quote(input_data: AccountInfo, amount: u64, reverse: bool) -> Result<u64> {
    // 读取池子储备量数据
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // 计算手续费调整后的金额（0.25%手续费）
    let adjusted_amount = amount - (amount * 25) / 10000;

    let quote = if !reverse {
        // 正向交易路径：amount_a -> amount_b
        if amount_a == 0 {
            return Ok(0);
        }
        amount_b * adjusted_amount / amount_a
    } else {
        // 反向交易路径：amount_b -> amount_a
        if amount_b == 0 {
            return Ok(0);
        }
        amount_a * adjusted_amount / amount_b
    };

    Ok(quote)
}

/// Pump.fun DEX报价计算函数
///
/// # 功能特点
/// * 基于复杂数学公式计算报价
/// * 包含1%的交易手续费
/// * 使用临时存储优化计算
/// * 支持双向交易路径
///
/// # 参数
/// * `input_data` - 包含池子储备量的账户数据
/// * `amount` - 输入金额
/// * `reverse` - 交易方向标志
pub fn pump_fun_get_quote(input_data: AccountInfo, amount: u64, reverse: bool) -> Result<u64> {
    let input_bytes = input_data.try_borrow_data()?;

    // 反向交易路径处理
    if reverse {
        // 提取储备量数据
        let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

        // 计算调整后的金额和输出值
        let adjusted_amount = amount.saturating_add(reserve_a);
        let mut output = reserve_b.saturating_mul(reserve_a) / adjusted_amount;

        // 应用1%手续费
        output = output.saturating_sub(output / 100);

        return Ok(output);
    } else {
        // 正向交易路径处理
        // 创建临时存储用于初始计算
        let mut temp_storage = [100u64, 0u64];

        // 提取储备量并计算报价
        let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

        // 计算调整后的金额和输出值
        let adjusted_amount = reserve_a.saturating_add(amount);
        let mut output = reserve_a.saturating_mul(reserve_b) / adjusted_amount;

        // 应用1%手续费
        output = output.saturating_sub(output / 100);

        return Ok(output);
    }
}
