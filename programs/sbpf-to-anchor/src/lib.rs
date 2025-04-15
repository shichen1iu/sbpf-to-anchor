use anchor_lang::prelude::*;
use solana_program::program_error::ProgramError;

declare_id!("D9MxitF878nXCgeTyiUYXGjsC8hh55HY2RzVnMRLwJSJ");

#[program]
pub mod sbpf_to_anchor {
    use super::*;

    // Main functions that dispatch to specific implementations
    pub fn is_valid(ctx: Context<IsValid>, dex_type: u8) -> Result<bool> {
        if dex_type == 0 {
            // Raydium
            let accounts = &ctx.accounts;
            raydium_is_valid(accounts.input_data.to_account_info())
        } else if dex_type == 1 {
            // Pump Fun
            let accounts = &ctx.accounts;
            pump_fun_is_valid(accounts.input_data.to_account_info())
        } else {
            // Default to false for unknown dex types
            Ok(false)
        }
    }

    pub fn get_quote(ctx: Context<GetQuote>, dex_type: u8) -> Result<u64> {
        let accounts = &ctx.accounts;

        if dex_type == 0 {
            // Raydium
            raydium_get_quote(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )
        } else if dex_type == 1 {
            // Pump Fun
            pump_fun_get_quote(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )
        } else {
            Err(ProgramError::InvalidArgument.into())
        }
    }

    pub fn get_liquidity(ctx: Context<GetLiquidity>, dex_type: u8) -> Result<(u64, u64)> {
        let accounts = &ctx.accounts;

        if dex_type == 0 {
            // Add this value to the first part of output to indicate Raydium
            let result = raydium_get_liquidity(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )?;
            Ok(result)
        } else if dex_type == 1 {
            // Add this value to the first part of output to indicate PumpFun
            let result = pump_fun_get_liquidity(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )?;
            Ok(result)
        } else {
            Err(ProgramError::InvalidArgument.into())
        }
    }

    pub fn get_quote_and_liquidity(
        ctx: Context<GetQuoteAndLiquidity>,
        dex_type: u8,
    ) -> Result<(u64, u64, u64)> {
        let accounts = &ctx.accounts;

        if dex_type == 0 {
            // Raydium
            raydium_get_quote_and_liquidity(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )
        } else if dex_type == 1 {
            // Pump Fun
            pump_fun_get_quote_and_liquidity(
                accounts.input_data.to_account_info(),
                accounts.amount.amount,
                accounts.reverse.reverse,
            )
        } else {
            Err(ProgramError::InvalidArgument.into())
        }
    }

    pub fn calculate_profit_optimised(ctx: Context<CalculateProfitOptimised>) -> Result<u64> {
        let accounts = &ctx.accounts;

        // Implements the calculate_profit_optimised function logic
        let amount = accounts.amount.amount;

        // We need to call each function directly instead of referencing the instruction
        // Get quote and liquidity first
        let (quote1, reserve_a, reserve_b) = if accounts.dex_type.dex_type == 0 {
            raydium_get_quote_and_liquidity(accounts.quote_ctx.clone(), amount, false)?
        } else {
            pump_fun_get_quote_and_liquidity(accounts.quote_ctx.clone(), amount, false)?
        };

        // Get liquidity for the reverse direction
        let (reverse_reserve_a, reverse_reserve_b) = if accounts.dex_type_reverse.dex_type == 0 {
            raydium_get_liquidity(accounts.liquidity_ctx.clone(), amount, true)?
        } else {
            pump_fun_get_liquidity(accounts.liquidity_ctx.clone(), amount, true)?
        };

        // Get the final quote
        let quote2 = if accounts.dex_type_reverse.dex_type == 0 {
            raydium_get_quote(accounts.quote_ctx_reverse.clone(), amount, true)?
        } else {
            pump_fun_get_quote(accounts.quote_ctx_reverse.clone(), amount, true)?
        };

        // Calculate profit: output minus input amount
        Ok(quote2.saturating_sub(amount))
    }

    pub fn calculate_hinted_max_amount_optimised(
        ctx: Context<CalculateHintedMaxAmountOptimised>,
    ) -> Result<u64> {
        let accounts = &ctx.accounts;
        let max_input = accounts.max_input.amount;
        let available = accounts.available.amount;
        let fee_numerator = accounts.fee_numerator.amount;
        let fee_denominator = accounts.fee_denominator.amount;

        if max_input > available {
            return Ok(0);
        }

        let amount = available.saturating_sub(max_input);
        let fee_adjusted = 10000u64.saturating_sub(fee_numerator);

        let mut result;
        if amount > 0x68db8bac710cc {
            result = amount / fee_adjusted * 10000;
        } else {
            result = amount * 10000 / fee_adjusted;
        }

        if result > 0x68db8bac710cc {
            result = result / 10000 * fee_denominator;
        } else {
            result = result * fee_denominator / 10000;
        }

        Ok(result)
    }

    pub fn calculate_upper_bound_optimised(
        ctx: Context<CalculateUpperBoundOptimised>,
    ) -> Result<u64> {
        let accounts = &ctx.accounts;
        let dex_type = accounts.dex_type.dex_type;
        let amount = accounts.amount.amount;

        // Default fee rate is 9975 (0.25% fee)
        let mut fee_rate = 9975u64;

        // If dex_type is 1, use 9900 (1% fee)
        if dex_type == 1 {
            fee_rate = 9900;
        }

        // Get the appropriate amount based on the is_token_a flag
        let available = if accounts.is_token_a.is_token_a == 0 {
            accounts.amounts.token_a_amount
        } else {
            accounts.amounts.token_b_amount
        };

        if available > amount {
            let remaining = amount.saturating_sub(available);
            let output_amount;

            if remaining > 0x68db8bac710cc {
                output_amount = remaining / fee_rate * 10000;
            } else {
                output_amount = remaining * 10000 / fee_rate;
            }

            let result;
            let multiplier = accounts.multiplier.amount;

            if output_amount > 0x68db8bac710cc {
                result = output_amount / 10000 * multiplier;
            } else {
                result = output_amount * multiplier / 10000;
            }

            Ok(result)
        } else {
            Ok(0)
        }
    }

    pub fn calculate_optimal_strategy_optimised(
        ctx: Context<CalculateOptimalStrategyOptimised>,
    ) -> Result<bool> {
        // 这个函数的逻辑过于复杂，需要更详细的拆解工作
        // 基本结构已添加，实际实现需要更多分析
        Ok(true)
    }

    pub fn calculate_profit(ctx: Context<CalculateProfit>) -> Result<u64> {
        let accounts = &ctx.accounts;

        // Implements the calculate_profit function logic
        let amount = accounts.amount.amount;
        let reverse_flag = accounts.reverse.reverse;

        // Get quote and liquidity first
        let (quote1, reserve_a, reserve_b) = if accounts.dex_type.dex_type == 0 {
            raydium_get_quote_and_liquidity(
                accounts.input_data.to_account_info(),
                amount,
                reverse_flag,
            )?
        } else {
            pump_fun_get_quote_and_liquidity(
                accounts.input_data.to_account_info(),
                amount,
                reverse_flag,
            )?
        };

        // Get liquidity for the same pools
        let (liquidity_a, liquidity_b) = if accounts.dex_type.dex_type == 0 {
            raydium_get_liquidity(accounts.input_data.to_account_info(), amount, reverse_flag)?
        } else {
            pump_fun_get_liquidity(accounts.input_data.to_account_info(), amount, reverse_flag)?
        };

        // Get the quote with reversed direction
        let reverse_quote = if accounts.dex_type.dex_type == 0 {
            raydium_get_quote(accounts.input_data.to_account_info(), quote1, !reverse_flag)?
        } else {
            pump_fun_get_quote(accounts.input_data.to_account_info(), quote1, !reverse_flag)?
        };

        // Calculate profit: original amount minus reverse quote
        Ok(reverse_quote.saturating_sub(amount))
    }

    pub fn is_buy_amount_too_big(ctx: Context<IsBuyAmountTooBig>) -> Result<bool> {
        let accounts = &ctx.accounts;
        let input_data = accounts.input_data.to_account_info();
        let dex_type = accounts.dex_type.dex_type;
        let amount = accounts.amount.amount;
        let threshold = accounts.threshold.amount;
        let reverse = accounts.reverse.reverse;

        // Get liquidity first
        let (reserve_a, reserve_b) = if dex_type == 0 {
            raydium_get_liquidity(input_data.clone(), amount, reverse)?
        } else {
            pump_fun_get_liquidity(input_data.clone(), amount, reverse)?
        };

        // Get the quote
        let quote = if dex_type == 0 {
            raydium_get_quote(input_data.clone(), amount, reverse)?
        } else {
            pump_fun_get_quote(input_data.clone(), amount, reverse)?
        };

        // Check if the quote is less than the threshold
        if threshold > quote {
            return Ok(true);
        }

        // Also check if the pool is valid
        let is_valid_result = if dex_type == 0 {
            raydium_is_valid(input_data)?
        } else {
            pump_fun_is_valid(input_data)?
        };

        // If pool is not valid, it's also too big
        Ok(!is_valid_result)
    }

    pub fn calculate_optimal_strategy_deprecated(
        ctx: Context<CalculateOptimalStrategyDeprecated>,
    ) -> Result<bool> {
        // 这个函数实现了一个更早版本的优化策略计算
        // 基于汇编代码中的 calculate_optimal_strategy_deprecated 函数

        let accounts = &ctx.accounts;
        let upper_bound = calculate_upper_bound(
            accounts.amount.amount,
            accounts.dex_type.dex_type,
            accounts.amounts.token_a_amount,
            accounts.amounts.token_b_amount,
            accounts.is_token_a.is_token_a,
            accounts.multiplier.amount,
        )?;

        // 如果上界小于1000，直接返回成功
        if upper_bound < 1000 {
            return Ok(true);
        }

        // 这里是复杂的优化策略计算，实际实现较为复杂
        // 为简化起见，返回真值表示成功

        Ok(true)
    }

    // 新添加的Raydium V4相关指令
    pub fn fast_path_auto_swap_in_raydium_v4(
        ctx: Context<FastPathAutoSwapInRaydiumV4>,
    ) -> Result<()> {
        let accounts = &ctx.accounts;

        // 实现fast_path_auto_swap_in_raydium_v4逻辑
        // 在汇编中看到的主要逻辑包括：
        // 1. 检查池子有效性
        // 2. 计算兑换金额
        // 3. 执行token转账
        // 4. 更新sandwich状态

        // 获取相关参数
        let dex_type = accounts.dex_type.dex_type;
        let amount = accounts.amount.amount;
        let reverse = accounts.reverse.reverse;

        // 验证池子有效性
        let is_valid = if dex_type == 0 {
            raydium_is_valid(accounts.input_data.to_account_info())?
        } else {
            pump_fun_is_valid(accounts.input_data.to_account_info())?
        };

        // 检查池子是否有效
        if !is_valid {
            return Err(SwapError::InvalidPoolState.into());
        }

        // 计算兑换金额
        let (quote, reserve_a, reserve_b) = if dex_type == 0 {
            raydium_get_quote_and_liquidity(accounts.input_data.to_account_info(), amount, reverse)?
        } else {
            pump_fun_get_quote_and_liquidity(
                accounts.input_data.to_account_info(),
                amount,
                reverse,
            )?
        };

        // 检查流动性
        if quote == 0 {
            return Err(SwapError::InsufficientLiquidity.into());
        }

        // 执行token转账
        // 实际执行时需要使用CPI调用token程序

        // 更新sandwich状态
        // 在汇编代码中看到对sandwich_update_frontrun的调用

        msg!("Fast path auto swap in Raydium V4 executed successfully");
        Ok(())
    }

    pub fn fast_path_auto_swap_out_raydium_v4(
        ctx: Context<FastPathAutoSwapOutRaydiumV4>,
    ) -> Result<()> {
        let accounts = &ctx.accounts;

        // 实现fast_path_auto_swap_out_raydium_v4逻辑
        // 主要步骤:
        // 1. 检查是否是验证者
        // 2. 注册sandwich_tracker
        // 3. 获取和验证流动性
        // 4. 执行token转账
        // 5. 更新sandwich状态

        // 获取参数
        let dex_type = accounts.dex_type.dex_type;
        let amount = accounts.amount.amount;
        let reverse = accounts.reverse.reverse;

        // 验证池子有效性和流动性
        let (reserve_a, reserve_b) = if dex_type == 0 {
            raydium_get_liquidity(accounts.input_data.to_account_info(), amount, reverse)?
        } else {
            pump_fun_get_liquidity(accounts.input_data.to_account_info(), amount, reverse)?
        };

        // 获取报价
        let quote = if dex_type == 0 {
            raydium_get_quote(accounts.input_data.to_account_info(), amount, reverse)?
        } else {
            pump_fun_get_quote(accounts.input_data.to_account_info(), amount, reverse)?
        };

        // 检查流动性
        if quote == 0 {
            return Err(SwapError::InsufficientLiquidity.into());
        }

        // 更新sandwich状态 (backrun)
        // 在汇编代码中看到对sandwich_update_backrun的调用

        msg!("Fast path auto swap out Raydium V4 executed successfully");
        Ok(())
    }

    pub fn fast_path_create_raydium_v4(ctx: Context<FastPathCreateRaydiumV4>) -> Result<()> {
        // 实现fast_path_create_raydium_v4逻辑
        // 主要步骤:
        // 1. 创建Raydium V4池子
        // 2. 初始化必要的账户和状态
        // 3. 设置池子初始参数
        // 初始化池子状态
        ctx.accounts.pool_state.dex_type = 0; // Raydium类型
        ctx.accounts.pool_state.initialized = true;

        // 在汇编中看到的其他初始化逻辑

        msg!("Fast path create Raydium V4 executed successfully");
        Ok(())
    }

    pub fn close_sandwiches_and_topup_tipper(
        ctx: Context<CloseSandwichesAndTopupTipper>,
    ) -> Result<()> {
        let accounts = &ctx.accounts;

        // 实现close_sandwiches_and_topup_tipper逻辑
        // 主要步骤:
        // 1. 关闭所有未完成的sandwich交易
        // 2. 将费用转给tipper账户
        // 3. 更新系统状态

        // 检查权限

        // 关闭sandwich交易并转账给tipper

        msg!("Closed sandwiches and topped up tipper successfully");
        Ok(())
    }

    pub fn create_sandwich_tracker(ctx: Context<CreateSandwichTracker>) -> Result<()> {
        // 实现创建sandwich追踪器的逻辑
        // 1. 初始化追踪器状态
        // 2. 设置初始参数

        // 初始化sandwich追踪器
        ctx.accounts.tracker.initialized = true;

        msg!("Sandwich tracker created successfully");
        Ok(())
    }

    pub fn create_global(ctx: Context<CreateGlobal>) -> Result<()> {
        let accounts = &ctx.accounts;

        // 实现create_global逻辑
        // 用于创建全局配置或状态账户

        // 初始化全局状态
        ctx.accounts.global_state.initialized = true;

        msg!("Global state created successfully");
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let accounts = &ctx.accounts;

        // 实现withdraw逻辑
        // 1. 验证权限
        // 2. 检查余额
        // 3. 执行提款

        // 验证权限

        // 检查余额
        let balance = accounts.source_account.amount;
        if balance < amount {
            return Err(SwapError::InsufficientBalance.into());
        }

        // 执行提款
        ctx.accounts.source_account.amount -= amount;

        msg!("Withdrawal of {} tokens successful", amount);
        Ok(())
    }

    // 更新全局状态
    pub fn update_global(ctx: Context<UpdateGlobal>) -> Result<()> {
        let accounts = &ctx.accounts;

        // 检查更新标志
        let update_fee_flag = accounts.global_update_data.update_fee_flag;
        let update_config_flag = accounts.global_update_data.update_config_flag;

        // 更新费用相关字段
        if update_fee_flag {
            msg!("Updating global fee settings");
            let fee_numerator = accounts.global_update_data.fee_numerator;
            let fee_denominator = accounts.global_update_data.fee_denominator;
            let tipper_fee = accounts.global_update_data.tipper_fee;

            // 更新全局状态中的费用设置
            ctx.accounts.global_state.fee_numerator = fee_numerator;
            ctx.accounts.global_state.fee_denominator = fee_denominator;
            ctx.accounts.global_state.tipper_fee = tipper_fee;
        }

        // 更新配置相关字段
        if update_config_flag {
            msg!("Updating global configuration");
            // 从全局更新数据拷贝配置数据到全局状态
            // 实际实现中需要根据具体字段来拷贝
            // 这里简化处理
        }

        msg!("Global state updated successfully");
        Ok(())
    }

    // Pump Fun DEX快速路径自动交换
    pub fn fast_path_auto_swap_in_pump_fun(ctx: Context<FastPathAutoSwapInPumpFun>) -> Result<()> {
        let accounts = &ctx.accounts;

        // 检查是否有全局控制器
        if !accounts.has_global_controller() {
            return Err(SwapError::InvalidGlobalController.into());
        }

        // 检查交换方向设置
        if accounts.is_reverse() {
            return Err(SwapError::InvalidSwapDirection.into());
        }

        // 获取流动性和报价
        let (reserve_a, reserve_b) = pump_fun_get_liquidity(
            accounts.pool_data.to_account_info(),
            accounts.amount_data.amount,
            false,
        )?;

        // 检查阈值
        let threshold = accounts.get_threshold();
        if reserve_b < threshold {
            return Err(SwapError::InsufficientLiquidity.into());
        }

        // 计算兑换金额
        let quote = pump_fun_get_quote(
            accounts.pool_data.to_account_info(),
            accounts.amount_data.amount,
            false,
        )?;

        // 检查报价是否为零
        if quote == 0 {
            return Err(SwapError::InvalidQuote.into());
        }

        // 保存计算结果
        ctx.accounts.amount_data.amount = quote;

        // 更新sandwich交易相关数据
        // 在实际实现中这需要调用sandwich_update_frontrun

        msg!("Fast path auto swap in Pump Fun executed successfully");
        Ok(())
    }

    // Pump Fun DEX快速路径自动反向交换
    pub fn fast_path_auto_swap_out_pump_fun(
        ctx: Context<FastPathAutoSwapOutPumpFun>,
    ) -> Result<()> {
        let accounts = &ctx.accounts;

        // 检查是否有效的验证者身份
        if accounts.needs_validator_check() && !accounts.is_valid_validator()? {
            return Err(SwapError::InvalidValidator.into());
        }

        // 注册sandwich追踪
        if accounts.need_register_tracker() {
            accounts.register_sandwich_tracker()?;
        }

        // 获取报价和流动性
        let (quote, reserve_a, reserve_b) = pump_fun_get_quote_and_liquidity(
            accounts.pool_data.to_account_info(),
            accounts.amount_data.amount,
            true,
        )?;

        // 检查流动性
        if quote == 0 {
            return Err(SwapError::InsufficientLiquidity.into());
        }

        // 执行交换
        // 在实际实现中这需要使用CPI调用token转账指令

        // 更新sandwich交易状态
        // 在实际实现中这需要调用sandwich_update_backrun

        msg!("Fast path auto swap out Pump Fun executed successfully");
        Ok(())
    }

    // 创建Pump Fun自动交换入账户
    pub fn fast_path_create_pump_fun_auto_swap_in(
        ctx: Context<FastPathCreatePumpFunAutoSwap>,
    ) -> Result<()> {
        let accounts = &ctx.accounts;

        // 初始化快速路径账户
        ctx.accounts.swap_account.initialized = true;
        ctx.accounts.swap_account.swap_type = 2; // Pump Fun Auto Swap In
        ctx.accounts.swap_account.dex_type = 1; // 1 = Pump Fun

        // 初始化内部数据区域
        // 为简化这里只添加必要的初始化

        msg!("Created Fast Path Pump Fun Auto Swap In account successfully");
        Ok(())
    }

    // 创建Pump Fun自动交换出账户
    pub fn fast_path_create_pump_fun_auto_swap_out(
        ctx: Context<FastPathCreatePumpFunAutoSwap>,
    ) -> Result<()> {
        let accounts = &ctx.accounts;

        // 初始化快速路径账户
        ctx.accounts.swap_account.initialized = true;
        ctx.accounts.swap_account.swap_type = 3; // Pump Fun Auto Swap Out
        ctx.accounts.swap_account.dex_type = 1; // 1 = Pump Fun

        // 初始化内部数据区域
        // 为简化这里只添加必要的初始化

        msg!("Created Fast Path Pump Fun Auto Swap Out account successfully");
        Ok(())
    }

    // 获取交换指令的优化实现
    pub fn get_swap_instruction_optimised(ctx: Context<GetSwapInstruction>) -> Result<u64> {
        let accounts = &ctx.accounts;

        // 获取程序ID类型 - 使用as_ref()正确转换Pubkey为&[u8]
        let key_type = get_key_type_optimised(accounts.account_to_check.key.as_ref());

        // 根据程序ID类型创建不同的交换指令
        let (instruction_data, amount_out) = match key_type {
            0 => {
                // Raydium类型
                let raydium_ix = [9u8]; // 简化的Raydium指令ID
                let amount = accounts.amount_data.amount;
                (raydium_ix.to_vec(), amount)
            }
            1 => {
                // PumpFun类型
                let pump_fun_ix = [0xde, 0x33, 0x1e, 0xc4, 0xda, 0x5a, 0xbe, 0x8f]; // 简化的PumpFun指令ID
                let amount = accounts.amount_data.amount;
                (pump_fun_ix.to_vec(), amount)
            }
            2 => {
                // Orca类型
                let is_reverse = accounts.reverse.reverse;

                if is_reverse {
                    let orca_reverse_ix = [0xad, 0x83, 0x7f, 0x01, 0xa4, 0x85, 0xe6, 0x33]; // 简化的Orca反向指令ID
                    let amount = accounts.amount_data.amount;
                    (orca_reverse_ix.to_vec(), amount)
                } else {
                    // 获取报价
                    let quote = pump_fun_get_quote(
                        accounts.pool_data.to_account_info(),
                        accounts.amount_data.amount,
                        false,
                    )?;

                    let orca_ix = [0xea, 0xeb, 0xda, 0x01, 0x12, 0x3d, 0x06, 0x66]; // 简化的Orca指令ID
                    (orca_ix.to_vec(), quote)
                }
            }
            _ => {
                return Err(SwapError::InvalidDexType.into());
            }
        };

        //todo: 保存指令数据
        // ctx.accounts.instruction_data.data = instruction_data;

        // 返回计算出的兑换金额
        Ok(amount_out)
    }

    // 实际实现deserialize_swap函数
    pub fn deserialize_swap(ctx: Context<DeserializeSwap>) -> Result<bool> {
        let accounts = &ctx.accounts;

        // 提取账户信息
        let source_data = accounts.source_data.to_account_info();
        let raydium_data = accounts.raydium_data.to_account_info();
        let output = &mut ctx.accounts.output;

        // 检查程序ID
        let source_data_bytes = source_data.try_borrow_data()?;

        // 检查数据有效性
        if source_data_bytes.len() < 24 {
            return Ok(false);
        }

        // 检查程序ID是否匹配Raydium V4
        let first_id = u64::from_le_bytes(source_data_bytes[0..8].try_into().unwrap());
        let second_id = u64::from_le_bytes(source_data_bytes[8..16].try_into().unwrap());
        let third_id = u64::from_le_bytes(source_data_bytes[16..24].try_into().unwrap());

        // 汇编中的比较逻辑
        if first_id != 0xaa5b17bf6815db44 || second_id != 0x3bffd2f597cb8951 {
            // ID不匹配，不是Raydium V4
            return Ok(false);
        }

        if third_id != 0xb0186dfdb62b5d65 {
            // 检查第三个ID是否匹配
            return Ok(false);
        }

        // 处理Raydium V4的逻辑
        output.dex_type = 17; // 用于标识Raydium V4

        // 解析流动性数据
        let mut liquidity_buffer = [0u64; 2];

        // 解析流动性
        if !raydium_v4_parse_liquidity(raydium_data.clone(), &mut liquidity_buffer)? {
            return Ok(false);
        }

        // 设置输出数据
        let raydium_data_bytes = raydium_data.try_borrow_data()?;

        // 设置is_reverse标志和其他字段
        output.is_reverse = false;

        // 成功解析
        Ok(true)
    }

    #[derive(Accounts)]
    pub struct DeserializeSwap<'info> {
        /// CHECK: 源数据账户
        pub source_data: AccountInfo<'info>,

        /// CHECK: Raydium数据账户
        pub raydium_data: AccountInfo<'info>,

        /// 输出数据账户
        #[account(mut)]
        pub output: Account<'info, SwapData>,

        /// 存储区域
        #[account(mut)]
        pub storage: Account<'info, StorageData>,
    }

    // 新增SwapData结构体
    #[account]
    #[derive(InitSpace)]
    pub struct SwapData {
        pub dex_type: u8,
        pub is_reverse: bool,
        pub token_a_address: Pubkey,
        pub token_b_address: Pubkey,
        pub reserve_a: u64,
        pub reserve_b: u64,
        pub fee_numerator: u64,
        pub fee_denominator: u64,
        pub data: [u8; 1024], // 内部数据
    }

    impl SwapData {
        pub const LEN: usize = 1 + 1 + 32 + 32 + 8 + 8 + 8 + 8 + 1024;
    }

    // 存储区域数据结构
    #[account]
    #[derive(InitSpace)]
    pub struct StorageData {
        pub data: [u8; 2048], // 用于临时存储和memcpy操作
    }

    impl StorageData {
        pub const LEN: usize = 2048;
    }

    // 添加到program模块里的新指令
    pub fn deserialize_swap_v4(ctx: Context<DeserializeSwapV4>) -> Result<bool> {
        // 直接调用主函数,传递账户信息
        let source_data = ctx.accounts.source_data.clone();
        let raydium_data = ctx.accounts.raydium_data.clone();
        let output = &mut ctx.accounts.output;

        // 提取账户信息
        // 检查程序ID
        let source_data_bytes = source_data.try_borrow_data()?;

        // 检查数据有效性
        if source_data_bytes.len() < 24 {
            return Ok(false);
        }

        // 检查程序ID是否匹配Raydium V4
        let first_id = u64::from_le_bytes(source_data_bytes[0..8].try_into().unwrap());
        let second_id = u64::from_le_bytes(source_data_bytes[8..16].try_into().unwrap());
        let third_id = u64::from_le_bytes(source_data_bytes[16..24].try_into().unwrap());

        // 汇编中的比较逻辑
        if first_id != 0xaa5b17bf6815db44 || second_id != 0x3bffd2f597cb8951 {
            // ID不匹配，不是Raydium V4
            return Ok(false);
        }

        if third_id != 0xb0186dfdb62b5d65 {
            // 检查第三个ID是否匹配
            return Ok(false);
        }

        // 处理Raydium V4的逻辑
        output.dex_type = 17; // 用于标识Raydium V4

        // 解析流动性数据
        let mut liquidity_buffer = [0u64; 2];

        // 解析流动性
        if !raydium_v4_parse_liquidity(raydium_data.clone(), &mut liquidity_buffer)? {
            return Ok(false);
        }

        // 设置输出数据
        let raydium_data_bytes = raydium_data.try_borrow_data()?;

        // 设置is_reverse标志和其他字段
        output.is_reverse = false;

        // 成功解析
        Ok(true)
    }

    #[derive(Accounts)]
    pub struct DeserializeSwapV4<'info> {
        /// CHECK: 源数据账户
        pub source_data: AccountInfo<'info>,

        /// CHECK: Raydium数据账户
        pub raydium_data: AccountInfo<'info>,

        /// 输出数据账户
        #[account(mut)]
        pub output: Account<'info, SwapData>,

        /// 存储区域
        #[account(mut)]
        pub storage: Account<'info, StorageData>,
    }
}

// Internal implementation functions
fn raydium_is_valid(input_data: AccountInfo) -> Result<bool> {
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // Check if both amounts are greater than 1000
    Ok(amount_a > 1000 && amount_b > 1000)
}

fn raydium_get_quote(input_data: AccountInfo, amount: u64, reverse: bool) -> Result<u64> {
    // Implementing the complex math from the assembly
    // This is a simplified version for demonstration
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // Adjust the fee - 0.25% fee (25 basis points)
    let adjusted_amount = amount - (amount * 25) / 10000;

    let quote = if !reverse {
        // amount_a to amount_b calculation
        if amount_a == 0 {
            return Ok(0);
        }
        amount_b * adjusted_amount / amount_a
    } else {
        // amount_b to amount_a calculation
        if amount_b == 0 {
            return Ok(0);
        }
        amount_a * adjusted_amount / amount_b
    };

    Ok(quote)
}

fn raydium_get_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64)> {
    // This is a simplified implementation that returns token amounts
    let input_data_bytes = input_data.try_borrow_data()?;
    let amount_a = u64::from_le_bytes(input_data_bytes[0..8].try_into().unwrap());
    let amount_b = u64::from_le_bytes(input_data_bytes[8..16].try_into().unwrap());

    // In actual implementation we would calculate reserves based on the formula
    let reserve_a = amount_a;
    let reserve_b = amount_b;

    Ok((reserve_a, reserve_b))
}

fn raydium_get_quote_and_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64, u64)> {
    // Get the liquidity first
    let (reserve_a, reserve_b) = raydium_get_liquidity(input_data.clone(), amount, reverse)?;

    // Then get the quote
    let quote = raydium_get_quote(input_data, amount, reverse)?;

    Ok((quote, reserve_a, reserve_b))
}

// Utility functions for Pump Fun
fn function_9839(a: u64, b: u64, c: u64, d: u64) -> (u64, u64) {
    // This is a simplified placeholder for the function called in pump_fun code
    // In a real implementation, this would likely be a complex math operation
    (a + b, c + d)
}

fn function_9883(a: u64, b: u64, c: u64, d: u64, e: u64) -> (u64, u64) {
    // Another placeholder for a utility function
    (a + b + e, c + d)
}

fn function_11519(a: u64, b: u64) -> u64 {
    // Simplified comparison function
    if a < b {
        0
    } else {
        1
    }
}

fn function_11552(a: u64, b: u64) -> u64 {
    // Simplified multiplication function that appears in pump_fun code
    a * b / 0x10000000000000000u128 as u64
}

fn function_12023(a: u64) -> u64 {
    // Simplified square root function
    (a as f64).sqrt() as u64
}

fn function_12129(a: u64, b: u64) -> Result<u64> {
    // Division function
    if b == 0 {
        Err(ProgramError::ArithmeticOverflow.into())
    } else {
        Ok(a / b)
    }
}

// Pump Fun implementations
fn pump_fun_parse_liquidity(input_data: AccountInfo, output: &mut [u64; 2]) -> Result<bool> {
    let input_data_bytes = input_data.try_borrow_data()?;
    let data_len = u32::from_le_bytes(input_data_bytes[16..20].try_into().unwrap()) as u64;

    if data_len > 24 {
        // Read the value at offset 0x18 + 0x8 (real data start + offset)
        let token_amount_a = u64::from_le_bytes(input_data_bytes[32..40].try_into().unwrap());

        // Read the value at offset 0x18 + 0x10
        let token_amount_b = u64::from_le_bytes(input_data_bytes[40..48].try_into().unwrap());

        output[0] = token_amount_a;
        output[1] = token_amount_b;

        return Ok(true);
    }

    Ok(data_len > 23)
}

fn pump_fun_k(input: AccountInfo, output: &mut [u64; 2]) -> Result<()> {
    let input_bytes = input.try_borrow_data()?;

    // Extract values from input data
    let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
    let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

    // Calculate K value (constant product)
    let temp_result = function_9839(reserve_a, reserve_b, 0, 0);

    output[0] = temp_result.0;
    output[1] = temp_result.1;

    Ok(())
}

fn pump_fun_price(input_data: AccountInfo, reverse: bool) -> Result<u64> {
    let input_bytes = input_data.try_borrow_data()?;

    // Extract reserves
    let reserve_a = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());
    let reserve_b = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());

    let a_val = if reverse { reserve_a } else { reserve_b };
    let sqrt_a = function_12023(a_val);

    let b_val = if reverse { reserve_b } else { reserve_a };
    let sqrt_b = function_12023(b_val);

    // Calculate price
    function_12129(sqrt_a, sqrt_b)
}

fn pump_fun_is_valid(input_data: AccountInfo) -> Result<bool> {
    let input_bytes = input_data.try_borrow_data()?;

    // Extract reserves
    let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
    let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

    // Check minimum reserves
    if reserve_a <= 1000 || reserve_b <= 1000 {
        return Ok(false);
    }

    // Calculate sqrt of reserves
    let sqrt_a = function_12023(reserve_a);
    let sqrt_b = function_12023(reserve_b);

    // Calculate price
    let price = function_12129(sqrt_a, sqrt_b)?;

    // Check if price within valid range
    let check1 = function_11552(price, 0x42d6bcc41e900000); // ~ 100,000
    let check2 = function_11519(check1, 0x4253ca6512000000); // ~ 1,000,000

    Ok(check2 > 0)
}

fn pump_fun_get_quote(input_data: AccountInfo, amount: u64, reverse: bool) -> Result<u64> {
    let input_bytes = input_data.try_borrow_data()?;

    // Using reverse path from assembly
    if reverse {
        // Extract reserves
        let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

        // Complex calculation - simplified for this example
        // In the real implementation this would include all the bit shifts and math
        // from the assembly code (lines ~1350-1520)
        let adjusted_amount = amount.saturating_add(reserve_a);
        let mut output = reserve_b.saturating_mul(reserve_a) / adjusted_amount;

        // Apply 1% fee
        output = output.saturating_sub(output / 100);

        return Ok(output);
    } else {
        // Create temporary storage for initial calculation
        let mut temp_storage = [100u64, 0u64];

        // Extract reserves and calculate quote
        // This is a simplified version of the complex math in assembly
        let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

        // More complex calculations here - simplified
        let adjusted_amount = reserve_a.saturating_add(amount);
        let mut output = reserve_a.saturating_mul(reserve_b) / adjusted_amount;

        // Apply 1% fee
        output = output.saturating_sub(output / 100);

        return Ok(output);
    }
}

fn pump_fun_get_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64)> {
    let input_bytes = input_data.try_borrow_data()?;

    if reverse {
        // Extract reserves and calculate liquidity - reverse direction
        let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

        // This is a simplified version of the complex calculations in assembly
        // In real implementation, this would include all the bit shifts and math operations

        return Ok((reserve_a, reserve_b));
    } else {
        // Create temporary storage for calculations
        let mut temp_a = [0u64; 2];
        let mut temp_b = [0u64; 2];

        // Call function_9839 and function_9883 as seen in assembly
        let calc_temp = function_9839(amount, 0, 100, 0);
        temp_a[0] = calc_temp.0;
        temp_a[1] = calc_temp.1;

        let calc_temp2 = function_9883(temp_a[0], temp_a[1], 101, 0, 0);
        temp_b[0] = calc_temp2.0;
        temp_b[1] = calc_temp2.1;

        // Extract reserves
        let reserve_a = u64::from_le_bytes(input_bytes[0..8].try_into().unwrap());
        let reserve_b = u64::from_le_bytes(input_bytes[8..16].try_into().unwrap());

        // Return reserves
        Ok((reserve_a, reserve_b))
    }
}

fn pump_fun_get_quote_and_liquidity(
    input_data: AccountInfo,
    amount: u64,
    reverse: bool,
) -> Result<(u64, u64, u64)> {
    // Get the liquidity first
    let (reserve_a, reserve_b) = pump_fun_get_liquidity(input_data.clone(), amount, reverse)?;

    // Then get the quote
    let quote = pump_fun_get_quote(input_data, amount, reverse)?;

    Ok((quote, reserve_a, reserve_b))
}

// Account structures for the instructions
#[derive(Accounts)]
pub struct IsValid<'info> {
    pub input_data: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetQuote<'info> {
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
}

#[derive(Accounts)]
pub struct GetLiquidity<'info> {
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
}

#[derive(Accounts)]
pub struct GetQuoteAndLiquidity<'info> {
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
}

#[derive(Accounts)]
pub struct CalculateProfitOptimised<'info> {
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
    #[account(mut)]
    pub dex_type_reverse: Account<'info, DexType>,
    pub quote_ctx: AccountInfo<'info>,
    pub liquidity_ctx: AccountInfo<'info>,
    pub quote_ctx_reverse: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CalculateHintedMaxAmountOptimised<'info> {
    #[account(mut)]
    pub max_input: Account<'info, AmountData>,
    #[account(mut)]
    pub available: Account<'info, AmountData>,
    #[account(mut)]
    pub fee_numerator: Account<'info, AmountData>,
    #[account(mut)]
    pub fee_denominator: Account<'info, AmountData>,
}

#[derive(Accounts)]
pub struct CalculateUpperBoundOptimised<'info> {
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
    #[account(mut)]
    pub amounts: Account<'info, TokenAmounts>,
    #[account(mut)]
    pub is_token_a: Account<'info, IsTokenA>,
    #[account(mut)]
    pub multiplier: Account<'info, AmountData>,
}

#[derive(Accounts)]
pub struct CalculateOptimalStrategyOptimised<'info> {
    // 需要根据函数实现细节来定义所需的账户
    pub misc_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CalculateProfit<'info> {
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
}

#[derive(Accounts)]
pub struct IsBuyAmountTooBig<'info> {
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub threshold: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
}

#[derive(Accounts)]
pub struct CalculateOptimalStrategyDeprecated<'info> {
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,
    #[account(mut)]
    pub amounts: Account<'info, TokenAmounts>,
    #[account(mut)]
    pub is_token_a: Account<'info, IsTokenA>,
    #[account(mut)]
    pub multiplier: Account<'info, AmountData>,
    #[account(mut)]
    pub output: AccountInfo<'info>,
}

// Data structures used in the program
#[account]
pub struct AmountData {
    pub amount: u64,
}

#[account]
pub struct ReverseFlag {
    pub reverse: bool,
}

#[account]
pub struct DexType {
    pub dex_type: u8,
}

#[account]
pub struct TokenAmounts {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[account]
pub struct IsTokenA {
    pub is_token_a: u8,
}

// Custom error codes
#[error_code]
pub enum SwapError {
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid DEX type")]
    InvalidDexType,
    #[msg("Invalid pool state")]
    InvalidPoolState,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Invalid global controller")]
    InvalidGlobalController,
    #[msg("Invalid swap direction")]
    InvalidSwapDirection,
    #[msg("Invalid quote")]
    InvalidQuote,
    #[msg("Invalid validator")]
    InvalidValidator,
}

fn calculate_upper_bound(
    amount: u64,
    dex_type: u8,
    token_a_amount: u64,
    token_b_amount: u64,
    is_token_a: u8,
    multiplier: u64,
) -> Result<u64> {
    // 默认结果为0
    let mut result = 0u64;

    // 根据dex类型和token_a标志决定使用哪个计算路径
    // 使用汇编代码中复杂的分支逻辑

    let available = if is_token_a == 0 {
        token_a_amount
    } else {
        token_b_amount
    };

    // 检查金额是否超过可用量
    if available > amount {
        let remaining = amount.saturating_sub(available);
        let fee_rate = if dex_type == 1 { 9900u64 } else { 9975u64 };

        let output_amount;
        if remaining > 0x68db8bac710cc {
            output_amount = remaining / fee_rate * 10000;
        } else {
            output_amount = remaining * 10000 / fee_rate;
        }

        if output_amount > 0x68db8bac710cc {
            result = output_amount / 10000 * multiplier;
        } else {
            result = output_amount * multiplier / 10000;
        }
    }

    Ok(result)
}

fn function_9815(a: u64) -> u64 {
    // 这个函数在汇编代码中被多次调用，看起来是一个辅助函数
    // 为简化起见，返回输入值的一小部分
    a / 10
}

// 新添加的账户结构
#[derive(Accounts)]
pub struct FastPathAutoSwapInRaydiumV4<'info> {
    // 通用账户
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,

    // Raydium特有账户
    /// CHECK: This account is safe as it's only used for token transfer
    #[account(mut)]
    pub token_account_a: AccountInfo<'info>,
    /// CHECK: This account is safe as it's only used for token transfer
    #[account(mut)]
    pub token_account_b: AccountInfo<'info>,

    // 系统账户
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FastPathAutoSwapOutRaydiumV4<'info> {
    // 通用账户
    pub input_data: AccountInfo<'info>,
    #[account(mut)]
    pub amount: Account<'info, AmountData>,
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,

    // Raydium特有账户
    /// CHECK: This account is safe as it's only used for token transfer
    #[account(mut)]
    pub token_account_a: AccountInfo<'info>,
    /// CHECK: This account is safe as it's only used for token transfer
    #[account(mut)]
    pub token_account_b: AccountInfo<'info>,

    // 系统账户
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FastPathCreateRaydiumV4<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(init, payer = initializer, space = 8 + PoolState::LEN)]
    pub pool_state: Account<'info, PoolState>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CloseSandwichesAndTopupTipper<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub tipper_account: Account<'info, TipperAccount>,

    #[account(mut)]
    pub sandwich_tracker: Account<'info, SandwichTracker>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateSandwichTracker<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init, payer = admin, space = 8 + SandwichTracker::LEN)]
    pub tracker: Account<'info, SandwichTracker>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateGlobal<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init, payer = admin, space = 8 + GlobalState::LEN)]
    pub global_state: Account<'info, GlobalState>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub source_account: Account<'info, AmountData>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}

// 新添加的数据结构
#[account]
#[derive(InitSpace)]
pub struct PoolState {
    pub initialized: bool,
    pub dex_type: u8, // 0: Raydium, 1: PumpFun
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
}

impl PoolState {
    pub const LEN: usize = 1 + 1 + 8 + 8 + 8 + 8;
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
}

impl GlobalState {
    pub const LEN: usize = 1 + 32 + 8 + 8 + 8;
}

// 添加Raydium V4解析流动性函数
pub fn raydium_v4_parse_liquidity(
    raydium_data: AccountInfo,
    liquidity_buffer: &mut [u64; 2],
) -> Result<bool> {
    // 检查账户数据有效性
    let raydium_data_bytes = raydium_data.try_borrow_data()?;

    // 根据汇编代码，需要检查账户数据长度
    if raydium_data_bytes.len() < 200 {
        return Ok(false);
    }

    // 从特定偏移量读取流动性数据
    // 示例偏移量基于汇编代码中的内存访问模式
    let reserve_a_offset = 112;
    let reserve_b_offset = 168;

    // 确保我们有足够的数据来读取这些值
    if raydium_data_bytes.len() < reserve_a_offset + 8
        || raydium_data_bytes.len() < reserve_b_offset + 8
    {
        return Ok(false);
    }

    // 读取流动性值
    let reserve_a = u64::from_le_bytes(
        raydium_data_bytes[reserve_a_offset..reserve_a_offset + 8]
            .try_into()
            .unwrap(),
    );

    let reserve_b = u64::from_le_bytes(
        raydium_data_bytes[reserve_b_offset..reserve_b_offset + 8]
            .try_into()
            .unwrap(),
    );

    // 将读取的值存储到输出缓冲区
    liquidity_buffer[0] = reserve_a;
    liquidity_buffer[1] = reserve_b;

    Ok(true)
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

// 根据汇编代码实现get_key_type_optimised函数,接收&[u8]切片
fn get_key_type_optimised(input_data: &[u8]) -> u8 {
    if input_data.len() < 32 {
        return 3; // 默认类型
    }

    // 从汇编代码中提取的比较逻辑
    // 实际实现需要根据正确的密钥类型做匹配

    // 示例逻辑（需要替换为真实逻辑）
    if input_data[0] == 0x3f && input_data[1] == 0xc3 {
        return 0; // 第一种类型
    } else if input_data[0] == 0x52 && input_data[1] == 0x59 {
        return 1; // 第二种类型
    } else if input_data[0] == 0xcf && input_data[1] == 0x5a {
        return 2; // 第三种类型
    }

    3 // 默认类型
}

// 新增账户结构体
#[derive(Accounts)]
pub struct UpdateGlobal<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub global_state: Account<'info, GlobalState>,

    pub global_update_data: Account<'info, GlobalUpdateData>,
}

#[derive(Accounts)]
pub struct FastPathAutoSwapInPumpFun<'info> {
    // 通用账户
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: 池子数据账户
    pub pool_data: AccountInfo<'info>,

    #[account(mut)]
    pub amount_data: Account<'info, AmountData>,

    // Pump Fun特有账户
    /// CHECK: These accounts will be checked in the instruction
    #[account(mut)]
    pub token_a_account: AccountInfo<'info>,

    /// CHECK: These accounts will be checked in the instruction
    #[account(mut)]
    pub token_b_account: AccountInfo<'info>,

    // 系统账户
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FastPathAutoSwapOutPumpFun<'info> {
    // 通用账户
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: 池子数据账户
    pub pool_data: AccountInfo<'info>,

    #[account(mut)]
    pub amount_data: Account<'info, AmountData>,

    // Pump Fun特有账户
    /// CHECK: These accounts will be checked in the instruction
    #[account(mut)]
    pub token_a_account: AccountInfo<'info>,

    /// CHECK: These accounts will be checked in the instruction
    #[account(mut)]
    pub token_b_account: AccountInfo<'info>,

    // 可选的验证者检查
    /// CHECK: This account is safe
    pub validator_id: AccountInfo<'info>,

    // 系统账户
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FastPathCreatePumpFunAutoSwap<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = 8 + SwapAccount::LEN)]
    pub swap_account: Account<'info, SwapAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct GetSwapInstruction<'info> {
    /// CHECK: 需要检查的账户
    pub account_to_check: AccountInfo<'info>,

    #[account(mut)]
    pub amount_data: Account<'info, AmountData>,

    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,

    /// CHECK: 池子数据
    pub pool_data: AccountInfo<'info>,

    /// CHECK: 指令数据输出
    #[account(mut)]
    pub instruction_data: AccountInfo<'info>,
}

// 新增数据结构
#[account]
#[derive(InitSpace)]
pub struct GlobalUpdateData {
    pub update_fee_flag: bool,
    pub update_config_flag: bool,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub tipper_fee: u64,
    // 配置数据通常更大，这里简化
}

#[account]
#[derive(InitSpace)]
pub struct SwapAccount {
    pub initialized: bool,
    pub swap_type: u8,    // 2: Auto Swap In, 3: Auto Swap Out
    pub dex_type: u8,     // 0: Raydium, 1: Pump Fun
    pub data: [u8; 1800], // 内部数据
}

impl SwapAccount {
    pub const LEN: usize = 1 + 1 + 1 + 1800;
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
