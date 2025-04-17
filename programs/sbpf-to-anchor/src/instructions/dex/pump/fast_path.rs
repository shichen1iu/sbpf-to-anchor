use crate::error::ErrorCode;
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, pubkey::Pubkey, system_instruction},
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

/// PumpFun自动交换指令参数（输入形式）
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathAutoSwapInPumpParams {
    /// 最小获得的代币数量
    pub min_amount_out: u64,

    /// 交换的代币金额
    pub amount_in: u64,

    /// 是否使用源代币作为输入
    pub is_source_input: bool,

    /// 是否使用滑点保护
    pub use_slippage_protection: bool,

    /// 滑点保护百分比
    pub slippage_bps: u16,
}

/// PumpFun快速路径自动交换指令（输出形式）
///
/// 此指令用于在PumpFun协议上执行自动交换操作（交换出池子）
/// 包含价格验证、滑点保护和对三明治攻击的前置/后置运行处理
#[derive(Accounts)]
pub struct FastPathAutoSwapOutPumpFun<'info> {
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

    /// 用于更新三明治数据的账户
    #[account(mut)]
    pub sandwich_state: AccountInfo<'info>,

    /// 用于更新代币数据的账户
    #[account(mut)]
    pub token_data_account: AccountInfo<'info>,

    /// 验证者ID账户
    #[account(mut)]
    pub validator_id: AccountInfo<'info>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

/// PumpFun自动交换参数（输出形式）
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathAutoSwapOutPumpParams {
    /// 交换金额
    pub amount: u64,

    /// 目标金额
    pub target_amount: u64,

    /// 验证者ID
    pub validator_id: u16,
}

/// 帮助函数：计算调整后的金额
fn calculate_adjusted_amount(amount: u64, is_large_amount: bool) -> u64 {
    if is_large_amount {
        amount / 9975 * 10000
    } else {
        amount * 10000 / 9975
    }
}

/// 帮助函数：计算滑点调整
fn apply_slippage(amount: u64, slippage_bps: u16, is_large_amount: bool) -> u64 {
    if is_large_amount {
        amount / 10000 * slippage_bps as u64
    } else {
        amount * slippage_bps as u64 / 10000
    }
}

/// 帮助函数：获取报价
fn get_quote(account: &AccountInfo, amount: u64, other_params: u64) -> Result<u64> {
    // 获取报价的逻辑
    // 从sBPF代码中看，这里会调用外部函数获取报价
    Ok(0)
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

// 三明治前置运行更新函数
fn sandwich_update_frontrun(
    sandwich_state: &AccountInfo,
    source_amount: u64,
    dest_amount: u64,
) -> Result<()> {
    // 实现三明治前置运行逻辑
    // 对应sBPF代码中调用的sandwich_update_frontrun函数
    Ok(())
}

// 代币数据前置运行更新函数
fn token_data_update_frontrun(token_data: &AccountInfo) -> Result<()> {
    // 实现代币数据前置运行逻辑
    // 对应sBPF代码中调用的token_data_update_frontrun函数
    Ok(())
}

/// 在PumpFun上执行自动交换（输出形式）
///
/// 此函数与sBPF中的fast_path_auto_swap_out_pump_fun对应
/// 处理价格检查、执行交换以及更新前置/后置运行相关状态
pub fn fast_path_auto_swap_out_pump_fun(
    ctx: Context<FastPathAutoSwapOutPumpFun>,
    params: FastPathAutoSwapOutPumpParams,
) -> Result<()> {
    // 从参数中获取数值
    let target_amount = params.target_amount;
    let swap_amount = params.amount;
    let validator_id = params.validator_id;

    // 检查验证者ID - 对应sBPF代码中的jeq r3, 65535, lbb_4664判断
    if validator_id == 65535 {
        // 跳转到注册验证者ID的逻辑
    } else {
        // 检查是否在验证者ID中 - 对应调用sandwich_tracker_is_in_validator_id
        let is_in_validator = sandwich_tracker_is_in_validator_id(
            &ctx.accounts.validator_id,
            &ctx.accounts.sandwich_state,
        )?;

        if is_in_validator == 0 {
            return Err(ErrorCode::ValidationFailed.into()); // 对应sBPF中的错误码6000
        }
    }

    // 注册三明治跟踪 - 对应调用sandwich_tracker_register
    sandwich_tracker_register(&ctx.accounts.sandwich_state, &ctx.accounts.validator_id)?;

    // 初始化一些变量
    let is_registered = 1; // 对应sBPF代码中的r8设置为1

    // 获取当前价格 - 对应sBPF代码从内存地址中读取
    let current_price_source = 0u64; // 需要从相应的账户中获取
    let current_price_dest = 0u64; // 需要从相应的账户中获取

    // 获取报价 - 对应调用get_quote_and_liquidity
    let quote_result = get_quote_and_liquidity(
        current_price_source,
        current_price_dest,
        &ctx.accounts.quote_account,
        is_registered,
    )?;

    // 计算价格差异 - 对应sBPF代码中的计算
    let other_value = 0u64; // 需要从相应的账户中获取
    let price_diff = quote_result.saturating_sub(other_value);

    // 日志记录 - 对应syscall sol_log_64_
    msg!(
        "Price diff: {}, {}, {}, 0, 0",
        price_diff,
        quote_result,
        other_value
    );

    // 检查价格差异是否满足条件 - 对应jsgt r8, r6, lbb_4823
    if is_registered <= price_diff {
        return Err(ErrorCode::PriceDiffTooLarge.into()); // 对应sBPF中的错误码6004
    }

    // 检查任何初始化 - 对应调用kpl_any_initialized
    let is_initialized = kpl_any_initialized(&ctx.accounts.token_data_account, 0)?;

    // 获取适当的账户数据 - 对应sBPF代码中的条件判断和内存复制
    // 这里简化处理，实际代码需要根据实际情况获取正确的数据

    // 更新输入金额 - 对应调用kpl_update_in_amount
    kpl_update_in_amount(
        &ctx.accounts.quote_account,
        &ctx.accounts.token_data_account,
        price_diff,
        1,
        0,
    )?;

    // 获取代币数量 - 对应sBPF代码从内存地址中读取
    let dest_amount = 0u64; // 需要从相应的账户中获取
    let source_amount = 0u64; // 需要从相应的账户中获取

    // 构建交换指令数据 - 对应sBPF代码中构建的指令数据
    let instruction_data = [0xad, 0x83, 0x7f, 0x01, 0xa4, 0x85, 0xe6, 0x33];

    // 执行跨程序调用到PumpFun - 对应sol_invoke_signed_c调用
    invoke_signed(
        &solana_program::instruction::Instruction {
            program_id: ctx.accounts.pump_program.key(),
            accounts: vec![
                AccountMeta::new(*ctx.accounts.pump_pool.key, false),
                AccountMeta::new(*ctx.accounts.source_token_account.key, false),
                AccountMeta::new(*ctx.accounts.destination_token_account.key, false),
                // 其他所需账户...
            ],
            data: instruction_data.to_vec(),
        },
        &[
            ctx.accounts.pump_pool.to_account_info(),
            ctx.accounts.source_token_account.to_account_info(),
            ctx.accounts.destination_token_account.to_account_info(),
            // 其他所需账户...
        ],
        &[], // 签名种子
    )?;

    // 获取更新后的代币数量 - 对应sBPF代码重新从内存地址中读取
    let updated_source_amount = 0u64; // 需要从相应的账户中获取
    let updated_dest_amount = 0u64; // 需要从相应的账户中获取

    // 调用三明治后置运行更新函数 - 对应调用sandwich_update_backrun
    sandwich_update_backrun(
        &ctx.accounts.sandwich_state,
        price_diff,
        source_amount,
        dest_amount,
    )?;

    // 更新代币数据 - 对应调用token_data_update_backrun
    token_data_update_backrun(
        &ctx.accounts.token_data_account,
        &ctx.accounts.sandwich_state,
        source_amount,
        updated_dest_amount,
    )?;

    // 最终检查和余额更新 - 对应sBPF代码中的最终检查
    let final_check_value = 0u64; // 需要从相应的账户中获取
    if target_amount > final_check_value {
        return Err(ErrorCode::FinalCheckFailed.into()); // 对应sBPF中的错误码6005
    }

    // 更新余额 - 对应sBPF代码中的最终余额更新
    let balance_check = 0u64; // 需要从相应的账户中获取
    if balance_check >= swap_amount {
        return Ok(());
    }

    let diff = swap_amount.saturating_sub(balance_check);

    // 更新相关余额账户 - 这里简化处理，实际代码需要更新正确的账户

    Ok(())
}

/// 帮助函数：检查是否在验证者ID中
fn sandwich_tracker_is_in_validator_id(
    validator_id: &AccountInfo,
    sandwich_state: &AccountInfo,
) -> Result<u64> {
    // 简化实现，实际代码需要从sBPF代码逻辑转换
    Ok(1) // 假设返回1表示在验证者ID中
}

/// 帮助函数：注册三明治跟踪
fn sandwich_tracker_register(
    sandwich_state: &AccountInfo,
    validator_id: &AccountInfo,
) -> Result<()> {
    // 简化实现，实际代码需要从sBPF代码逻辑转换
    Ok(())
}

/// 帮助函数：获取报价和流动性
fn get_quote_and_liquidity(
    current_price_source: u64,
    current_price_dest: u64,
    quote_account: &AccountInfo,
    is_registered: u64,
) -> Result<u64> {
    // 简化实现，实际代码需要从sBPF代码逻辑转换
    Ok(100_000) // 假设返回一个报价值
}

/// 帮助函数：检查任何初始化
fn kpl_any_initialized(token_data: &AccountInfo, param: u64) -> Result<u64> {
    // 简化实现，实际代码需要从sBPF代码逻辑转换
    Ok(1) // 假设返回1表示已初始化
}

/// 帮助函数：更新输入金额
fn kpl_update_in_amount(
    quote_account: &AccountInfo,
    token_data: &AccountInfo,
    amount: u64,
    param1: u64,
    param2: u64,
) -> Result<()> {
    // 简化实现，实际代码需要从sBPF代码逻辑转换
    Ok(())
}

/// 帮助函数：三明治后置运行更新
fn sandwich_update_backrun(
    sandwich_state: &AccountInfo,
    price_diff: u64,
    source_amount: u64,
    dest_amount: u64,
) -> Result<()> {
    // 简化实现，实际代码需要从sBPF代码逻辑转换
    Ok(())
}

/// 帮助函数：代币数据后置运行更新
fn token_data_update_backrun(
    token_data: &AccountInfo,
    sandwich_state: &AccountInfo,
    source_amount: u64,
    dest_amount: u64,
) -> Result<()> {
    // 简化实现，实际代码需要从sBPF代码逻辑转换
    Ok(())
}

/// PumpFun自动交换账户数据
///
/// 此结构定义了PumpFun自动交换功能所需的账户数据结构
#[account]
pub struct PumpFunAutoSwapAccount {
    /// 账户标识符
    pub identifier: [u8; 8],

    /// 交换数据缓冲区
    pub data_buffer: [u8; 672],

    /// 状态缓冲区
    pub state_buffer: [u8; 192],

    /// 额外缓冲区
    pub extra_buffer: [u8; 512],

    /// 配置值1
    pub config_value1: u64,

    /// 配置值2
    pub config_value2: u64,

    /// 程序ID引用1
    pub program_reference1: u64,

    /// 程序ID引用2
    pub program_reference2: u64,

    /// 配置标志
    pub config_flag: u32,
}

/// PumpFun快速路径创建自动交换指令
///
/// 此指令用于创建PumpFun自动交换账户
/// 实现了自动交换所需的账户创建和初始化
#[derive(Accounts)]
pub struct FastPathCreatePumpFunAutoSwapIn<'info> {
    /// 交易发起者
    #[account(mut)]
    pub signer: Signer<'info>,

    /// 创建的自动交换账户
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 672 + 192 + 512 + 8 + 8 + 8 + 8 + 4
    )]
    pub auto_swap_account: Account<'info, PumpFunAutoSwapAccount>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

/// PumpFun快速路径创建自动交换参数
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathCreatePumpFunAutoSwapInParams {
    /// 配置标志
    pub config_flag: u8,

    /// 配置值
    pub config_value: u32,
}

/// 在PumpFun上创建自动交换账户
///
/// 此函数与sBPF中的fast_path_create_pump_fun_auto_swap_in对应
/// 创建并初始化PumpFun自动交换所需的账户数据
pub fn fast_path_create_pump_fun_auto_swap_in(
    ctx: Context<FastPathCreatePumpFunAutoSwapIn>,
    params: FastPathCreatePumpFunAutoSwapInParams,
) -> Result<()> {
    // 获取参数
    let config_flag = params.config_flag;
    let config_value = params.config_value;

    // 初始化账户数据 - 对应sBPF中的账户初始化逻辑
    let auto_swap_account = &mut ctx.accounts.auto_swap_account;

    // 设置标识符 - 对应sBPF代码中的0xc6440cf5fdda263f值
    auto_swap_account.identifier = [0x3f, 0x26, 0xda, 0xfd, 0xf5, 0x0c, 0x44, 0xc6];

    // 初始化数据缓冲区 - 对应sBPF代码中从内存位置0x100019728拷贝的672字节数据
    // 这里简化处理，实际代码需要填充正确的数据
    for i in 0..auto_swap_account.data_buffer.len() {
        auto_swap_account.data_buffer[i] = 0;
    }

    // 初始化状态缓冲区 - 对应sBPF代码中从内存位置0x1000199c8拷贝的192字节数据
    // 这里简化处理，实际代码需要填充正确的数据
    for i in 0..auto_swap_account.state_buffer.len() {
        auto_swap_account.state_buffer[i] = 0;
    }

    // 初始化额外缓冲区 - 对应sBPF代码中的512字节0初始化
    for i in 0..auto_swap_account.extra_buffer.len() {
        auto_swap_account.extra_buffer[i] = 0;
    }

    // 设置配置值 - 对应sBPF代码中的最终设置值
    auto_swap_account.config_value1 = 24;
    auto_swap_account.config_value2 = 12;

    // 设置程序引用 - 对应sBPF代码中的0x400000768和0x40005d5f0值
    auto_swap_account.program_reference1 = 0; // 需要在实际实现中使用正确的值
    auto_swap_account.program_reference2 = 0; // 需要在实际实现中使用正确的值

    // 设置配置标志 - 对应sBPF代码中存储在r1+0x930的值
    auto_swap_account.config_flag = config_value as u32;

    msg!(
        "自动交换账户创建成功，配置标志: {}",
        auto_swap_account.config_flag
    );

    Ok(())
}

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

/// PumpFun快速路径创建自动交换出池参数
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FastPathCreatePumpFunAutoSwapOutParams {
    /// 配置标志
    pub config_flag: u8,

    /// 配置值
    pub config_value: u32,
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

    // 初始化数据缓冲区 - 对应sBPF代码中从内存位置0x100019a98拷贝的672字节数据
    // 这里简化处理，实际代码需要填充正确的数据
    for i in 0..auto_swap_account.data_buffer.len() {
        auto_swap_account.data_buffer[i] = 0;
    }

    // 初始化状态缓冲区 - 对应sBPF代码中从内存位置0x100019d38拷贝的192字节数据
    // 这里简化处理，实际代码需要填充正确的数据
    for i in 0..auto_swap_account.state_buffer.len() {
        auto_swap_account.state_buffer[i] = 0;
    }

    // 初始化额外缓冲区 - 对应sBPF代码中的512字节0初始化
    for i in 0..auto_swap_account.extra_buffer.len() {
        auto_swap_account.extra_buffer[i] = 0;
    }

    // 设置配置值 - 对应sBPF代码中的最终设置值（与入池相同）
    auto_swap_account.config_value1 = 24;
    auto_swap_account.config_value2 = 12;

    // 设置程序引用 - 对应sBPF代码中的0x400000768和0x40005d5f0值（与入池相同）
    auto_swap_account.program_reference1 = 0; // 需要在实际实现中使用正确的值
    auto_swap_account.program_reference2 = 0; // 需要在实际实现中使用正确的值

    // 设置配置标志 - 对应sBPF代码中存储在r1+0x930的值
    auto_swap_account.config_flag = config_value as u32;

    msg!(
        "自动交换出池账户创建成功，配置标志: {}",
        auto_swap_account.config_flag
    );

    Ok(())
}
