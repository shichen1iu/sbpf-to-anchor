use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 创建三明治追踪器的账户结构
/// 用于初始化和配置三明治交易的追踪系统
#[derive(Accounts)]
pub struct CreateSandwichTracker<'info> {
    /// 管理员账户
    /// 必须是经过验证的签名者，负责支付创建账户的费用
    #[account(mut)]
    pub admin: Signer<'info>,

    /// 三明治追踪器账户
    /// 用于记录和追踪三明治交易的状态
    /// 空间大小为8字节的discriminator加上SandwichTracker的大小
    #[account(init, payer = admin, space = 8 + SandwichTracker::LEN)]
    pub tracker: Account<'info, SandwichTracker>,

    /// 系统程序账户
    /// 用于创建新账户
    pub system_program: Program<'info, System>,

    /// 租金账户
    /// 用于计算账户所需的最小余额
    pub rent: Sysvar<'info, Rent>,
}

/// 创建并初始化三明治追踪器账户的函数
///
/// # 功能特点
/// * 创建追踪器账户
/// * 设置魔术数字标识
/// * 记录时间槽信息
/// * 计算最大有效时间槽
/// * 处理租金转移
///
/// # 安全考虑
/// * 使用魔术数字验证
/// * 防止数值溢出
/// * 安全的租金计算
///
/// # 数据结构 (24字节)
/// * 0-7字节: 魔术数字
/// * 8-15字节: 当前时间槽
/// * 16-23字节: 最大时间槽
///
/// # 返回值
/// * `Result<()>` - 返回创建操作是否成功
pub fn create_sandwich_tracker(ctx: Context<CreateSandwichTracker>) -> Result<()> {
    let tracker: &mut Account<'_, SandwichTracker> = &mut ctx.accounts.tracker;

    // 初始化魔术数字
    // 用于验证追踪器账户的有效性
    let magic_number: u64 = 0x8373e4e400d333af;

    // 获取当前Solana网络的时间槽
    // 用于记录追踪器创建时间
    let clock = Clock::get()?;
    let current_slot = clock.slot;

    // 初始化追踪器数据缓冲区
    // 总大小为24字节，用于存储魔术数字和时间槽信息
    let mut data = [0u8; 24];

    // 写入魔术数字（前8字节）
    // 使用小端字节序
    data[0..8].copy_from_slice(&magic_number.to_le_bytes());

    // 写入当前时间槽（中间8字节）
    // 记录创建时的时间点
    data[8..16].copy_from_slice(&current_slot.to_le_bytes());

    // 计算并写入最大时间槽（后8字节）
    // 最大槽位为当前槽位乘以432000（约5天）
    let max_slot = current_slot
        .checked_mul(432000)
        .ok_or(ErrorCode::Overflow)?;
    data[16..24].copy_from_slice(&max_slot.to_le_bytes());

    // 设置追踪器的数据字段
    tracker.data = data;

    // 计算所需的最小租金余额
    // 确保账户有足够的SOL来维持存活
    let rent = Rent::get()?;
    let rent_exempt_lamports = rent.minimum_balance(285656);

    // 从支付者账户转移租金
    // 使用checked_sub防止溢出
    let payer_lamports = ctx.accounts.payer.lamports();
    **ctx.accounts.payer.try_borrow_mut_lamports()? = payer_lamports
        .checked_sub(rent_exempt_lamports)
        .ok_or(ErrorCode::Overflow)?;

    Ok(())
}
