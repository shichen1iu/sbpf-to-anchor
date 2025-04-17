use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 静态小费账户结构
/// 用于处理固定金额小费的转账操作
#[derive(Accounts)]
pub struct TipStatic<'info> {
    /// 支付账户
    /// 用于支付小费的账户，必须是可变的以扣除小费
    #[account(mut)]
    pub from: AccountInfo<'info>,

    /// 接收账户
    /// 用于接收小费的账户，必须是可变的以接收小费
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

/// 静态小费处理函数
///
/// # 功能特点
/// * 支持固定金额小费转账
/// * 安全的数学运算
/// * 原子性转账操作
///
/// # 参数
/// * `ctx` - 包含账户信息的上下文
/// * `amount` - 固定小费金额（以lamports为单位）
///
/// # 错误处理
/// * 溢出检查：所有数学运算都使用checked操作
/// * 余额检查：确保账户有足够的SOL支付小费
///
/// # 安全考虑
/// * 使用checked运算防止溢出
/// * 原子性转账确保资金安全
pub fn tip_static(ctx: Context<TipStatic>, amount: u64) -> Result<()> {
    let from = &ctx.accounts.from;
    let to = &ctx.accounts.to;

    // 执行SOL转账
    // 1. 获取当前账户余额
    let from_lamports = from.lamports();
    let to_lamports = to.lamports();

    // 2. 更新账户余额
    // 使用checked运算确保不会发生溢出
    **from.try_borrow_mut_lamports()? = from_lamports
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;
    **to.try_borrow_mut_lamports()? = to_lamports.checked_add(amount).ok_or(ErrorCode::Overflow)?;

    Ok(())
}
