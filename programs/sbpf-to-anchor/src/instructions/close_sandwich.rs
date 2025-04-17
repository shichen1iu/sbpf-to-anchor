use super::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 三明治交易关闭指令的账户结构
/// 用于安全地关闭三明治交易并回收资金
#[derive(Accounts)]
pub struct CloseSandwich<'info> {
    /// 操作权限账户
    /// 必须是经过验证的签名者
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 三明治交易数据账户
    /// 存储需要关闭的三明治交易信息
    /// CHECK: 三明治数据账户
    #[account(mut)]
    pub sandwich_data: AccountInfo<'info>,

    /// 资金接收账户
    /// 用于接收关闭账户后的SOL余额
    /// CHECK: 目标账户
    #[account(mut)]
    pub destination: AccountInfo<'info>,
}

/// 关闭三明治交易并回收资金的函数
///
/// # 功能特点
/// * 安全关闭三明治交易账户
/// * 转移剩余SOL到指定账户
/// * 记录关闭操作日志
/// * 防止溢出的安全检查
///
/// # 安全考虑
/// * 检查账户数据长度
/// * 使用checked_add防止溢出
/// * 使用try_borrow_mut保证安全访问
///
/// # 返回值
/// * `Result<()>` - 操作是否成功
///   - Ok(()): 关闭成功
///   - Err: 包含具体错误信息
pub fn close_sandwich(ctx: Context<CloseSandwich>) -> Result<()> {
    // 检查三明治账户是否有效
    // 只有当账户中有数据时才执行关闭操作
    if ctx.accounts.sandwich_data.data_len() > 0 {
        // 记录关闭操作的日志
        // 包含三明治账户和目标账户的公钥信息
        msg!(
            "Closing sandwich: {}, {}",
            ctx.accounts.sandwich_data.key(),
            ctx.accounts.destination.key()
        );

        // 执行账户关闭和SOL转移操作
        // 1. 获取账户中的SOL余额
        let rent_lamports = ctx.accounts.sandwich_data.lamports();

        // 2. 清空源账户的SOL余额
        **ctx.accounts.sandwich_data.try_borrow_mut_lamports()? = 0;

        // 3. 将SOL转移到目标账户
        // 使用checked_add防止金额溢出
        **ctx.accounts.destination.try_borrow_mut_lamports()? = ctx
            .accounts
            .destination
            .lamports()
            .checked_add(rent_lamports)
            .ok_or(ErrorCode::Overflow)?;
    }

    Ok(())
}
