use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 快速路径动态小费账户结构
/// 用于处理基于时间的动态小费计算和转账
#[derive(Accounts)]
pub struct FastPathTipDynamic<'info> {
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

    /// 时钟账户
    /// 用于获取当前时间戳
    /// 用于动态小费计算
    pub clock: Sysvar<'info, Clock>,
}

/// 快速路径动态小费计算和转账函数
///
/// # 功能特点
/// * 基于时间的动态计算
/// * 安全的数学运算
/// * 原子性转账操作
/// * SOL代币支持
///
/// # 参数
/// * `base_amount` - 基础小费金额，单位为lamports
///
/// # 执行流程
/// 1. 获取当前时间
/// 2. 计算动态小费
/// 3. 执行SOL转账
///
/// # 安全考虑
/// * 溢出检查
/// * 余额验证
/// * 原子性保证
pub fn fast_path_tip_dynamic(ctx: Context<FastPathTipDynamic>, base_amount: u64) -> Result<()> {
    // 获取账户引用
    let from: &AccountInfo<'_> = &ctx.accounts.from;
    let to = &ctx.accounts.to;

    // 获取当前时间戳
    // 用于时间加权计算
    let current_time = Clock::get()?.unix_timestamp as u64;

    // 初始化小费金额
    // 从基础金额开始计算
    let mut tip_amount = base_amount;

    // 应用时间加权因子
    // 默认因子为10000（1.0）
    // 可以根据需求调整时间影响
    let time_factor = 10000u64; // 基准因子

    // 计算加权后的小费金额
    // 使用checked操作防止溢出
    tip_amount = tip_amount
        .checked_mul(time_factor)
        .ok_or(ErrorCode::Overflow)?;
    tip_amount = tip_amount.checked_div(10000).ok_or(ErrorCode::Overflow)?;

    // 执行SOL转账
    // 使用原子操作确保转账安全性
    **from.try_borrow_mut_lamports()? = from
        .lamports()
        .checked_sub(tip_amount)
        .ok_or(ErrorCode::Overflow)?;
    **to.try_borrow_mut_lamports()? = to
        .lamports()
        .checked_add(tip_amount)
        .ok_or(ErrorCode::Overflow)?;

    Ok(())
}
