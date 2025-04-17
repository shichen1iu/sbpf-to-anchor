use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 准备指令账户结构
/// 用于初始化和准备DEX交易所需的账户
#[derive(Accounts)]
pub struct Prepare<'info> {
    /// 支付账户
    /// 用于支付创建账户的手续费
    #[account(mut)]
    pub payer: Signer<'info>,

    /// DEX类型账户
    /// 存储当前使用的DEX类型信息
    pub dex_type: Account<'info, DexType>,

    /// 池子状态账户
    /// 记录流动性池的初始化状态
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    /// 代币A铸币账户
    /// 用于验证代币A的铸币权限
    pub token_a_mint: Account<'info, anchor_spl::token::Mint>,

    /// 代币A账户
    /// 用于存储代币A的余额和其他信息
    /// 使用UncheckedAccount因为账户可能尚未初始化
    #[account(mut)]
    pub token_a_account: UncheckedAccount<'info>,

    /// 代币数据账户
    /// 存储代币的额外元数据信息
    /// 使用UncheckedAccount因为账户可能尚未初始化
    #[account(mut)]
    pub token_data: UncheckedAccount<'info>,

    /// SPL代币程序
    /// 用于代币账户的创建和管理
    pub token_program: Program<'info, anchor_spl::token::Token>,

    /// 系统程序
    /// 用于创建新账户
    pub system_program: Program<'info, System>,

    /// 租金账户
    /// 用于计算账户所需的最小余额
    pub rent: Sysvar<'info, Rent>,
}

/// 准备函数
/// 初始化DEX交易所需的所有账户和数据
///
/// # 功能特点
/// * 检查池子初始化状态
/// * 创建代币账户
/// * 初始化代币数据
/// * 数据迁移支持
///
/// # 处理流程
/// 1. 检查池子是否已初始化
/// 2. 如果未初始化，执行以下步骤：
///    - 创建代币账户
///    - 创建代币数据
///    - 迁移代币数据
///
/// # 返回值
/// * `Result<()>` - 成功返回Ok(()), 失败返回错误
pub fn prepare(ctx: Context<Prepare>) -> Result<()> {
    // 获取池子初始化状态
    let initialized = ctx.accounts.pool_state.initialized;

    // 如果池子未初始化，执行初始化流程
    if !initialized {
        // 读取DEX类型，用于后续处理
        let dex_type = ctx.accounts.dex_type.dex_type;

        // 创建代币账户
        // 包括设置所有者和初始化账户数据
        create_token_account(
            &ctx.accounts.payer,
            &ctx.accounts.token_a_mint,
            &ctx.accounts.token_a_account,
            &ctx.accounts.token_program,
            &ctx.accounts.system_program,
            &ctx.accounts.rent,
        )?;

        // 创建代币数据账户
        // 初始化代币的元数据信息
        create_token_data_intern(
            &ctx.accounts.token_a_mint.to_account_info(),
            &ctx.accounts.payer.to_account_info(),
            &ctx.accounts.token_data.to_account_info(),
        )?;

        // 迁移代币数据
        // 将代币账户的数据迁移到新的数据结构中
        migrate_token_data(
            &ctx.accounts.token_a_account.to_account_info(),
            &ctx.accounts.token_data.to_account_info(),
        )?;
    }

    Ok(())
}
