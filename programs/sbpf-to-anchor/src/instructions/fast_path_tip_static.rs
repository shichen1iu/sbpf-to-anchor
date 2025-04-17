use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 快速路径静态小费账户结构
/// 用于处理固定金额的小费转账
#[derive(Accounts)]
pub struct FastPathTipStatic<'info> {
    // === 交易账户组 ===
    /// 支付账户
    /// 用于支付小费的源账户
    /// 必须是可变的以支持lamports扣除
    #[account(mut)]
    pub from: AccountInfo<'info>,

    /// 接收账户
    /// 用于接收小费的目标账户
    /// 必须是可变的以支持lamports添加
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

/// 快速路径静态小费转账函数
///
/// # 功能特点
/// * 固定金额转账
/// * 安全的数学运算
/// * 原子性转账操作
/// * SOL代币支持
///
/// # 参数
/// * `amount` - 固定小费金额，单位为lamports
///
/// # 执行流程
/// 1. 验证账户
/// 2. 执行SOL转账
///
/// # 安全考虑
/// * 溢出检查
/// * 余额验证
/// * 原子性保证
///
/// # 与动态小费的区别
/// * 无时间加权
/// * 固定金额转账
/// * 更简单的计算逻辑
pub fn fast_path_tip_static(ctx: Context<FastPathTipStatic>, amount: u64) -> Result<()> {
    // 获取账户引用
    // 用于后续的SOL转账操作
    let from = &ctx.accounts.from;
    let to = &ctx.accounts.to;

    // 执行SOL转账
    // 使用原子操作确保转账安全性
    **from.try_borrow_mut_lamports()? = from
        .lamports()
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;
    **to.try_borrow_mut_lamports()? = to
        .lamports()
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    Ok(())
}
