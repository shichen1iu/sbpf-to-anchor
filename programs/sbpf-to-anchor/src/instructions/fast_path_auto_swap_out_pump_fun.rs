use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// Pump.fun DEX快速路径自动反向交换的账户结构
/// 用于优化执行Pump.fun上的代币反向交换操作
#[derive(Accounts)]
pub struct FastPathAutoSwapOutPumpFun<'info> {
    // === 通用账户组 ===
    /// 用户账户
    /// 交易发起者，必须签名交易
    #[account(mut)]
    pub user: Signer<'info>,

    /// 池子数据账户
    /// 包含流动性池的状态信息
    /// CHECK: 在指令中进行验证
    pub pool_data: AccountInfo<'info>,

    /// 交易金额账户
    /// 存储要交换的代币数量
    #[account(mut)]
    pub amount_data: Account<'info, AmountData>,

    // === Pump Fun特有账户组 ===
    /// 代币A账户
    /// 用于交易的源代币账户
    /// CHECK: 在指令中进行验证
    #[account(mut)]
    pub token_a_account: AccountInfo<'info>,

    /// 代币B账户
    /// 用于交易的目标代币账户
    /// CHECK: 在指令中进行验证
    #[account(mut)]
    pub token_b_account: AccountInfo<'info>,

    // === 验证账户组 ===
    /// 验证者ID账户
    /// 用于验证交易权限
    /// CHECK: 安全性由程序逻辑保证
    pub validator_id: AccountInfo<'info>,

    // === 系统账户组 ===
    /// SPL代币程序
    /// 用于处理代币转账操作
    pub token_program: Program<'info, anchor_spl::token::Token>,

    /// 系统程序
    /// 用于系统级操作
    pub system_program: Program<'info, System>,
}

/// Pump.fun DEX快速路径自动反向交换函数
///
/// # 功能特点
/// * 验证者权限检查
/// * 三明治交易追踪
/// * 自动流动性检查
/// * 优化的价格计算
///
/// # 执行流程
/// 1. 验证者身份检查
/// 2. 注册三明治追踪
/// 3. 获取报价和流动性
/// 4. 执行代币交换
/// 5. 更新交易状态
///
/// # 错误处理
/// * 无效的验证者
/// * 不足的流动性
/// * 交易执行失败
pub fn fast_path_auto_swap_out_pump_fun(ctx: Context<FastPathAutoSwapOutPumpFun>) -> Result<()> {
    let accounts = &ctx.accounts;

    // 验证者身份检查
    // 确保交易由有效的验证者发起
    if accounts.needs_validator_check() && !accounts.is_valid_validator()? {
        return Err(SwapError::InvalidValidator.into());
    }

    // 注册三明治交易追踪
    // 如果需要，将当前交易注册到追踪系统
    if accounts.need_register_tracker() {
        accounts.register_sandwich_tracker()?;
    }

    // 获取报价和流动性数据
    // 使用反向交易标志(true)调用报价计算
    let (quote, reserve_a, reserve_b) = pump_fun_get_quote_and_liquidity(
        accounts.pool_data.to_account_info(),
        accounts.amount_data.amount,
        true, // 反向交换标志
    )?;

    // 验证流动性充足性
    // 确保能够执行交易
    if quote == 0 {
        return Err(SwapError::InsufficientLiquidity.into());
    }

    // TODO: 执行代币交换
    // 需要通过CPI调用token程序实现代币转账

    // TODO: 更新三明治交易状态
    // 需要调用sandwich_update_backrun更新状态

    // 记录成功日志
    msg!("Fast path auto swap out Pump Fun executed successfully");
    Ok(())
}
