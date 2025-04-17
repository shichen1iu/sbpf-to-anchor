use crate::instructions::sandwich::state::SandwichTracker;
use anchor_lang::prelude::*;

/// 扩展三明治追踪器指令
#[derive(Accounts)]
pub struct ExtendSandwichTracker<'info> {
    /// 三明治跟踪器账户
    #[account(mut)]
    pub tracker: Account<'info, SandwichTracker>,
}

/// 扩展三明治追踪器
///
/// 此函数实现了与sBPF同等的逻辑:
/// 1. 检查计数是否超过285655
/// 2. 如果没有超过，计算需要增加的量(285656减去当前值)
/// 3. 如果增加量超过10240，则限制为10240
/// 4. 将计算出的增加量加到原值上
pub fn extend_sandwich_tracker(ctx: Context<ExtendSandwichTracker>) -> Result<()> {
    // 获取当前计数值
    let current_count = ctx.accounts.tracker.count;

    // 如果计数已经超过285655，则无需增加
    if current_count > 285655 {
        return Ok(());
    }

    // 计算增加量，上限为10240
    let mut increment = 285656_u64.saturating_sub(current_count);
    if increment > 10240 {
        increment = 10240;
    }

    // 更新计数器
    ctx.accounts.tracker.count = ctx.accounts.tracker.count.saturating_add(increment);

    Ok(())
}
