// 获取交换指令的优化实现
use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 交换指令账户结构
/// 定义了执行交换操作所需的所有账户
#[derive(Accounts)]
pub struct GetSwapInstruction<'info> {
    /// 待检查的程序ID账户
    /// 用于确定DEX类型（Raydium/PumpFun/Orca）
    /// CHECK: 在指令处理中进行安全性验证
    pub account_to_check: AccountInfo<'info>,

    /// 交易金额数据账户
    /// 存储用户希望交换的代币数量
    #[account(mut)]
    pub amount_data: Account<'info, AmountData>,

    /// 交易方向标志账户
    /// 指示是正向还是反向交换
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,

    /// 流动性池数据账户
    /// 包含当前池子的储备量信息
    /// CHECK: 在指令处理中验证池子数据的有效性
    pub pool_data: AccountInfo<'info>,

    /// 交换指令数据输出账户
    /// 用于存储生成的交换指令
    /// CHECK: 作为输出账户，不需要预先验证
    #[account(mut)]
    pub instruction_data: AccountInfo<'info>,
}

/// 优化版获取交换指令函数
///
/// # 功能特点
/// * 支持多个DEX（Raydium/PumpFun/Orca）
/// * 优化的程序ID类型判断
/// * 自动处理正反向交易
/// * 内置报价计算
///
/// # 返回值
/// * `Result<u64>` - 返回计算出的兑换金额
///
/// # 错误处理
/// * 无效的DEX类型将返回SwapError::InvalidDexType
pub fn get_swap_instruction_optimised(ctx: Context<GetSwapInstruction>) -> Result<u64> {
    let accounts = &ctx.accounts;

    // 获取程序ID类型
    // 使用as_ref()将Pubkey正确转换为字节切片
    let key_type = get_key_type_optimised(accounts.account_to_check.key.as_ref());

    // 根据不同的DEX类型生成对应的交换指令
    let (instruction_data, amount_out) = match key_type {
        0 => {
            // Raydium DEX处理
            // 使用简化的Raydium交换指令ID
            let raydium_ix = [9u8];
            let amount = accounts.amount_data.amount;
            (raydium_ix.to_vec(), amount)
        }
        1 => {
            // Pump.fun DEX处理
            // 使用特定的PumpFun交换指令ID
            let pump_fun_ix = [0xde, 0x33, 0x1e, 0xc4, 0xda, 0x5a, 0xbe, 0x8f];
            let amount = accounts.amount_data.amount;
            (pump_fun_ix.to_vec(), amount)
        }
        2 => {
            // Orca DEX处理
            let is_reverse = accounts.reverse.reverse;

            if is_reverse {
                // 反向交易处理
                // 使用Orca反向交换指令ID
                let orca_reverse_ix = [0xad, 0x83, 0x7f, 0x01, 0xa4, 0x85, 0xe6, 0x33];
                let amount = accounts.amount_data.amount;
                (orca_reverse_ix.to_vec(), amount)
            } else {
                // 正向交易处理
                // 获取交易报价
                let quote = pump_fun_get_quote(
                    accounts.pool_data.to_account_info(),
                    accounts.amount_data.amount,
                    false,
                )?;

                // 使用Orca正向交换指令ID
                let orca_ix = [0xea, 0xeb, 0xda, 0x01, 0x12, 0x3d, 0x06, 0x66];
                (orca_ix.to_vec(), quote)
            }
        }
        _ => {
            // 处理无效的DEX类型
            return Err(SwapError::InvalidDexType.into());
        }
    };

    // TODO: 实现指令数据的保存逻辑
    // ctx.accounts.instruction_data.data = instruction_data;

    // 返回计算得到的兑换金额
    Ok(amount_out)
}
