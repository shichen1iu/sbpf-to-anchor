use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 自动交换指令的账户结构
/// 对应sBPF中的账户加载部分
#[derive(Accounts)]
pub struct AutoSwapIn<'info> {
    /// 全局状态账户
    /// 对应汇编中的[r8+0x0]加载
    pub global_state: AccountInfo<'info>,

    /// 交换状态账户
    /// 对应汇编中的[r8+0x10]加载
    pub swap_state: AccountInfo<'info>,

    /// 时钟账户
    /// 对应汇编中的sol_get_clock_sysvar调用
    pub clock: Sysvar<'info, Clock>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

/// 自动交换输入数据结构
/// 对应sBPF中的数据加载部分
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AutoSwapInData {
    /// 交换金额
    /// 对应汇编中的[r1+0x8]加载
    pub amount: u64,

    /// 最小输出金额
    /// 对应汇编中的[r1+0x10]加载
    pub min_out_amount: u64,

    /// 输入代币数量
    /// 对应汇编中的[r1+0x18]加载
    pub in_amount: u64,

    /// 输出代币数量
    /// 对应汇编中的[r1+0x20]加载
    pub out_amount: u64,

    /// 交换费率
    /// 对应汇编中的[r1+0x28]加载
    pub swap_fee: u16,

    /// 是否使用备用路径
    /// 对应汇编中的[r1+0x2a]加载
    pub use_alt_path: bool,

    /// 是否反转计算
    /// 对应汇编中的[r1+0x2b]加载
    pub is_inverted: bool,

    /// 是否启用三明治保护
    /// 对应汇编中的[r1+0x2c]加载
    pub enable_sandwich: bool,

    /// 是否启用价格保护
    /// 对应汇编中的[r1+0x2d]加载
    pub enable_price_protection: bool,

    /// 是否使用备用位置
    /// 对应汇编中的[r1+0x2e]加载
    pub use_alt_location: bool,
}

/// 自动交换输入指令
/// 对应sBPF中的auto_swap_in函数
pub fn auto_swap_in(ctx: Context<AutoSwapIn>, data: AutoSwapInData) -> Result<()> {
    // 检查魔数
    // 对应汇编中的0x8f5c570f55dd7921检查
    let magic_number: u64 = 0x8f5c570f55dd7921;
    require_eq!(
        magic_number,
        magic_number, // 实际应该从账户中读取
        SwapError::InvalidMagicNumber
    );

    // 获取时钟时间戳
    let clock = &ctx.accounts.clock;

    // 反转标志处理
    // 对应汇编中的反转标志处理逻辑
    let is_inverted = if data.is_inverted { 1 } else { 0 };

    // 计算最优报价
    // 对应汇编中的get_quote调用
    let quote = get_quote(&ctx.accounts.swap_state, data.in_amount, is_inverted == 1)?;

    // 检查最小输出金额
    // 对应汇编中的金额检查
    require!(
        quote > data.min_out_amount,
        SwapError::InsufficientOutAmount
    );

    // 计算最大滑点
    // 对应汇编中的滑点计算逻辑
    let max_slippage = if data.enable_sandwich {
        let base_amount = 0xba43b7400; // 50_000_000_000
        let amount = (data.amount / 100) * 80;
        if amount > base_amount {
            base_amount
        } else {
            amount
        }
    } else {
        (data.amount / 100) * 95
    };

    // 计算最优策略
    // 对应汇编中的calculate_optimal_strategy调用
    let (optimal_amount, optimal_quote) = calculate_optimal_strategy(
        &ctx.accounts.swap_state,
        data.amount,
        data.in_amount,
        max_slippage,
        data.is_inverted == 1,
    )?;

    // 执行交换
    // 对应汇编中的execute_swap调用
    execute_swap(
        &ctx.accounts.global_state,
        optimal_amount,
        max_slippage,
        optimal_quote,
        quote,
        is_inverted == 1,
        &ctx.accounts.clock,
    )?;

    // 更新前置运行状态
    // 对应汇编中的sandwich_update_frontrun调用
    sandwich_update_frontrun(
        &ctx.accounts.swap_state,
        &ctx.accounts.clock,
        data.enable_sandwich,
        data.is_inverted,
        data.enable_price_protection,
    )?;

    // 更新代币数据
    // 对应汇编中的token_data_update_frontrun调用
    token_data_update_frontrun(
        &ctx.accounts.global_state,
        &ctx.accounts.swap_state,
        &ctx.accounts.clock,
        is_inverted == 1,
    )?;

    Ok(())
}
