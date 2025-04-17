use crate::instructions::sandwich::state::{SandwichTracker, TrackerIdentity};
use anchor_lang::prelude::*;

/// 写入三明治追踪器身份指令
#[derive(Accounts)]
pub struct WriteSandwichTrackerIdentities<'info> {
    /// 三明治跟踪器账户
    #[account(mut)]
    pub tracker: Account<'info, SandwichTracker>,
    /// 身份数据提供者
    pub identity_provider: AccountInfo<'info>,
}

/// 写入三明治追踪器身份数据
///
/// 此函数实现了与sBPF同等的逻辑:
/// 1. 从identity_provider账户加载身份数据
/// 2. 检查身份数据数量，如果为0则直接返回
/// 3. 遍历身份数组，将数据写入到tracker账户中
pub fn write_sandwich_tracker_identities(
    ctx: Context<WriteSandwichTrackerIdentities>,
) -> Result<()> {
    // 获取身份提供者账户数据
    let identity_provider = &ctx.accounts.identity_provider;
    let identity_data = identity_provider.try_borrow_data()?;

    // 读取身份数量 (从偏移量0x8处读取)
    let identity_count = if identity_data.len() >= 16 {
        let count_bytes = &identity_data[8..16];
        u64::from_le_bytes(count_bytes.try_into().unwrap())
    } else {
        return Ok(());
    };

    // 如果身份数量为0，直接返回
    if identity_count == 0 {
        return Ok(());
    }

    // 读取偏移量 (从偏移量0x0处读取)
    let offset = if identity_data.len() >= 8 {
        let offset_bytes = &identity_data[0..8];
        u64::from_le_bytes(offset_bytes.try_into().unwrap())
    } else {
        0
    };

    // 写入身份数据到跟踪器账户
    let mut tracker = &mut ctx.accounts.tracker;

    // 确保跟踪器有足够空间存储身份数据
    if tracker.identities.len() < (offset as usize + identity_count as usize) {
        tracker.identities.resize(
            offset as usize + identity_count as usize,
            TrackerIdentity::default(),
        );
    }

    // 遍历并写入身份数据
    for i in 0..identity_count {
        // 计算源数据位置 (从偏移量16开始，每32字节一个身份)
        let src_pos = 16 + (i as usize) * 32;

        // 确保源数据有足够的字节
        if src_pos + 32 > identity_data.len() {
            break;
        }

        // 读取数据并创建身份
        let data1 = u64::from_le_bytes(identity_data[src_pos..src_pos + 8].try_into().unwrap());
        let data2 =
            u64::from_le_bytes(identity_data[src_pos + 8..src_pos + 16].try_into().unwrap());
        let data3 = u64::from_le_bytes(
            identity_data[src_pos + 16..src_pos + 24]
                .try_into()
                .unwrap(),
        );
        let data4 = u64::from_le_bytes(
            identity_data[src_pos + 24..src_pos + 32]
                .try_into()
                .unwrap(),
        );

        // 写入到目标位置
        let dest_idx = (offset + i) as usize;
        tracker.identities[dest_idx] = TrackerIdentity {
            data1,
            data2,
            data3,
            data4,
        };
    }

    Ok(())
}
