use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 创建授权账户的账户结构
/// 用于初始化和配置新的授权账户
#[derive(Accounts)]
pub struct CreateAuth<'info> {
    /// 操作权限账户
    /// 必须是经过验证的签名者，同时作为授权账户的支付者
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 新创建的授权账户
    /// 使用Anchor的init约束自动初始化账户
    /// 空间大小为8字节的discriminator加上AuthAccount的大小
    #[account(
        init,
        payer = authority,
        space = 8 + AuthAccount::LEN
    )]
    pub auth_account: Account<'info, AuthAccount>,

    /// 授权账户的种子值
    /// 用于生成唯一的授权账户
    pub seed: u8,

    /// 系统程序账户
    /// 用于创建新账户
    pub system_program: Program<'info, System>,

    /// 租金账户
    /// 用于计算账户所需的最小余额
    pub rent: Sysvar<'info, Rent>,
}

/// 创建并初始化授权账户的函数
///
/// # 功能特点
/// * 创建新的授权账户
/// * 设置账户的基本属性
/// * 写入特殊签名标识
/// * 建立权限关联
///
/// # 安全考虑
/// * 使用Anchor的账户约束
/// * 验证签名者权限
/// * 使用特殊签名防伪
///
/// # 初始化流程
/// 1. 创建新账户
/// 2. 设置基本属性
/// 3. 写入安全标识
///
/// # 返回值
/// * `Result<()>` - 返回创建操作是否成功
pub fn create_auth(ctx: Context<CreateAuth>) -> Result<()> {
    // 获取授权账户的可变引用
    let auth_account = &mut ctx.accounts.auth_account;

    // 设置授权账户的基本属性
    auth_account.seed = ctx.accounts.seed; // 设置种子值
    auth_account.authority = ctx.accounts.authority.key(); // 设置权限账户
    auth_account.initialized = true; // 标记为已初始化

    // 写入特殊签名标识
    // 0xbdf49c3c3882102f 作为安全验证标识
    auth_account.signature = 0xbdf49c3c3882102f;

    Ok(())
}
