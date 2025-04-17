use super::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 批量关闭三明治交易并充值小费账户的账户结构
/// 用于批量处理三明治交易的关闭操作并管理小费账户
#[derive(Accounts)]
pub struct CloseSandwichesAndTopupTipper<'info> {
    /// 操作权限账户
    /// 必须是经过验证的签名者
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 三明治交易计数账户
    /// 记录需要处理的三明治交易数量
    pub sandwiches_count: Account<'info, SandwichesCount>,

    /// 三明治交易账户列表
    /// 存储所有需要关闭的三明治交易账户
    /// CHECK: 三明治账户列表
    #[account(mut)]
    pub sandwiches: UncheckedAccount<'info>,

    /// 小费接收账户
    /// 用于接收关闭账户后的SOL和额外充值
    /// CHECK: 小费接收账户
    #[account(mut)]
    pub tipper: AccountInfo<'info>,

    /// 支付账户
    /// 用于支付额外的小费充值
    /// CHECK: 支付者
    #[account(mut)]
    pub payer: Signer<'info>,

    /// 系统程序账户
    /// 用于执行SOL转账操作
    pub system_program: Program<'info, System>,
}

/// 批量关闭三明治交易并充值小费账户的函数
///
/// # 功能特点
/// * 批量关闭多个三明治交易账户
/// * 收集所有账户的租金
/// * 自动充值小费账户
/// * 安全的资金转移
///
/// # 安全考虑
/// * 限制最大处理数量
/// * 使用checked_add防止溢出
/// * 验证账户有效性
/// * 安全的资金转移
///
/// # 处理流程
/// 1. 计算租金总额
/// 2. 批量关闭账户
/// 3. 充值小费账户
///
/// # 返回值
/// * `Result<()>` - 操作是否成功
pub fn close_sandwiches_and_topup_tipper(
    ctx: Context<CloseSandwichesAndTopupTipper>,
) -> Result<()> {
    // 初始化金额计数器
    let mut total_amount = 0u64; // 总金额
    let mut total_rent = 0u64; // 总租金

    // 记录操作开始的日志
    msg!("Closing sandwiches and topping up tipper");

    // 设置批处理限制
    let max_sandwiches = 10; // 单次最多处理10个账户
    let sandwiches_count = ctx.accounts.sandwiches_count.count;

    // 检查是否在处理限制范围内
    if sandwiches_count <= max_sandwiches {
        // 计算单个账户的租金
        // 165是账户数据大小
        let rent_per_account = calculate_rent(165)?;

        // 遍历并处理每个三明治账户
        let mut collected_rent = 0;
        for i in 0..sandwiches_count {
            // 确保不越界访问
            if i < ctx.accounts.sandwiches.len() {
                let sandwich = &ctx.accounts.sandwiches[i];
                // 只处理非空账户
                if !sandwich.data_is_empty() {
                    // 记录当前处理的账户
                    msg!("Closing sandwich: {}", sandwich.key());

                    // 安全地累加租金
                    collected_rent = collected_rent
                        .checked_add(rent_per_account)
                        .ok_or(ErrorCode::Overflow)?;

                    // 关闭账户并转移资金到小费账户
                    close_account_intern(sandwich, &ctx.accounts.tipper.to_account_info())?;
                }
            }
        }

        // 更新总金额统计
        total_rent = collected_rent;
        total_amount = total_rent;
    }

    // 执行小费账户充值操作
    // 将收集的租金和额外充值转入小费账户
    topup_tipper_intern(
        total_amount,
        total_rent,
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.tipper.to_account_info(),
    )?;

    Ok(())
}
