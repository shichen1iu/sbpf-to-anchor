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

    pub fn fast_path_auto_swap_in_pump_fun(
        ctx: Context<FastPathAutoSwapInPumpFun>,
        params: FastPathAutoSwapInPumpParams,
    ) -> Result<()> {
        instructions::dex::pump::fast_path_auto_swap_in_pump_fun(ctx, params)
    }

    pub fn fast_path_auto_swap_out_pump_fun(
        ctx: Context<FastPathAutoSwapOutPumpFun>,
        params: FastPathAutoSwapOutPumpParams,
    ) -> Result<()> {
        instructions::dex::pump::fast_path_auto_swap_out_pump_fun(ctx, params)
    }

    pub fn fast_path_create_pump_fun_auto_swap_in(
        ctx: Context<FastPathCreatePumpFunAutoSwapIn>,
        params: FastPathCreatePumpFunAutoSwapInParams,
    ) -> Result<()> {
        instructions::dex::pump::fast_path_create_pump_fun_auto_swap_in(ctx, params)
    }

    pub fn fast_path_create_pump_fun_auto_swap_out(
        ctx: Context<FastPathCreatePumpFunAutoSwapOut>,
        params: FastPathCreatePumpFunAutoSwapOutParams,
    ) -> Result<()> {
        instructions::dex::pump::fast_path_create_pump_fun_auto_swap_out(ctx, params)
    }

    pub fn fast_path_auto_swap_in_raydium_v4(
        ctx: Context<FastPathAutoSwapInRaydiumV4>,
        params: FastPathAutoSwapParams,
    ) -> Result<()> {
        instructions::dex::raydium::fast_path_auto_swap_in_raydium_v4(ctx, params)
    }

    pub fn fast_path_auto_swap_out_raydium_v4(
        ctx: Context<FastPathAutoSwapOutRaydiumV4>,
        params: FastPathAutoSwapOutParams,
    ) -> Result<()> {
        instructions::dex::raydium::fast_path_auto_swap_out_raydium_v4(ctx, params)
    }

    pub fn fast_path_create_raydium_v4(
        ctx: Context<FastPathCreateRaydiumV4>,
        params: FastPathCreateRaydiumParams,
    ) -> Result<()> {
        instructions::dex::raydium::fast_path_create_raydium_v4(ctx, params)
    }

    pub fn write_sandwich_tracker_identities(
        ctx: Context<WriteSandwichTrackerIdentities>,
    ) -> Result<()> {
        instructions::sandwich::write_sandwich_tracker_identities(ctx)
    }

    pub fn write_sandwich_tracker_leaders(ctx: Context<WriteSandwichTrackerLeaders>) -> Result<()> {
        instructions::sandwich::write_sandwich_tracker_leaders(ctx)
    }

    pub fn extend_sandwich_tracker(ctx: Context<ExtendSandwichTracker>) -> Result<()> {
        instructions::sandwich::extend_sandwich_tracker(ctx)
    }

    pub fn execute_swap_optimised(
        ctx: Context<ExecuteSwapOptimised>,
        swap_data: Vec<u8>,
        bump: u8,
    ) -> Result<()> {
        instructions::swap::execute_swap_optimised(ctx, swap_data, bump)
    }

    pub fn execute_swap(ctx: Context<ExecuteSwap>, swap_data: Vec<u8>, bump: u8) -> Result<()> {
        instructions::swap::execute_swap(ctx, swap_data, bump)
    }

    pub fn create_global(ctx: Context<CreateGlobal>, input_data: Vec<u8>) -> Result<()> {
        instructions::system::create_global(ctx, input_data)
    }

    pub fn update_global(ctx: Context<UpdateGlobal>) -> Result<()> {
        instructions::system::update_global(ctx)
    }

    pub fn create_tipper(ctx: Context<CreateTipper>) -> Result<()> {
        instructions::system::create_tipper(ctx)
    }

    pub fn create_global(ctx: Context<CreateGlobal>, input_data: Vec<u8>) -> Result<()> {
        instructions::system::create_global(ctx, input_data)
    }

    pub fn update_global(ctx: Context<UpdateGlobal>) -> Result<()> {
        instructions::system::update_global(ctx)
    }

    pub fn auto_swap_in(ctx: Context<AutoSwapIn>, data: AutoSwapInData) -> Result<()> {
        instructions::swap::auto_swap_in(ctx, data)
    }

    pub fn auto_swap_out(ctx: Context<AutoSwapOut>, data: AutoSwapOutData) -> Result<()> {
        instructions::swap::auto_swap_out(ctx, data)
    }
}
