use crate::error::ErrorCode;
use crate::instructions::dex::pump::{
    helpers::{
        apply_slippage, calculate_adjusted_amount, get_quote, sandwich_update_frontrun,
        token_data_update_frontrun,
    },
    state::FastPathAutoSwapInPumpParams,
};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, pubkey::Pubkey},
};

/// PumpFun快速路径自动交换指令（输入形式）
///
/// 此指令用于在PumpFun协议上执行自动交换操作（交换入池子）
/// 包含价格验证、滑点保护和对三明治攻击的前置运行处理
#[derive(Accounts)]
pub struct FastPathAutoSwapInPumpFun<'info> {
    /// 交换交易的发起者
    #[account(mut)]
    pub signer: Signer<'info>,

    /// PumpFun池账户
    #[account(mut)]
    pub pump_pool: AccountInfo<'info>,

    /// 源代币账户
    #[account(mut)]
    pub source_token_account: AccountInfo<'info>,

    /// 目标代币账户
    #[account(mut)]
    pub destination_token_account: AccountInfo<'info>,

    /// PumpFun程序ID
    /// @executable
    pub pump_program: AccountInfo<'info>,

    /// 报价账户
    #[account(mut)]
    pub quote_account: AccountInfo<'info>,

    /// 用于更新前置运行数据的账户
    #[account(mut)]
    pub token_data_account: AccountInfo<'info>,

    /// 三明治状态账户
    #[account(mut)]
    pub sandwich_state: AccountInfo<'info>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

/// 在PumpFun上执行自动交换（输入形式）
///
/// 此函数与sBPF中的fast_path_auto_swap_in_pump_fun对应
/// 处理价格检查、执行交换以及更新前置运行相关状态
pub fn fast_path_auto_swap_in_pump_fun(
    ctx: Context<FastPathAutoSwapInPumpFun>,
    params: FastPathAutoSwapInPumpParams,
) -> Result<()> {
    // 设置初始签名数据
    // 从sBPF代码看，这里设置了一个特定的签名
    let signature = 0x8f5c570f55dd7921u64;

    // 检查条件 - 从sBPF代码看，只有在特定条件下才执行交换
    // 对应sBPF代码中的判断逻辑：r1不为0且r1为0时才执行
    let enable_check1 = true; // 假设对应sBPF中的 0x4000082d8 位置的值
    let enable_check2 = false; // 假设对应sBPF中的 0x40000837c 位置的值

    if !enable_check1 || enable_check2 {
        // 不满足条件，直接返回
        return Ok(());
    }

    // 获取源和目标代币当前价格
    // 对应sBPF代码中从内存位置0x400067a48和0x400067a50读取的值
    let current_price = 0u64; // 当前源代币价格
    let expected_price = 0u64; // 当前目标代币价格

    // 获取最大允许的价格差异
    // 对应sBPF代码中从内存位置0x4000b8d48读取的值
    let max_price_diff = 0u64;

    // 如果当前价格超过预期价格加上最大差异，则退出
    if expected_price > max_price_diff + current_price {
        return Ok(());
    }

    // 计算价格差异
    let price_diff = max_price_diff.saturating_sub(expected_price);

    // 根据价格差异调整交易量
    // 对应sBPF代码中的滑点计算逻辑
    let is_large_amount = price_diff > 1_000_000_000_000_000;
    let mut adjusted_diff = calculate_adjusted_amount(price_diff, is_large_amount);

    // 如果调整后的差异为0，则退出
    if adjusted_diff == 0 {
        return Ok(());
    }

    // 应用滑点保护
    // 对应sBPF代码中从内存位置0x4000b8d60读取的滑点值
    let slippage = params.slippage_bps;
    let mut slippage_adjusted_amount = adjusted_diff;

    // 根据滑点计算最终金额
    slippage_adjusted_amount = apply_slippage(adjusted_diff, slippage, is_large_amount);

    // 处理其他滑点逻辑
    // 对应sBPF代码中检查0x4000b8d62的条件
    let use_slippage_protection = params.use_slippage_protection;
    if !use_slippage_protection {
        // 额外的滑点保护逻辑
        let mut additional_amount = 0;
        let min_amount = 0u64; // 假设对应sBPF代码中的某个内存位置的值

        if expected_price > min_amount {
            let diff = min_amount.saturating_sub(expected_price);
            let adjusted = calculate_adjusted_amount(diff, diff > 1_000_000_000_000_000);
            additional_amount = adjusted;
        }

        let min_output = 0u64; // 假设对应sBPF代码中的某个内存位置的值
        let mut final_amount = slippage_adjusted_amount;

        if additional_amount > slippage_adjusted_amount {
            let combined = additional_amount + min_output;
            let half = combined / 2;
            final_amount = half;
        }

        if min_output > slippage_adjusted_amount {
            slippage_adjusted_amount = final_amount;
        }
    }

    // 获取源代币和目标代币的数量
    // 对应sBPF代码中从内存位置0x40006f4d8和0x40006cc20读取的值
    let source_amount = 0u64;
    let dest_amount = 0u64;

    // 安全检查：确保输出金额不低于最小期望值（95%的源金额）
    let minimum_out = source_amount.saturating_div(100).saturating_mul(95);
    if minimum_out > slippage_adjusted_amount {
        slippage_adjusted_amount = minimum_out;
    }

    // 获取报价
    // 对应sBPF代码中调用get_quote函数
    let quote = get_quote(
        &ctx.accounts.quote_account.to_account_info(),
        slippage_adjusted_amount,
        0,
    )?;

    // 构建交换指令数据
    // 对应sBPF代码中构建的指令数据
    let instruction_data = [0xea, 0xeb, 0xda, 0x01, 0x12, 0x3d, 0x06, 0x66];

    // 执行跨程序调用到PumpFun
    // 对应sBPF代码中的sol_invoke_signed_c调用
    invoke_signed(
        &solana_program::instruction::Instruction {
            program_id: ctx.accounts.pump_program.key(),
            accounts: vec![
                AccountMeta::new(*ctx.accounts.pump_pool.key, false),
                AccountMeta::new(*ctx.accounts.source_token_account.key, false),
                AccountMeta::new(*ctx.accounts.destination_token_account.key, false),
                // 其他所需账户
            ],
            data: instruction_data.to_vec(),
        },
        &[
            ctx.accounts.pump_pool.to_account_info(),
            ctx.accounts.source_token_account.to_account_info(),
            ctx.accounts.destination_token_account.to_account_info(),
            // 其他所需账户
        ],
        &[], // 签名种子
    )?;

    // 调用三明治更新函数
    // 对应sBPF代码中的sandwich_update_frontrun调用
    sandwich_update_frontrun(&ctx.accounts.sandwich_state, source_amount, dest_amount)?;

    // 更新代币数据
    // 对应sBPF代码中的token_data_update_frontrun调用
    token_data_update_frontrun(&ctx.accounts.token_data_account)?;

    Ok(())
}
