use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 小费账户充值结构
/// 用于向小费账户充值SOL的账户结构
#[derive(Accounts)]
pub struct TopupTipper<'info> {
    /// 支付账户
    /// 用于支付充值金额的账户，必须是签名者
    #[account(mut)]
    pub payer: Signer<'info>,

    /// 小费账户
    /// 接收充值的小费账户，必须是可变的以接收SOL
    #[account(mut)]
    pub tipper: AccountInfo<'info>,

    /// 系统程序
    /// 用于执行SOL转账操作
    pub system_program: Program<'info, System>,
}

/// 小费账户充值函数
///
/// # 功能特点
/// * 支持向小费账户充值SOL
/// * 安全的数学运算
/// * 原子性转账操作
///
/// # 参数
/// * `ctx` - 包含账户信息的上下文
/// * `amount` - 充值金额（以lamports为单位）
///
/// # 错误处理
/// * 溢出检查：所有数学运算都使用checked操作
/// * 余额检查：确保支付账户有足够的SOL
///
/// # 安全考虑
/// * 要求支付账户必须签名
/// * 使用checked运算防止溢出
/// * 原子性转账确保资金安全
pub fn topup_tipper(ctx: Context<TopupTipper>, amount: u64) -> Result<()> {
    let payer: &Signer<'_> = &ctx.accounts.payer;
    let tipper = &ctx.accounts.tipper;

    // 执行SOL转账
    // 1. 获取当前账户余额
    let payer_lamports = payer.lamports();
    let tipper_lamports = tipper.lamports();

    // 2. 更新账户余额
    // 使用checked运算确保不会发生溢出
    **payer.try_borrow_mut_lamports()? = payer_lamports
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;
    **tipper.try_borrow_mut_lamports()? = tipper_lamports
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    Ok(())
}
