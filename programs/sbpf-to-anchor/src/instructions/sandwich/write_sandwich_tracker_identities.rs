use crate::instructions::sandwich::state::{SandwichTracker, TrackerIdentity};
use crate::states::*;
use anchor_lang::prelude::*;

/// 写入三明治追踪器身份指令
#[derive(Accounts)]
pub struct WriteSandwichTrackerIdentities<'info> {
    /// 三明治跟踪器账户
    #[account(
        mut,
        realloc = 8 + 8 + 4 + (offset as usize + identity_count as usize) * std::mem::size_of::<TrackerIdentity>(),
        realloc::payer = identity_provider,
        realloc::zero = true
    )]
    pub tracker: Account<'info, SandwichTracker>,
    /// 身份数据提供者
    #[account(mut)]
    pub identity_provider: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// 写入三明治追踪器身份数据
///
/// 1. 从identity_provider账户加载身份数据
/// 2. 检查身份数据数量，如果为0则直接返回
/// 3. 遍历身份数组，将数据写入到tracker账户中
pub fn write_sandwich_tracker_identities(
    ctx: Context<WriteSandwichTrackerIdentities>,
) -> Result<()> {
    // 获取身份提供者账户数据
    let identity_provider = &ctx.accounts.identity_provider;
    let identity_data = identity_provider.try_borrow_data()?;

    // 读取身份数量和偏移量
    let identity_count = if identity_data.len() >= 16 {
        let count_bytes = &identity_data[8..16];
        u64::from_le_bytes(count_bytes.try_into().unwrap())
    } else {
        return Ok(());
    };

    if identity_count == 0 {
        return Ok(());
    }

    let offset = if identity_data.len() >= 8 {
        let offset_bytes = &identity_data[0..8];
        u64::from_le_bytes(offset_bytes.try_into().unwrap())
    } else {
        0
    };

    // 账户空间已通过realloc处理好，可以直接写入数据
    let mut tracker = &mut ctx.accounts.tracker;

    // 遍历并写入身份数据
    for i in 0..identity_count {
        // 计算源数据位置
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
        if dest_idx >= tracker.identities.len() {
            tracker.identities.push(TrackerIdentity {
                data1,
                data2,
                data3,
                data4,
            });
        } else {
            tracker.identities[dest_idx] = TrackerIdentity {
                data1,
                data2,
                data3,
                data4,
            };
        }
    }

    Ok(())
}
