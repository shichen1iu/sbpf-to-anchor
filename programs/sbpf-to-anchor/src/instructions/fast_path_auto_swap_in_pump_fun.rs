use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// Pump.fun DEX快速路径自动交换的账户结构
/// 用于优化执行Pump.fun上的代币交换操作
#[derive(Accounts)]
pub struct FastPathAutoSwapInPumpFun<'info> {
    // === 通用账户组 ===
    /// 用户账户
    /// 交易发起者，必须签名交易
    #[account(mut)]
    pub user: Signer<'info>,

    /// Pump.fun流动性池数据账户
    /// 包含池子的状态信息和流动性数据
    /// CHECK: 在指令中进行验证
    pub pool_data: AccountInfo<'info>,

    /// 交易金额账户
    /// 存储交易数量和计算结果
    #[account(mut)]
    pub amount_data: Account<'info, AmountData>,

    // === Pump.fun特有账户组 ===
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

    // === 系统账户组 ===
    /// SPL代币程序
    /// 用于处理代币转账操作
    pub token_program: Program<'info, anchor_spl::token::Token>,

    /// 系统程序
    /// 用于系统级操作
    pub system_program: Program<'info, System>,
}

/// Pump.fun DEX快速路径自动交换函数
///
/// # 功能特点
/// * 快速路径优化
/// * 自动流动性检查
/// * 智能报价计算
/// * 三明治交易支持
///
/// # 执行流程
/// 1. 验证全局控制器
/// 2. 检查交易方向
/// 3. 获取流动性数据
/// 4. 验证流动性阈值
/// 5. 计算交换金额
/// 6. 更新状态数据
///
/// # 错误处理
/// * 无效的全局控制器
/// * 错误的交换方向
/// * 不足的流动性
/// * 无效的报价
pub fn fast_path_auto_swap_in_pump_fun(ctx: Context<FastPathAutoSwapInPumpFun>) -> Result<()> {
    let accounts = &ctx.accounts;

    // 验证全局控制器
    // 确保交易在有效的控制环境下执行
    if !accounts.has_global_controller() {
        return Err(SwapError::InvalidGlobalController.into());
    }

    // 验证交易方向
    // 确保交易方向符合预期设置
    if accounts.is_reverse() {
        return Err(SwapError::InvalidSwapDirection.into());
    }

    // 获取池子流动性数据
    // 查询当前可用的代币储备量
    let (reserve_a, reserve_b) = pump_fun_get_liquidity(
        accounts.pool_data.to_account_info(),
        accounts.amount_data.amount,
        false, // 使用正向交易路径
    )?;

    // 验证流动性阈值
    // 确保池子有足够的流动性执行交易
    let threshold = accounts.get_threshold();
    if reserve_b < threshold {
        return Err(SwapError::InsufficientLiquidity.into());
    }

    // 计算交换获得的代币数量
    // 基于当前池子状态计算预期输出
    let quote = pump_fun_get_quote(
        accounts.pool_data.to_account_info(),
        accounts.amount_data.amount,
        false, // 使用正向交易路径
    )?;

    // 验证报价有效性
    // 确保计算得到的报价大于零
    if quote == 0 {
        return Err(SwapError::InvalidQuote.into());
    }

    // 更新交易金额
    // 将计算得到的报价保存到金额账户
    ctx.accounts.amount_data.amount = quote;

    // TODO: 更新三明治交易数据
    // 在实际实现中需要调用sandwich_update_frontrun
    // 用于记录和更新三明治交易的状态

    // 记录成功日志
    msg!("Fast path auto swap in Pump Fun executed successfully");
    Ok(())
}
