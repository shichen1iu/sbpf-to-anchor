use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 三明治追踪器扩展的账户结构
/// 用于动态扩展追踪器账户的存储空间
#[derive(Accounts)]
pub struct ExtendSandwichTracker<'info> {
    /// 三明治追踪器账户
    /// 需要扩展存储空间的目标账户
    #[account(mut)]
    pub tracker: Account<'info, SandwichTracker>,
}

/// 扩展三明治追踪器账户空间的函数
///
/// # 功能特点
/// * 动态扩展账户存储空间
/// * 渐进式扩展策略
/// * 安全的内存操作
/// * 溢出保护
///
/// # 技术参数
/// * 最大账户大小: 285656字节
/// * 单次最大扩展: 10240字节
/// * 当前大小阈值: 285655字节
///
/// # 安全考虑
/// * 检查账户大小限制
/// * 使用saturating_sub防止下溢
/// * 使用checked_add防止上溢
/// * 安全的指针操作
pub fn extend_sandwich_tracker(ctx: Context<ExtendSandwichTracker>) -> Result<()> {
    // 获取追踪器账户的可变引用
    let tracker = &mut ctx.accounts.tracker;

    // 获取账户当前大小
    // 将数据长度转换为u64以便后续计算
    let current_size = tracker.to_account_info().data_len() as u64;

    // 检查是否达到最大大小限制
    // 如果当前大小超过285655字节，则不需要扩展
    if current_size > 285655 {
        return Ok(());
    }

    // 计算需要扩展的大小
    // 使用saturating_sub确保不会发生下溢
    let max_size = 285656; // 最大允许大小
    let mut extension_size = max_size.saturating_sub(current_size);

    // 限制单次扩展大小
    // 每次最多扩展10240字节，确保渐进式扩展
    if extension_size > 10240 {
        extension_size = 10240;
    }

    // 获取数据指针并计算新的大小
    // 使用checked_add防止大小计算时溢出
    let data_ptr = tracker.to_account_info().data.borrow_mut();
    let new_size = current_size
        .checked_add(extension_size)
        .ok_or(ErrorCode::Overflow)?;

    // 更新账户数据大小
    // 使用unsafe块直接修改内存中的大小值
    unsafe {
        // 获取大小字段的指针（位于数据区域前8字节）
        let size_ptr = (data_ptr.as_ptr() as *mut u64).offset(-1);
        // 更新大小值
        *size_ptr = new_size;
    }

    Ok(())
}
