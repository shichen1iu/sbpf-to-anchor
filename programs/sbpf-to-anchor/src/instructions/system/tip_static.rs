use crate::error::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;
use solana_program::sysvar::clock::Clock;

/// 静态打赏的指令
///
/// 根据sBPF汇编代码，该指令实现固定金额的打赏
/// 从付款方向接收方转移固定金额代币
#[derive(Accounts)]
pub struct TipStatic<'info> {
    /// 打赏金额账户，包含固定打赏金额
    pub amount_account: Signer<'info>,

    /// 付款方账户，必须是可变的以扣除金额
    #[account(mut)]
    pub payer: UncheckedAccount<'info>,

    /// 接收方账户，必须是可变的以接收金额
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,

    /// Tipper状态账户，记录打赏信息
    #[account(mut)]
    pub tipper_state: Account<'info, TipperState>,
}

/// 静态打赏功能
///
/// 转移固定金额的代币从付款方到接收方
pub fn tip_static(ctx: Context<TipStatic>) -> Result<()> {
    // 获取账户数据
    let amount_account = &ctx.accounts.amount_account;
    let payer = &ctx.accounts.payer;
    let recipient = &ctx.accounts.recipient;
    let tipper_state = &mut ctx.accounts.tipper_state;

    // 从amount_account中获取打赏金额 (对应汇编中的tip_static中的ldxdw r2, [r2+0x0])
    let data_bytes = amount_account.to_account_info().data.borrow();
    let tip_amount = u64::from_le_bytes([
        data_bytes[0],
        data_bytes[1],
        data_bytes[2],
        data_bytes[3],
        data_bytes[4],
        data_bytes[5],
        data_bytes[6],
        data_bytes[7],
    ]);

    msg!("静态打赏金额: {}", tip_amount);

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

    // 更新tipper状态中的静态打赏金额 (对应汇编中的stxdw [r1+0x90], r2)
    tipper_state.static_tip_amount = tip_amount;

    msg!("静态打赏完成，金额: {}", tip_amount);
    Ok(())
}
