use anchor_lang::prelude::*;

/// Tipper状态结构
///
/// 基于sBPF汇编代码中的数据布局创建
/// 包含魔数字段和打赏记录
#[account]
#[derive(Default, Debug)]
pub struct TipperState {
    /// 魔数 - 对应汇编中的 0xfc74a147ce401273
    pub magic: u64,

    /// 预留空间 - 对应汇编中0x8到0x90的区域
    pub reserved: [u8; 136],

    /// 静态打赏金额 - 记录在汇编中的偏移0x90位置
    pub static_tip_amount: u64,

    /// 动态打赏金额 - 记录在汇编中的偏移0x88位置
    pub tip_amount: u64,

    /// 打赏标志 - 记录在汇编中的偏移0x98位置
    pub tip_flag: u8,
}

impl TipperState {
    /// 账户总大小（不包括8字节的discriminator）
    pub const SIZE: usize = 8 + 136 + 8 + 8 + 1;
}
