use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// Pump.fun快速路径自动交换账户创建结构
/// 用于初始化自动交换所需的账户和配置
#[derive(Accounts)]
pub struct FastPathCreatePumpFunAutoSwap<'info> {
    // === 权限账户组 ===
    /// 权限账户
    /// 必须是签名者且负责支付账户创建费用
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 交换账户
    /// 存储交换配置和状态的主账户
    /// 使用authority作为付款人初始化
    /// 空间大小为8字节的discriminator加上SwapAccount的大小
    #[account(init, payer = authority, space = 8 + SwapAccount::LEN)]
    pub swap_account: Account<'info, SwapAccount>,

    // === 系统账户组 ===
    /// 系统程序
    /// 用于创建新账户
    pub system_program: Program<'info, System>,

    /// 租金账户
    /// 用于验证账户余额是否足够支付租金
    pub rent: Sysvar<'info, Rent>,
}

/// Pump.fun快速路径自动交换入账户创建函数
///
/// # 功能特点
/// * 账户初始化
/// * 交换类型设置
/// * DEX类型配置
/// * 内部数据初始化
///
/// # 执行流程
/// 1. 账户标记初始化
/// 2. 设置交换类型
/// 3. 设置DEX类型
/// 4. 初始化数据区域
///
/// # 安全考虑
/// * 权限验证
/// * 租金检查
/// * 空间分配
pub fn fast_path_create_pump_fun_auto_swap_in(
    ctx: Context<FastPathCreatePumpFunAutoSwap>,
) -> Result<()> {
    let accounts = &ctx.accounts;

    // 初始化快速路径账户
    // 设置基本状态标志
    ctx.accounts.swap_account.initialized = true;

    // 设置交换类型
    // 2 表示 Pump Fun自动交换入
    ctx.accounts.swap_account.swap_type = 2; // Pump Fun Auto Swap In

    // 设置DEX类型
    // 1 表示 Pump Fun DEX
    ctx.accounts.swap_account.dex_type = 1; // 1 = Pump Fun

    // TODO: 初始化内部数据区域
    // 需要根据业务需求添加更多初始化逻辑

    // 记录成功日志
    msg!("Created Fast Path Pump Fun Auto Swap In account successfully");
    Ok(())
}
