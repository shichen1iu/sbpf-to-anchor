use crate::instructions::sandwich::state::SandwichTracker;
use anchor_lang::prelude::*;
use crate::states::*;


/// 写入三明治追踪器领导者指令
#[derive(Accounts)]
pub struct WriteSandwichTrackerLeaders<'info> {
    /// 三明治跟踪器账户
    #[account(mut)]
    pub tracker: Account<'info, SandwichTracker>,
    /// 领导者数据提供者
    ///check:
    pub leader_provider: AccountInfo<'info>,
}

/// 写入三明治追踪器领导者数据
///
/// 此函数实现了与sBPF同等的逻辑:
/// 1. 从leader_provider账户加载领导者数据
/// 2. 检查领导者数据数量，如果为0则直接返回
/// 3. 计算基础偏移量以及目标位置
/// 4. 遍历并写入领导者数据(每个为u16类型)
pub fn write_sandwich_tracker_leaders(ctx: Context<WriteSandwichTrackerLeaders>) -> Result<()> {
    // 获取领导者提供者账户数据
    let leader_provider = &ctx.accounts.leader_provider;
    let leader_data = leader_provider.try_borrow_data()?;

    // 读取领导者数量 (从偏移量0x8处读取)
    let leader_count = if leader_data.len() >= 16 {
        let count_bytes = &leader_data[8..16];
        u64::from_le_bytes(count_bytes.try_into().unwrap())
    } else {
        return Ok(());
    };

    // 如果领导者数量为0，直接返回
    if leader_count == 0 {
        return Ok(());
    }

    // 获取Tracker账户
    let mut tracker = &mut ctx.accounts.tracker;

    // 读取基础偏移量 (从偏移量0x0处读取)
    let base_offset = if leader_data.len() >= 8 {
        let offset_bytes = &leader_data[0..8];
        let offset = u64::from_le_bytes(offset_bytes.try_into().unwrap());
        offset << 1
    } else {
        0
    };

    // 计算目标位置起始偏移量 (基础偏移量 + 65560)
    let target_base_offset = base_offset + 65560;

    // 确保跟踪器有足够空间存储领导者数据
    let required_size = (target_base_offset + leader_count * 2) as usize;
    if tracker.leaders.len() < required_size {
        tracker.leaders.resize(required_size, 0);
    }

    // 遍历并写入领导者数据
    for i in 0..leader_count {
        // 计算源数据位置 (从偏移量16开始，每2字节一个领导者数据)
        let src_pos = 16 + (i as usize) * 2;

        // 确保源数据有足够的字节
        if src_pos + 2 > leader_data.len() {
            break;
        }

        // 读取领导者数据
        let leader = u16::from_le_bytes(leader_data[src_pos..src_pos + 2].try_into().unwrap());

        // 计算目标位置并写入
        let dest_idx = (target_base_offset + i * 2) as usize;
        tracker.leaders[dest_idx] = leader;
    }

    Ok(())
}
