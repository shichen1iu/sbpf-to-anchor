use super::get_quote_and_liquidity::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AutoSwapOut<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: 程序ID账户
    pub program_id: AccountInfo<'info>,

    /// CHECK: DEX池数据
    pub pool_data: AccountInfo<'info>,

    #[account(mut)]
    pub amount: Account<'info, AmountData>,

    #[account(mut)]
    pub dex_type: Account<'info, DexType>,

    #[account(mut)]
    pub is_reverse: Account<'info, ReverseFlag>,

    /// CHECK: DEX程序
    pub dex_program: AccountInfo<'info>,

    /// 三明治追踪器
    #[account(mut)]
    pub sandwich_tracker: Account<'info, SandwichTracker>,

    /// CHECK: 验证者ID账户,可选
    pub validator_id: AccountInfo<'info>,

    /// 交换输出数据
    #[account(mut)]
    pub output: Account<'info, SwapData>,

    /// CHECK: 代币A账户
    #[account(mut)]
    pub token_a_account: AccountInfo<'info>,

    /// CHECK: 代币B账户
    #[account(mut)]
    pub token_b_account: AccountInfo<'info>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}

// 执行自动化的代币交换操作
pub fn auto_swap_out(ctx: Context<AutoSwapOut>) -> Result<()> {
    // 获取当前时间
    let clock = Clock::get()?;
    let current_slot = clock.slot;

    // 验证交易者身份
    if ctx.accounts.validator_id.data_is_empty() == false {
        if !sandwich_tracker_is_in_validator_id(&ctx.accounts.sandwich_tracker, current_slot)? {
            return Err(SwapError::InvalidValidator.into());
        }
    }

    // 注册sandwich追踪
    sandwich_tracker_register(
        &ctx.accounts.sandwich_tracker,
        current_slot,
        ctx.accounts.user.key(),
    )?;

    // // 反序列化交换数据
    // let swap_data = deserialize_swap(
    //     &ctx.accounts.program_id,
    //     &ctx.accounts.pool_data,
    //     &mut ctx.accounts.output,
    // )?;

    // if !swap_data {
    //     return Err(SwapError::InvalidPoolState.into());
    // }

    // 获取报价和流动性
    let reverse = ctx.accounts.is_reverse.reverse;
    let (quote, reserve_a, reserve_b) = if ctx.accounts.dex_type.dex_type == 0 {
        raydium_get_quote_and_liquidity(
            ctx.accounts.pool_data.to_account_info(),
            ctx.accounts.amount.amount,
            reverse,
        )?
    } else {
        pump_fun_get_quote_and_liquidity(
            ctx.accounts.pool_data.to_account_info(),
            ctx.accounts.amount.amount,
            reverse,
        )?
    };

    // 检查流动性
    if quote == 0 {
        return Err(SwapError::InsufficientLiquidity.into());
    }

    // 执行交换指令
    execute_swap(
        &ctx.accounts.dex_program.key(),
        &[
            ctx.accounts.pool_data.to_account_info(),
            ctx.accounts.token_a_account.to_account_info(),
            ctx.accounts.token_b_account.to_account_info(),
        ],
        &[3u8], // 简化的指令数据
        &[],
    )?;

    // 更新sandwich状态 (backrun)
    sandwich_update_backrun(
        &ctx.accounts.sandwich_tracker,
        ctx.accounts.amount.amount,
        quote,
        reserve_a,
        reserve_b,
    )?;

    Ok(())
}
