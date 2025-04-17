use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 提现账户结构
/// 用于处理用户提取代币和SOL的账户结构
#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// 所有者账户
    /// 必须是签名者且有权限执行提现操作
    #[account(mut)]
    pub owner: Signer<'info>,

    /// 源代币账户
    /// 存储要提取的代币，必须由所有者拥有
    #[account(
        mut,
        constraint = source_account.owner == owner.key()
    )]
    pub source_account: Account<'info, AmountData>,

    /// 目标代币账户
    /// 接收提取代币的账户
    #[account(mut)]
    pub destination_token: Account<'info, anchor_spl::token::TokenAccount>,

    /// 目标SOL账户
    /// 接收租金返还的账户
    #[account(mut)]
    pub destination_sol: AccountInfo<'info>,

    /// SPL代币程序
    /// 用于执行代币转账操作
    pub token_program: Program<'info, anchor_spl::token::Token>,

    /// 系统程序
    /// 用于执行SOL转账操作
    pub system_program: Program<'info, System>,
}

/// 提现处理函数
///
/// # 功能特点
/// * 支持代币提现
/// * 租金返还
/// * 权限验证
/// * 余额检查
///
/// # 参数
/// * `ctx` - 包含账户信息的上下文
/// * `amount` - 提现金额
///
/// # 处理流程
/// 1. 验证提现权限
/// 2. 检查账户余额
/// 3. 执行代币转账
/// 4. 返还账户租金
/// 5. 更新账户状态
///
/// # 安全考虑
/// * 所有者权限验证
/// * 余额充足性检查
/// * 使用checked运算防止溢出
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let accounts = &ctx.accounts;

    // 验证提现权限
    // 确保操作者是账户所有者
    require!(
        accounts.owner.key() == accounts.source_account.owner,
        ErrorCode::InvalidAuthority
    );

    // 检查代币余额
    // 确保账户有足够的代币可供提现
    let balance = accounts.source_account.amount;
    require!(balance >= amount, SwapError::InsufficientBalance);

    // 执行代币转账
    // 使用CPI调用代币程序进行转账
    anchor_spl::token::transfer(
        CpiContext::new(
            accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: accounts.source_account.to_account_info(),
                to: accounts.destination_token.to_account_info(),
                authority: accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    // 执行SOL租金返还
    // 计算并转移最小租金余额
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(254); // 账户数据大小为254字节

    // 更新账户SOL余额
    // 使用checked运算确保安全
    **accounts.owner.try_borrow_mut_lamports()? = accounts
        .owner
        .lamports()
        .checked_sub(rent_lamports)
        .ok_or(ErrorCode::Overflow)?;
    **accounts.destination_sol.try_borrow_mut_lamports()? = accounts
        .destination_sol
        .lamports()
        .checked_add(rent_lamports)
        .ok_or(ErrorCode::Overflow)?;

    // 更新源账户代币余额
    // 使用checked运算确保安全
    accounts.source_account.amount = accounts
        .source_account
        .amount
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;

    // 记录提现成功
    msg!(
        "Withdrawal successful - Amount: {}, Rent: {}",
        amount,
        rent_lamports
    );
    Ok(())
}
