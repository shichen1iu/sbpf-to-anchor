use anchor_lang::prelude::*;

/// PumpFun自动交换指令参数（输入形式）
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathAutoSwapInPumpParams {
    /// 最小获得的代币数量
    pub min_amount_out: u64,

    /// 交换的代币金额
    pub amount_in: u64,

    /// 是否使用源代币作为输入
    pub is_source_input: bool,

    /// 是否使用滑点保护
    pub use_slippage_protection: bool,

    /// 滑点保护百分比
    pub slippage_bps: u16,
}

/// PumpFun自动交换参数（输出形式）
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathAutoSwapOutPumpParams {
    /// 交换金额
    pub amount: u64,

    /// 目标金额
    pub target_amount: u64,

    /// 验证者ID
    pub validator_id: u16,
}

/// PumpFun快速路径创建自动交换参数
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathCreatePumpFunAutoSwapInParams {
    /// 配置标志
    pub config_flag: u8,

    /// 配置值
    pub config_value: u32,
}

/// PumpFun快速路径创建自动交换出池参数
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathCreatePumpFunAutoSwapOutParams {
    /// 配置标志
    pub config_flag: u8,

    /// 配置值
    pub config_value: u32,
}

/// PumpFun自动交换账户数据
///
/// 此结构定义了PumpFun自动交换功能所需的账户数据结构
#[account]
pub struct PumpFunAutoSwapAccount {
    /// 账户标识符
    pub identifier: [u8; 8],

    /// 交换数据缓冲区
    pub data_buffer: [u8; 672],

    /// 状态缓冲区
    pub state_buffer: [u8; 192],

    /// 额外缓冲区
    pub extra_buffer: [u8; 512],

    /// 配置值1
    pub config_value1: u64,

    /// 配置值2
    pub config_value2: u64,

    /// 程序ID引用1
    pub program_reference1: u64,

    /// 程序ID引用2
    pub program_reference2: u64,

    /// 配置标志
    pub config_flag: u32,
}
