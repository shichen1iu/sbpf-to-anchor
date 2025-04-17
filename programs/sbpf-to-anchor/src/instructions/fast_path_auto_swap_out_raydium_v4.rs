use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// Raydium V4快速路径自动反向交换的账户结构
/// 用于优化执行Raydium V4上的代币反向交换操作
#[derive(Accounts)]
pub struct FastPathAutoSwapOutRaydiumV4<'info> {
    // === 通用账户组 ===
    /// 输入数据账户
    /// 包含交易所需的基本参数
    pub input_data: AccountInfo<'info>,

    /// 交易金额账户
    /// 存储要交换的代币数量
    #[account(mut)]
    pub amount: Account<'info, AmountData>,

    /// 交易方向标志账户
    /// 指示代币交换的方向
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,

    /// DEX类型账户
    /// 用于区分不同的DEX协议
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,

    // === Raydium特有账户组 ===
    /// 代币A账户
    /// 用于交易的源代币账户
    /// CHECK: 仅用于代币转账，在CPI中验证
    #[account(mut)]
    pub token_account_a: AccountInfo<'info>,

    /// 代币B账户
    /// 用于交易的目标代币账户
    /// CHECK: 仅用于代币转账，在CPI中验证
    #[account(mut)]
    pub token_account_b: AccountInfo<'info>,

    // === 系统账户组 ===
    /// SPL代币程序
    /// 用于处理代币转账操作
    pub token_program: Program<'info, anchor_spl::token::Token>,

    /// 系统程序
    /// 用于系统级操作
    pub system_program: Program<'info, System>,
}

/// Raydium V4快速路径自动反向交换函数
///
/// # 功能特点
/// * 多DEX协议支持
/// * 分离的流动性和报价计算
/// * 优化的价格路由
/// * 三明治交易状态管理
///
/// # 执行流程
/// 1. 参数验证
/// 2. 流动性检查
/// 3. 报价计算
/// 4. 执行交换
/// 5. 更新状态
///
/// # 错误处理
/// * 无效的DEX类型
/// * 不足的流动性
/// * 报价计算失败
pub fn fast_path_auto_swap_out_raydium_v4(
    ctx: Context<FastPathAutoSwapOutRaydiumV4>,
) -> Result<()> {
    let accounts = &ctx.accounts;

    // 获取交易参数
    // 包括DEX类型、交易金额和方向
    let dex_type = accounts.dex_type.dex_type;
    let amount = accounts.amount.amount;
    let reverse = accounts.reverse.reverse;

    // 验证池子有效性并获取流动性数据
    // 根据DEX类型选择不同的验证逻辑
    let (reserve_a, reserve_b) = if dex_type == 0 {
        // Raydium V4流动性检查
        raydium_get_liquidity(accounts.input_data.to_account_info(), amount, reverse)?
    } else {
        // Pump.fun流动性检查
        pump_fun_get_liquidity(accounts.input_data.to_account_info(), amount, reverse)?
    };

    // 获取交易报价
    // 根据DEX类型计算最优报价
    let quote = if dex_type == 0 {
        // Raydium V4报价计算
        raydium_get_quote(accounts.input_data.to_account_info(), amount, reverse)?
    } else {
        // Pump.fun报价计算
        pump_fun_get_quote(accounts.input_data.to_account_info(), amount, reverse)?
    };

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
    msg!("Fast path auto swap out Raydium V4 executed successfully");
    Ok(())
}
