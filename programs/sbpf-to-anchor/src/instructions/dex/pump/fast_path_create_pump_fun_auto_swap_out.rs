use crate::instructions::dex::pump::state::{
    FastPathCreatePumpFunAutoSwapOutParams, PumpFunAutoSwapAccount,
};
use crate::states::*;
use anchor_lang::prelude::*;

/// PumpFun快速路径创建自动交换出池指令
///
/// 此指令用于创建PumpFun自动交换出池账户
/// 实现了自动交换出池所需的账户创建和初始化
#[derive(Accounts)]
pub struct FastPathCreatePumpFunAutoSwapOut<'info> {
    /// 交易发起者
    #[account(mut)]
    pub signer: Signer<'info>,

    /// 创建的自动交换出池账户
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 672 + 192 + 512 + 8 + 8 + 8 + 8 + 4
    )]
    pub auto_swap_account: Account<'info, PumpFunAutoSwapAccount>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

/// 在PumpFun上创建自动交换出池账户
///
/// 此函数与sBPF中的fast_path_create_pump_fun_auto_swap_out对应
/// 创建并初始化PumpFun自动交换出池所需的账户数据
pub fn fast_path_create_pump_fun_auto_swap_out(
    ctx: Context<FastPathCreatePumpFunAutoSwapOut>,
    params: FastPathCreatePumpFunAutoSwapOutParams,
) -> Result<()> {
    // 获取参数
    let config_flag = params.config_flag;
    let config_value = params.config_value;

    // 初始化账户数据 - 对应sBPF中的账户初始化逻辑
    let auto_swap_account = &mut ctx.accounts.auto_swap_account;

    // 设置标识符 - 对应sBPF代码中的0xc6440cf5fdda263f值（与入池相同）
    auto_swap_account.identifier = [0x3f, 0x26, 0xda, 0xfd, 0xf5, 0x0c, 0x44, 0xc6];

    // 初始化数据缓冲区
    for i in 0..auto_swap_account.data_buffer.len() {
        auto_swap_account.data_buffer[i] = 0;
    }

    // 初始化状态缓冲区
    for i in 0..auto_swap_account.state_buffer.len() {
        auto_swap_account.state_buffer[i] = 0;
    }

    // 初始化额外缓冲区
    for i in 0..auto_swap_account.extra_buffer.len() {
        auto_swap_account.extra_buffer[i] = 0;
    }

    // 设置配置值
    auto_swap_account.config_value1 = 24;
    auto_swap_account.config_value2 = 12;

    // 设置程序引用
    auto_swap_account.program_reference1 = 0; // 需要在实际实现中使用正确的值
    auto_swap_account.program_reference2 = 0; // 需要在实际实现中使用正确的值

    // 设置配置标志
    auto_swap_account.config_flag = config_value as u32;

    Ok(())
}
