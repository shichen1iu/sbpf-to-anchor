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

    /// 在Raydium V4上执行自动交换
    pub fn fast_path_auto_swap_in_raydium_v4(
        ctx: Context<FastPathAutoSwapInRaydiumV4>,
        params: FastPathAutoSwapParams,
    ) -> Result<()> {
        instructions::dex::raydium::fast_path_instructions::fast_path_auto_swap_in_raydium_v4(
            ctx, params,
        )
    }

    /// 在Raydium V4上执行自动输出交换
    pub fn fast_path_auto_swap_out_raydium_v4(
        ctx: Context<FastPathAutoSwapOutRaydiumV4>,
        params: FastPathAutoSwapOutParams,
    ) -> Result<()> {
        instructions::dex::raydium::fast_path_instructions::fast_path_auto_swap_out_raydium_v4(
            ctx, params,
        )
    }

    /// 创建Raydium V4池账户和初始化相关数据
    pub fn fast_path_create_raydium_v4(
        ctx: Context<FastPathCreateRaydiumV4>,
        params: FastPathCreateRaydiumParams,
    ) -> Result<()> {
        instructions::dex::raydium::fast_path_instructions::fast_path_create_raydium_v4(ctx, params)
    }
}
