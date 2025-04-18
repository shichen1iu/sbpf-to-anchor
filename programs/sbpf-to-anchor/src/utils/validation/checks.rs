use crate::utils::dex::*;
use anchor_lang::prelude::*;

/// 检查流动性信息是否有效
/// 这个函数检查不同类型DEX的流动性有效性
///
/// # 参数说明
/// * `liquidity_info` - 流动性信息 (对应汇编中的 r1)
pub fn is_valid(liquidity_info: &[u8]) -> Result<bool> {
    // 初始化返回值为false (mov64 r0, 0)
    let mut result = false;

    // 读取DEX类型 (ldxw r2, [r1+0x0])
    let dex_type = u32::from_le_bytes(liquidity_info[0..4].try_into().unwrap());

    // 如果不是Raydium类型(0)，直接返回false (jne r2, 0)
    if dex_type == 0 {
        // 调用Raydium特定的验证函数 (call raydium_is_valid)
        // 跳过类型字段，使用后续的流动性数据
        result = raydium_is_valid(&liquidity_info[8..])?;
    }

    Ok(result)
}

/// 检查三明治交易追踪器中是否存在验证者ID
/// 这个函数检查给定的验证者ID是否在追踪器的记录范围内
///
/// # 参数说明
/// * `tracker_data` - 追踪器数据 (对应汇编中的 r1)
/// * `current_slot` - 当前时隙 (对应汇编中的 r2)
/// * `validator_id` - 验证者ID (对应汇编中的 r3)
pub fn sandwich_tracker_is_in_validator_id(
    tracker_data: &[u8],
    current_slot: u64,
    validator_id: u16,
) -> Result<bool> {
    // 初始化返回值为false (mov64 r0, 0)
    let mut result = false;

    // 读取追踪器的起始时隙 (ldxdw r4, [r1+0x10])
    let start_slot = u64::from_le_bytes(tracker_data[16..24].try_into().unwrap());

    // 计算结束时隙 = 起始时隙 + 432000 (add64 r6, 432000)
    let end_slot = start_slot + 432000;

    // 调整当前时隙，减去4 (add64 r7, -4)
    let adjusted_slot = current_slot.saturating_sub(4);

    // 初始化第一个ID记录位置为0 (mov64 r5, 0)
    let mut first_id_position = 0u64;

    // 第一个检查：检查起始时隙是否小于调整后的当前时隙且调整后的当前时隙小于结束时隙
    if start_slot <= adjusted_slot && adjusted_slot < end_slot {
        // 计算偏移量 (sub64 r7, r4; rsh64 r7, 1; and64 r7, r5)
        let offset = (adjusted_slot - start_slot) >> 1 & 0x7ffffffffffffffe;

        // 计算ID在数据中的位置 (add64 r8, r7; add64 r8, 65560)
        let id_position = offset + 65560;

        // 确保位置在有效范围内 (确保索引不超出数组边界)
        if id_position < tracker_data.len() as u64 - 1 {
            // 读取ID (ldxh r7, [r8+0x0])
            let id = u16::from_le_bytes(
                tracker_data[id_position as usize..id_position as usize + 2]
                    .try_into()
                    .unwrap(),
            );

            // 检查ID是否有效 (jgt r7, 2047)
            if id <= 2047 {
                first_id_position = id_position;
            }
        }
    }

    // 第二个检查：检查当前时隙是否在有效范围内
    if start_slot <= current_slot && current_slot < end_slot && first_id_position != 0 {
        // 计算当前时隙对应的偏移量
        let offset = (current_slot - start_slot) >> 1 & 0x7ffffffffffffffe;

        // 计算当前ID在数据中的位置
        let current_id_position = offset + 65560;

        // 确保位置在有效范围内
        if current_id_position < tracker_data.len() as u64 - 1 {
            // 读取当前ID
            let current_id = u16::from_le_bytes(
                tracker_data[current_id_position as usize..current_id_position as usize + 2]
                    .try_into()
                    .unwrap(),
            );

            // 检查当前ID是否有效
            if current_id <= 2047 {
                // 设置结果为true (mov64 r0, 1)
                result = true;

                // 读取第一个位置的ID
                let first_id = u16::from_le_bytes(
                    tracker_data[first_id_position as usize..first_id_position as usize + 2]
                        .try_into()
                        .unwrap(),
                );

                // 如果任一ID与验证者ID匹配，则结果为false
                if first_id == validator_id || current_id == validator_id {
                    result = false;
                }
            }
        }
    }

    // 确保结果是布尔值 (and64 r0, 1)
    Ok(result)
}
