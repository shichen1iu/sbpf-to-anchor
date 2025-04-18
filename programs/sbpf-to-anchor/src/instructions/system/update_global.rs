use crate::error::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

/// 更新全局状态的指令
///
/// 根据sBPF汇编代码，该指令负责更新全局状态数据
/// 汇编代码中通过检查标志位决定是否更新不同部分的数据
#[derive(Accounts)]
pub struct UpdateGlobal<'info> {
    /// 全局状态账户，必须是可变的
    #[account(mut)]
    pub global_state: Account<'info, GlobalState>,

    /// 全局状态更新数据
    pub global_data: Signer<'info>,
}

pub fn update_global(ctx: Context<UpdateGlobal>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    let global_data = &ctx.accounts.global_data;

    // 从global_data.data中提取相关字段的值
    let data_bytes = global_data.to_account_info().data.borrow();

    // 检查是否更新第一部分数据 (对应汇编中的 ldxb r1, [r6+0xb8] 检查)
    let update_first_part = data_bytes[0xb8] != 0;

    if update_first_part {
        // 从global_data中读取数据并更新global_state
        // 对应汇编中的:
        // ldxdw r1, [r6+0x10] / stxdw [r7+0x18], r1
        // ldxdw r1, [r6+0x8] / stxdw [r7+0x10], r1
        // ldxdw r1, [r6+0x0] / stxdw [r7+0x8], r1
        let offset0 = 0usize;
        let offset8 = 8usize;
        let offset16 = 16usize;

        let mut data_slice = [0u8; 8];

        // 更新第一个8字节数据
        data_slice.copy_from_slice(&data_bytes[offset0..offset0 + 8]);
        global_state.data1 = u64::from_le_bytes(data_slice);

        // 更新第二个8字节数据
        data_slice.copy_from_slice(&data_bytes[offset8..offset8 + 8]);
        global_state.data2 = u64::from_le_bytes(data_slice);

        // 更新第三个8字节数据
        data_slice.copy_from_slice(&data_bytes[offset16..offset16 + 8]);
        global_state.data3 = u64::from_le_bytes(data_slice);
    }

    // 检查是否更新第二部分数据 (对应汇编中的 ldxb r1, [r6+0xb9] 检查)
    let update_second_part = data_bytes[0xb9] != 0;

    if update_second_part {
        // 对应汇编中的:
        // add64 r7, 32
        // add64 r6, 24
        // mov64 r1, r7
        // mov64 r2, r6
        // mov64 r3, 160
        // call memcpy

        // 从global_data偏移24字节处复制160字节数据到global_state偏移32字节处
        global_state.copy_from_slice(&data_bytes[24..24 + 160], 32);
    }

    Ok(())
}
