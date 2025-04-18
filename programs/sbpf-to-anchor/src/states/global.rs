use anchor_lang::prelude::*;

/// 全局状态结构
///
/// 基于sBPF汇编代码中的数据布局创建
/// 包含两部分数据：前24字节的基本数据和后续160字节的扩展数据
#[account]
#[derive(Default, Debug)]
pub struct GlobalState {
    /// 魔数 - 对应汇编中的 0x8001c27439650c5c
    pub magic: u64,

    /// 基本数据1 - 对应汇编中[r7+0x8]位置的数据
    pub data1: u64,

    /// 基本数据2 - 对应汇编中[r7+0x10]位置的数据
    pub data2: u64,

    /// 基本数据3 - 对应汇编中[r7+0x18]位置的数据
    pub data3: u64,

    /// 扩展数据 - 对应汇编中通过memcpy复制的160字节数据
    /// 从偏移32字节位置开始
    pub extended_data: [u8; 160],
}

impl GlobalState {
    /// 账户总大小（不包括8字节的discriminator）
    pub const SIZE: usize = 8 + 8 + 8 + 8 + 160;

    /// 从指定偏移处复制数据到扩展数据区域
    pub fn copy_from_slice(&mut self, data: &[u8], offset: usize) {
        if offset == 32 && data.len() <= 160 {
            self.extended_data[..data.len()].copy_from_slice(data);
        }
    }
}
