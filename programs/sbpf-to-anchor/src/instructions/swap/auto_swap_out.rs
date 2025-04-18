use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 自动交换输出指令的账户结构
/// 对应sBPF中的账户加载部分
#[derive(Accounts)]
pub struct AutoSwapOut<'info> {
    /// 全局状态账户
    /// 对应汇编中的[r7+0x0]加载
    ///check:
    pub global_state: AccountInfo<'info>,

    /// 交换状态账户
    /// 对应汇编中的[r7+0x10]加载
    ///check:
    pub swap_state: AccountInfo<'info>,

    /// 时钟账户
    /// 对应汇编中的sol_get_clock_sysvar调用
    pub clock: Sysvar<'info, Clock>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

/// 自动交换输出数据结构
/// 对应sBPF中的数据加载部分
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AutoSwapOutData {
    /// 输入金额
    /// 对应汇编中的[r1+0x0]加载
    pub input_amount: u64,

    /// 输出金额
    /// 对应汇编中的[r1+0x8]加载
    pub output_amount: u64,

    /// 验证者ID
    /// 对应汇编中的[r1+0x10]加载
    pub validator_id: u16,

    /// 是否反转计算
    /// 对应汇编中的[r1+0x12]加载
    pub is_inverted: bool,

    /// 是否启用三明治保护
    /// 对应汇编中的[r1+0x13]加载
    pub enable_sandwich: bool,

    /// 是否使用备用位置
    /// 对应汇编中的[r1+0x15]加载
    pub use_alt_location: bool,
}

/// 自动交换输出指令
/// 对应sBPF中的auto_swap_out函数
pub fn auto_swap_out(ctx: Context<AutoSwapOut>, data: AutoSwapOutData) -> Result<()> {
    // 检查验证者ID
    // 对应汇编中的验证者ID检查
    if data.validator_id != 0xFFFF {
        // 检查验证者是否在跟踪列表中
        require!(
            sandwich_tracker_is_in_validator_id(
                &ctx.accounts.global_state,
                &ctx.accounts.clock,
                data.validator_id
            )?,
            SwapError::InvalidValidatorId
        );
    }

    // 注册三明治跟踪器
    // 对应汇编中的sandwich_tracker_register调用
    sandwich_tracker_register(&ctx.accounts.global_state, &ctx.accounts.clock)?;

    // 反序列化交换数据
    // 对应汇编中的deserialize_swap调用
    let swap_data = deserialize_swap(
        &ctx.accounts.global_state,
        &ctx.accounts.swap_state,
        data.is_inverted,
        data.enable_sandwich,
    )?;

    // 获取报价和流动性
    // 对应汇编中的get_quote_and_liquidity调用
    let (quote, liquidity) = get_quote_and_liquidity(&ctx.accounts.swap_state, data.output_amount)?;

    // 计算滑点
    // 对应汇编中的滑点计算逻辑
    let slippage = if data.use_alt_location {
        let base_price = get_base_price(&ctx.accounts.swap_state)?;
        let alt_price = get_alt_price(&ctx.accounts.global_state)?;
        base_price + alt_price
    } else {
        let market_price = get_market_price(&ctx.accounts.swap_state)?;
        let base_price = get_base_price(&ctx.accounts.swap_state)?;
        base_price - market_price
    };

    // 更新输入金额
    // 对应汇编中的kpl_update_in_amount调用
    kpl_update_in_amount(
        &ctx.accounts.swap_state,
        &mut data.input_amount,
        data.is_inverted,
        data.use_alt_location,
        quote,
    )?;

    // 更新周期性卖出金额
    // 对应汇编中的periodic_sell_off_update_in_amount调用
    periodic_sell_off_update_in_amount(
        &ctx.accounts.swap_state,
        &ctx.accounts.clock,
        data.is_inverted,
        data.use_alt_location,
    )?;

    // 执行交换
    // 对应汇编中的execute_swap调用
    execute_swap(
        &ctx.accounts.global_state,
        data.output_amount,
        slippage,
        quote,
        liquidity,
        data.is_inverted,
        &ctx.accounts.clock,
    )?;

    // 更新后置运行状态
    // 对应汇编中的sandwich_update_backrun调用
    sandwich_update_backrun(
        &ctx.accounts.swap_state,
        data.output_amount,
        data.input_amount,
        quote,
        data.is_inverted,
        slippage,
    )?;

    // 更新代币数据
    // 对应汇编中的token_data_update_backrun调用
    token_data_update_backrun(
        &ctx.accounts.global_state,
        &ctx.accounts.swap_state,
        data.is_inverted,
    )?;

    // 检查最终金额
    // 对应汇编中的金额检查
    require!(
        data.input_amount <= get_max_input(&ctx.accounts.swap_state)?,
        SwapError::ExceedMaxInput
    );

    require!(
        get_min_output(&ctx.accounts.global_state)? <= data.output_amount,
        SwapError::InsufficientOutput
    );

    // 执行转账
    // 对应汇编中的transfer_调用
    transfer_(
        &ctx.accounts.global_state,
        &ctx.accounts.swap_state,
        data.output_amount - get_min_output(&ctx.accounts.global_state)?,
    )?;

    Ok(())
}
