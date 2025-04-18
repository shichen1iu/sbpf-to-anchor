use anchor_lang::prelude::*;

/// 三明治追踪器状态账户
#[account]
pub struct SandwichTracker {
    /// 追踪器计数
    pub count: u64,
    /// 身份数据数组
    pub identities: Vec<TrackerIdentity>,
    /// 领导者数据数组
    pub leaders: Vec<u16>,
}

/// 追踪器身份数据结构
/// 每个身份包含4个u64值，总共32字节
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq)]
pub struct TrackerIdentity {
    /// 第一个数据字段
    pub data1: u64,
    /// 第二个数据字段
    pub data2: u64,
    /// 第三个数据字段
    pub data3: u64,
    /// 第四个数据字段
    pub data4: u64,
}
