use anchor_lang::prelude::*;

/// Raydium V4自动交换指令参数
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathAutoSwapParams {
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

/// Raydium V4自动输出交换指令参数
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathAutoSwapOutParams {
    /// 交换的代币金额
    pub amount_in: u64,

    /// 允许的最大滑点
    pub max_slippage: u64,

    /// 是否使用源代币作为输入
    pub is_source_input: bool,

    /// 是否验证交易
    pub validate_transaction: bool,

    /// 交易验证器ID
    pub validator_id: u16,
}

/// Raydium V4创建池参数
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathCreateRaydiumParams {
    /// 池标识符/编号
    pub pool_id: u32,

    /// 额外的配置数据
    pub config_data: u8,
}
