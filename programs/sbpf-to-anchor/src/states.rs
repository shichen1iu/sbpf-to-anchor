// Data structures used in the program
#[account]
pub struct AmountData {
    pub amount: u64,
}
#[account]
pub struct ReverseFlag {
    pub reverse: bool,
}

#[account]
pub struct DexType {
    pub dex_type: u8,
}

#[account]
pub struct TokenAmounts {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[account]
pub struct IsTokenA {
    pub is_token_a: u8,
}

// 新增SwapData结构体
#[account]
#[derive(InitSpace)]
pub struct SwapData {
    pub dex_type: u8,
    pub is_reverse: bool,
    pub token_a_address: Pubkey,
    pub token_b_address: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub data: [u8; 1024], // 内部数据
}

impl SwapData {
    pub const LEN: usize = 1 + 1 + 32 + 32 + 8 + 8 + 8 + 8 + 1024;
}

// 存储区域数据结构
#[account]
#[derive(InitSpace)]
pub struct StorageData {
    pub data: [u8; 2048], // 用于临时存储和memcpy操作
}

impl StorageData {
    pub const LEN: usize = 2048;
}

#[account]
pub struct SandwichTracker {
    pub data: [u8; 24],
}

// 新数据结构定义
#[account]
#[derive(InitSpace)]
pub struct TipAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub tip_type: TipType,
    pub time_factor: u64,
}

impl TipAccount {
    pub const LEN: usize = 32 + 8 + 1 + 8;
}

#[account]
#[derive(InitSpace)]
pub struct AuthAccount {
    pub seed: u8,
    pub authority: Pubkey,
    pub initialized: bool,
    pub signature: u64,
}

impl AuthAccount {
    pub const LEN: usize = 1 + 32 + 1 + 8;
}

#[account]
#[derive(InitSpace)]
pub struct SandwichesCount {
    pub count: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum TipType {
    Static,
    Dynamic,
}

// 新增数据结构
#[account]
#[derive(InitSpace)]
pub struct GlobalUpdateData {
    pub update_fee_flag: bool,
    pub update_config_flag: bool,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub tipper_fee: u64,
    // 配置数据通常更大，这里简化
}

// 新添加的数据结构
#[account]
#[derive(InitSpace)]
pub struct PoolState {
    pub initialized: bool,
    pub dex_type: u8, // 0: Raydium, 1: PumpFun
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
}

impl PoolState {
    pub const LEN: usize = 1 + 1 + 8 + 8 + 8 + 8;
}

#[account]
#[derive(InitSpace)]
pub struct SwapAccount {
    pub initialized: bool,
    pub swap_type: u8,    // 2: Auto Swap In, 3: Auto Swap Out
    pub dex_type: u8,     // 0: Raydium, 1: Pump Fun
    pub data: [u8; 1800], // 内部数据
}

impl SwapAccount {
    pub const LEN: usize = 1 + 1 + 1 + 1800;

    // 检查交换类型是否有效
    pub fn is_valid_swap_type(&self) -> bool {
        matches!(self.swap_type, 2 | 3) // 2: Auto Swap In, 3: Auto Swap Out
    }

    // 检查DEX类型是否有效
    pub fn is_valid_dex_type(&self) -> bool {
        matches!(self.dex_type, 0 | 1) // 0: Raydium, 1: Pump Fun
    }

    // 获取内部数据的引用
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    // 获取可变内部数据的引用
    pub fn get_data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
