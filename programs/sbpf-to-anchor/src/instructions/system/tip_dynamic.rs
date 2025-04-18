use crate::error::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;
use solana_program::sysvar::clock::Clock;

/// 动态打赏的指令
///
/// 根据sBPF汇编代码，该指令实现从付款方向接收方转移代币
/// 并记录打赏信息到tipper账户
#[derive(Accounts)]
pub struct TipDynamic<'info> {
    /// 打赏数据账户，包含打赏参数
    pub tip_data: Signer<'info>,

    /// 付款方账户，必须是可变的以扣除金额
    #[account(mut)]
    ///check:
    pub payer: UncheckedAccount<'info>,

    /// 接收方账户，必须是可变的以接收金额
    #[account(mut)]
    ///check:
    pub recipient: UncheckedAccount<'info>,

    /// Tipper状态账户，记录打赏信息
    #[account(mut)]
    pub tipper_state: Account<'info, TipperState>,

    /// 系统时钟，用于获取当前时间
    pub clock: Sysvar<'info, Clock>,
}

/// 动态打赏功能
///
/// 计算打赏金额并从付款方向接收方转移代币
pub fn tip_dynamic(ctx: Context<TipDynamic>) -> Result<()> {
    // 获取账户数据
    let tip_data = &ctx.accounts.tip_data;
    let payer = &ctx.accounts.payer;
    let recipient = &ctx.accounts.recipient;
    let tipper_state = &mut ctx.accounts.tipper_state;
    let clock = &ctx.accounts.clock;

    // 从tip_data账户中提取参数
    let data_bytes = tip_data.to_account_info().data.borrow();

    // 对应汇编中的读取和异或操作 (0x2f5cc235403ddc54常量)
    let xor_constant = 0x2f5cc235403ddc54u64;

    // 提取并异或四个参数 (对应汇编中的ldxdw和xor64操作)
    let mut param1 = u64::from_le_bytes([
        data_bytes[0],
        data_bytes[1],
        data_bytes[2],
        data_bytes[3],
        data_bytes[4],
        data_bytes[5],
        data_bytes[6],
        data_bytes[7],
    ]) ^ xor_constant;

    let mut param2 = u64::from_le_bytes([
        data_bytes[8],
        data_bytes[9],
        data_bytes[10],
        data_bytes[11],
        data_bytes[12],
        data_bytes[13],
        data_bytes[14],
        data_bytes[15],
    ]) ^ xor_constant;

    let param3 = u64::from_le_bytes([
        data_bytes[16],
        data_bytes[17],
        data_bytes[18],
        data_bytes[19],
        data_bytes[20],
        data_bytes[21],
        data_bytes[22],
        data_bytes[23],
    ]) ^ xor_constant;

    let param4 = u64::from_le_bytes([
        data_bytes[24],
        data_bytes[25],
        data_bytes[26],
        data_bytes[27],
        data_bytes[28],
        data_bytes[29],
        data_bytes[30],
        data_bytes[31],
    ]) ^ xor_constant;

    // 记录参数信息 (对应汇编中的sol_log_64_)
    msg!("打赏参数: {} {} {} {}", param1, param2, param3, param4);

    // 获取当前时间戳 (对应汇编中的ldxdw r1, [r1+0x70])
    let timestamp = clock.unix_timestamp as u64;

    // 计算打赏金额 (对应汇编中的乘法和除法操作)
    // 如果param2不为0，则使用param2计算，否则使用时间戳
    if param2 == 0 {
        param2 = (timestamp * param1) / 10000;
    }

    // 计算基础打赏金额
    let mut tip_amount = (timestamp * param3) / 10000;

    // 如果param4大于当前时间戳，使用tip_amount作为最终打赏金额 (对应汇编中的条件判断)
    if param4 <= timestamp {
        tip_amount = param2;
    }

    msg!("计算的打赏金额: {}", tip_amount);

    // 获取付款方和接收方的lamports
    let mut payer_lamports = payer.lamports();
    let mut recipient_lamports = recipient.lamports();

    // 从付款方扣除金额 (对应汇编中的sub64操作)
    payer_lamports = payer_lamports
        .checked_sub(tip_amount)
        .ok_or(ErrorCode::InsufficientFunds)?;

    // 向接收方添加金额 (对应汇编中的add64操作)
    recipient_lamports = recipient_lamports
        .checked_add(tip_amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // 更新账户lamports
    **payer.lamports.borrow_mut() = payer_lamports;
    **recipient.lamports.borrow_mut() = recipient_lamports;

    // 更新tipper状态 (对应汇编中的stxdw和stxb操作)
    tipper_state.tip_amount = tip_amount;
    tipper_state.tip_flag = data_bytes[0x20]; // 从tip_data的0x20偏移处获取标志

    msg!("动态打赏完成，金额: {}", tip_amount);
    Ok(())
}
