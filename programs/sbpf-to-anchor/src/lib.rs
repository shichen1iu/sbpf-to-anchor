use anchor_lang::prelude::*;
use solana_program::program_error::ProgramError;

mod error;
mod instructions;
mod states;
mod utils;

use instructions::*;
use states::*;

declare_id!("D9MxitF878nXCgeTyiUYXGjsC8hh55HY2RzVnMRLwJSJ");

#[program]
pub mod sbpf_to_anchor {
    use super::*;

    // Main functions that dispatch to specific implementations
    pub fn is_valid(ctx: Context<IsValid>, dex_type: u8) -> Result<bool> {
        instructions::is_valid(ctx, dex_type)
    }

    pub fn get_quote(ctx: Context<GetQuote>, dex_type: u8) -> Result<u64> {
        instructions::get_quote(ctx, dex_type)
    }

    pub fn get_liquidity(ctx: Context<GetLiquidity>, dex_type: u8) -> Result<(u64, u64)> {
        instructions::get_liquidity(ctx, dex_type)
    }

    pub fn get_quote_and_liquidity(
        ctx: Context<GetQuoteAndLiquidity>,
        dex_type: u8,
    ) -> Result<(u64, u64, u64)> {
        instructions::get_quote_and_liquidity(ctx, dex_type)
    }

    pub fn calculate_profit_optimised(ctx: Context<CalculateProfitOptimised>) -> Result<u64> {
        instructions::calculate_profit_optimised(ctx)
    }

    pub fn calculate_hinted_max_amount_optimised(
        ctx: Context<CalculateHintedMaxAmountOptimised>,
    ) -> Result<u64> {
        instructions::calculate_hinted_max_amount_optimised(ctx)
    }

    pub fn calculate_upper_bound_optimised(
        ctx: Context<CalculateUpperBoundOptimised>,
    ) -> Result<u64> {
        instructions::calculate_upper_bound_optimised(ctx)
    }

    pub fn calculate_optimal_strategy_optimised(
        ctx: Context<CalculateOptimalStrategyOptimised>,
    ) -> Result<bool> {
        instructions::calculate_optimal_strategy_optimised(ctx)
    }

    pub fn calculate_profit(ctx: Context<CalculateProfit>) -> Result<u64> {
        instructions::calculate_profit(ctx)
    }

    pub fn is_buy_amount_too_big(ctx: Context<IsBuyAmountTooBig>) -> Result<bool> {
        instructions::is_buy_amount_too_big(ctx)
    }

    pub fn calculate_optimal_strategy_deprecated(
        ctx: Context<CalculateOptimalStrategyDeprecated>,
    ) -> Result<bool> {
        instructions::calculate_optimal_strategy_deprecated(ctx)
    }

    // 新添加的Raydium V4相关指令
    pub fn fast_path_auto_swap_in_raydium_v4(
        ctx: Context<FastPathAutoSwapInRaydiumV4>,
    ) -> Result<()> {
        instructions::fast_path_auto_swap_in_raydium_v4(ctx)
    }

    pub fn fast_path_auto_swap_out_raydium_v4(
        ctx: Context<FastPathAutoSwapOutRaydiumV4>,
    ) -> Result<()> {
        instructions::fast_path_auto_swap_out_raydium_v4(ctx)
    }

    pub fn fast_path_create_raydium_v4(ctx: Context<FastPathCreateRaydiumV4>) -> Result<()> {
        instructions::fast_path_create_raydium_v4(ctx)
    }

    pub fn create_sandwich_tracker(ctx: Context<CreateSandwichTracker>) -> Result<()> {
        instructions::create_sandwich_tracker(ctx)
    }

    pub fn create_global(ctx: Context<CreateGlobal>) -> Result<()> {
        instructions::create_global(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    // 更新全局状态
    pub fn update_global(ctx: Context<UpdateGlobal>) -> Result<()> {
        instructions::update_global(ctx)
    }

    // Pump Fun DEX快速路径自动交换
    pub fn fast_path_auto_swap_in_pump_fun(ctx: Context<FastPathAutoSwapInPumpFun>) -> Result<()> {
        instructions::fast_path_auto_swap_in_pump_fun(ctx)
    }

    // Pump Fun DEX快速路径自动反向交换
    pub fn fast_path_auto_swap_out_pump_fun(
        ctx: Context<FastPathAutoSwapOutPumpFun>,
    ) -> Result<()> {
        instructions::fast_path_auto_swap_out_pump_fun(ctx)
    }

    // 创建Pump Fun自动交换入账户
    pub fn fast_path_create_pump_fun_auto_swap_in(
        ctx: Context<FastPathCreatePumpFunAutoSwap>,
    ) -> Result<()> {
        instructions::fast_path_create_pump_fun_auto_swap_in(ctx)
    }

    // 创建Pump Fun自动交换出账户
    pub fn fast_path_create_pump_fun_auto_swap_out(
        ctx: Context<FastPathCreatePumpFunAutoSwap>,
    ) -> Result<()> {
        instructions::fast_path_create_pump_fun_auto_swap_out(ctx)
    }

    // 获取交换指令的优化实现
    pub fn get_swap_instruction_optimised(ctx: Context<GetSwapInstruction>) -> Result<u64> {
        instructions::get_swap_instruction_optimised(ctx)
    }

    // 实际实现deserialize_swap函数
    pub fn deserialize_swap(ctx: Context<DeserializeSwap>) -> Result<bool> {
        instructions::deserialize_swap(ctx)
    }

    // 添加到program模块里的新指令
    pub fn deserialize_swap_v4(ctx: Context<DeserializeSwapV4>) -> Result<bool> {
        instructions::deserialize_swap_v4(ctx)
    }

    pub fn exit_program(ctx: Context<ExitProgram>) -> Result<()> {
        instructions::exit_program(ctx)
    }

    #[derive(Accounts)]
    pub struct CreateCreditor<'info> {
        #[account(mut)]
        pub payer: Signer<'info>,
        #[account(
            init,
            payer = payer,
            space = 8 + 56
        )]
        pub creditor: Account<'info, Creditor>,
        pub system_program: Program<'info, System>,
    }
    #[account]
    pub struct Creditor {
        pub data: [u8; 56],
    }
    pub fn create_creditor(ctx: Context<CreateCreditor>) -> Result<()> {
        let creditor = &mut ctx.accounts.creditor;
        creditor.data = [0; 56];
        Ok(())
    }

    pub fn tip_dynamic(ctx: Context<TipDynamic>, amount: u64) -> Result<()> {
        instructions::tip_dynamic(ctx, amount)
    }

    pub fn tip_static(ctx: Context<TipStatic>, amount: u64) -> Result<()> {
        instructions::tip_static(ctx, amount)
    }

    pub fn topup_tipper(ctx: Context<TopupTipper>, amount: u64) -> Result<()> {
        instructions::topup_tipper(ctx, amount)
    }

    pub fn extend_sandwich_tracker(ctx: Context<ExtendSandwichTracker>) -> Result<()> {
        instructions::extend_sandwich_tracker(ctx)
    }

    pub fn write_sandwich_tracker_identities(
        ctx: Context<WriteSandwichTrackerIdentities>,
    ) -> Result<()> {
        instructions::write_sandwich_tracker_identities(ctx)
    }

    // 快速路径小费相关功能
    pub fn fast_path_tip_static(ctx: Context<FastPathTipStatic>, amount: u64) -> Result<()> {
        instructions::fast_path_tip_static(ctx, amount)
    }

    pub fn fast_path_tip_dynamic(ctx: Context<FastPathTipDynamic>, base_amount: u64) -> Result<()> {
        instructions::fast_path_tip_dynamic(ctx, base_amount)
    }

    #[derive(Accounts)]
    pub struct FastPathCreateTipStatic<'info> {
        #[account(mut)]
        pub authority: Signer<'info>,
        #[account(
            init,
            payer = authority,
            space = 8 + TipAccount::LEN
        )]
        pub tip_account: Account<'info, TipAccount>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }
    pub fn fast_path_create_tip_static(ctx: Context<FastPathCreateTipStatic>) -> Result<()> {
        // 初始化静态小费账户
        let tip_account = &mut ctx.accounts.tip_account;
        tip_account.owner = ctx.accounts.authority.key();
        tip_account.amount = 0;
        tip_account.tip_type = TipType::Static;

        Ok(())
    }

    #[derive(Accounts)]
    pub struct FastPathCreateTipDynamic<'info> {
        #[account(mut)]
        pub authority: Signer<'info>,
        #[account(
            init,
            payer = authority,
            space = 8 + TipAccount::LEN
        )]
        pub tip_account: Account<'info, TipAccount>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }
    pub fn fast_path_create_tip_dynamic(ctx: Context<FastPathCreateTipDynamic>) -> Result<()> {
        // 初始化动态小费账户
        let tip_account = &mut ctx.accounts.tip_account;
        tip_account.owner = ctx.accounts.authority.key();
        tip_account.amount = 0;
        tip_account.tip_type = TipType::Dynamic;
        tip_account.time_factor = 10000; // 默认时间因子

        Ok(())
    }

    // 三明治交易相关功能
    pub fn auto_swap_out(ctx: Context<AutoSwapOut>) -> Result<()> {
        instructions::auto_swap_out(ctx)
    }

    pub fn close_sandwich(ctx: Context<CloseSandwich>) -> Result<()> {
        instructions::close_sandwich(ctx)
    }

    pub fn close_sandwiches_and_topup_tipper(
        ctx: Context<CloseSandwichesAndTopupTipper>,
    ) -> Result<()> {
        instructions::close_sandwiches_and_topup_tipper(ctx)
    }

    // 创建授权
    pub fn create_auth(ctx: Context<CreateAuth>) -> Result<()> {
        instructions::create_auth(ctx)
    }
}

#[account]
#[derive(InitSpace)]
pub struct SandwichTracker {
    pub initialized: bool,
    pub frontrun_tracker: [u8; 512],
    pub backrun_tracker: [u8; 512],
    pub leaders: [Pubkey; 5],
    pub scores: [u64; 5],
}

impl SandwichTracker {
    pub const LEN: usize = 1 + 512 + 512 + (32 * 5) + (8 * 5);
}

#[account]
#[derive(InitSpace)]
pub struct TipperAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub total_tipped: u64,
}

impl TipperAccount {
    pub const LEN: usize = 32 + 8 + 8;
}

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub initialized: bool,
    pub admin: Pubkey,
    pub tipper_fee: u64,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub magic_number: u64,
}

impl GlobalState {
    pub const LEN: usize = 1 + 32 + 8 + 8 + 8;
}

// Raydium V4特有的反序列化函数
fn deserialize_swap_optimised(
    swap_data: AccountInfo,
    pool_data: AccountInfo,
    output_data: &mut [AccountMeta; 10],
) -> Result<bool> {
    // 检查程序ID是否是Raydium V4
    let program_id = swap_data.try_borrow_data()?;
    let program_id_bytes = &program_id[0..32];

    // 从汇编代码中提取的解析逻辑
    let mut result = false;

    // 可以根据程序ID判断
    if program_id_bytes == b"RAYDIUM_V4_PROGRAM_ID................" {
        let mut liquidity_buffer = [0u64; 2];

        // 调用Raydium解析函数
        if raydium_v4_parse_liquidity(pool_data.clone(), &mut liquidity_buffer)? {
            // 解析成功，设置账户元数据
            // 注意：实际实现需要根据Raydium V4的协议格式定制

            result = true;
        }
    }

    Ok(result)
}

// 辅助函数和扩展实现
impl<'info> FastPathAutoSwapInPumpFun<'info> {
    pub fn has_global_controller(&self) -> bool {
        // 实际实现中需要检查全局控制器是否存在
        true
    }

    pub fn is_reverse(&self) -> bool {
        // 实际实现中需要检查交换方向
        false
    }

    pub fn get_threshold(&self) -> u64 {
        // 实际实现中需要从配置中获取阈值
        1000
    }
}

impl<'info> FastPathAutoSwapOutPumpFun<'info> {
    pub fn needs_validator_check(&self) -> bool {
        // 实际实现中需要检查是否需要验证者检查
        !self.validator_id.key.eq(&Pubkey::default())
    }

    pub fn is_valid_validator(&self) -> Result<bool> {
        // 实际实现中需要检查验证者ID是否有效
        Ok(true)
    }

    pub fn need_register_tracker(&self) -> bool {
        // 实际实现中需要检查是否需要注册追踪
        true
    }

    pub fn register_sandwich_tracker(&self) -> Result<()> {
        // 实际实现中需要注册sandwich追踪
        Ok(())
    }
}
