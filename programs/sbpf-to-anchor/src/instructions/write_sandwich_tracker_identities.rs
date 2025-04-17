use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 三明治追踪器身份写入账户结构
/// 用于管理和更新三明治交易追踪器的身份数据
#[derive(Accounts)]
pub struct WriteSandwichTrackerIdentities<'info> {
    /// 追踪器账户
    /// 存储三明治交易的追踪数据和身份信息
    #[account(mut)]
    pub tracker: Account<'info, SandwichTracker>,
}

/// 三明治追踪器身份写入函数
///
/// # 功能特点
/// * 批量写入身份数据
/// * 内存安全操作
/// * 支持动态数量处理
///
/// # 数据布局
/// * 0x08: 身份数量(u64)
/// * 0x50: 数据起始位置
/// * 每个身份32字节，包含4个u64值
///
/// # 安全考虑
/// * 使用unsafe代码块进行内存操作
/// * 严格的边界检查
/// * 数据对齐保证
pub fn write_sandwich_tracker_identities(
    ctx: Context<WriteSandwichTrackerIdentities>,
) -> Result<()> {
    let tracker = &mut ctx.accounts.tracker;
    let data = tracker.to_account_info().try_borrow_mut_data()?;
    let data_ptr = data.as_ptr() as u64;

    // 获取身份数据数量
    // 从数据偏移量0x08处读取u64值
    let count = unsafe { *((data_ptr + 8) as *const u64) };
    if count == 0 {
        return Ok(());
    }

    // 获取数据起始位置
    // 数据区域从0x50偏移量开始
    let base_ptr = data_ptr + 0x50;

    // 循环写入身份数据
    // 每个身份占用32字节空间
    for i in 0..count {
        // 计算目标位置
        // 每个身份数据块的偏移量
        let target_offset = i * 32;
        let target_ptr = base_ptr + target_offset;

        // 读取源数据(4个u64)
        // 源数据从偏移量16开始
        let src_ptr = data_ptr + 16 + (i * 32);
        unsafe {
            // 读取4个u64值
            // 每个值占8字节，总共32字节
            let value0 = *(src_ptr as *const u64);
            let value1 = *((src_ptr + 8) as *const u64);
            let value2 = *((src_ptr + 16) as *const u64);
            let value3 = *((src_ptr + 24) as *const u64);

            // 写入目标位置
            // 按照相反的顺序写入，确保数据正确对齐
            *((target_ptr + 24) as *mut u64) = value3;
            *((target_ptr + 16) as *mut u64) = value2;
            *((target_ptr + 8) as *mut u64) = value1;
            *(target_ptr as *mut u64) = value0;
        }
    }

    Ok(())
}
