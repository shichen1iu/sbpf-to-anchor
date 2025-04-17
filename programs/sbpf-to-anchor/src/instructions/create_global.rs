use super::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 创建全局状态账户的账户结构
/// 用于初始化和配置系统的全局参数
#[derive(Accounts)]
pub struct CreateGlobal<'info> {
    /// 管理员账户
    /// 必须是经过验证的签名者，负责支付创建账户的费用
    #[account(mut)]
    pub admin: Signer<'info>,

    /// 全局状态账户
    /// 存储系统的关键配置参数和状态信息
    /// 空间大小为8字节的discriminator加上GlobalState的大小
    #[account(init, payer = admin, space = 8 + GlobalState::LEN)]
    pub global_state: Account<'info, GlobalState>,

    /// 系统程序账户
    /// 用于创建新账户
    pub system_program: Program<'info, System>,

    /// 租金账户
    /// 用于计算账户所需的最小余额
    pub rent: Sysvar<'info, Rent>,
}

/// 创建并初始化全局状态账户的函数
///
/// # 功能特点
/// * 创建全局状态账户
/// * 设置系统魔术数字
/// * 配置管理员权限
/// * 初始化费用参数
/// * 验证账户空间
///
/// # 安全考虑
/// * 使用魔术数字作为安全标识
/// * 验证账户大小限制
/// * 记录关键操作日志
///
/// # 初始化参数
/// * 默认小费费用: 0
/// * 默认费率: 0.1% (1/1000)
///
/// # 返回值
/// * `Result<()>` - 返回创建操作是否成功
pub fn create_global(ctx: Context<CreateGlobal>) -> Result<()> {
    let accounts = &ctx.accounts;

    // 定义系统魔术数字
    // 用作安全验证和版本控制的标识符
    const MAGIC_NUMBER: u64 = 0x8001c27439650c5c;

    // 获取全局状态账户的可变引用
    let global_state = &mut ctx.accounts.global_state;

    // 设置系统魔术数字
    // 用于验证账户的有效性和版本
    global_state.magic_number = MAGIC_NUMBER;

    // 设置系统管理员
    // 管理员拥有修改全局参数的权限
    global_state.admin = ctx.accounts.admin.key();

    // 初始化系统参数
    global_state.initialized = true; // 标记为已初始化
    global_state.tipper_fee = 0; // 设置默认小费费用为0
    global_state.fee_numerator = 1; // 设置费率分子为1
    global_state.fee_denominator = 1000; // 设置费率分母为1000，得到0.1%的默认费率

    // 验证账户空间是否足够
    // 确保分配的空间不超过限制
    let required_space = 8 + GlobalState::LEN; // 8字节用于账户判别器
    require!(
        required_space <= 320, // 账户大小上限为320字节
        ErrorCode::AccountTooSmall
    );

    // 记录创建成功的日志
    msg!("Global state created successfully");
    msg!("Admin: {}", global_state.admin);
    msg!("Magic number: {:#x}", global_state.magic_number);

    Ok(())
}
