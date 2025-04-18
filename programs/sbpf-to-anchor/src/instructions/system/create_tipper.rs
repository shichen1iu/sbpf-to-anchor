use crate::error::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;
use solana_program::sysvar::clock::Clock;


/// 创建Tipper的指令
///
/// 根据sBPF汇编代码，该指令创建Tipper账户并初始化
/// 设置魔数字段为固定值
#[derive(Accounts)]
pub struct CreateTipper<'info> {
    /// 用于支付创建账户所需的费用
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Tipper账户，将被初始化
    #[account(
        init, 
        payer = payer, 
        space = 8 + TipperState::SIZE, 
        seeds = [&[payer.key()[0]]], // 汇编中使用一个字节作为种子
        bump
    )]
    pub tipper_state: Account<'info, TipperState>,
    
    /// 系统程序，用于创建新账户
    pub system_program: Program<'info, System>,
}

/// 创建Tipper账户并初始化
///
/// 基于汇编中的create_tipper函数实现
/// 设置魔数为固定值
pub fn create_tipper(ctx: Context<CreateTipper>) -> Result<()> {
    let tipper_state = &mut ctx.accounts.tipper_state;
    
    // 设置魔数字段 (对应汇编中的 0xfc74a147ce401273)
    tipper_state.magic = 0xfc74a147ce401273;
    
    msg!("Tipper账户创建并初始化成功");
    Ok(())
}