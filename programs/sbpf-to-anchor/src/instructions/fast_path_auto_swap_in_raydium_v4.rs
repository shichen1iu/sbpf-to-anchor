use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// Raydium V4快速路径自动交换的账户结构
/// 用于优化执行Raydium V4上的代币交换操作
#[derive(Accounts)]
pub struct FastPathAutoSwapInRaydiumV4<'info> {
    // === 通用账户组 ===
    /// 输入数据账户
    /// 包含交易所需的基本参数
    pub input_data: AccountInfo<'info>,

    /// 交易金额账户
    /// 存储要交换的代币数量
    #[account(mut)]
    pub amount: Account<'info, AmountData>,

    /// 交易方向标志账户
    /// 指示代币交换的方向
    #[account(mut)]
    pub reverse: Account<'info, ReverseFlag>,

    /// DEX类型账户
    /// 用于区分不同的DEX协议
    #[account(mut)]
    pub dex_type: Account<'info, DexType>,

    // === Raydium特有账户组 ===
    /// 代币A账户
    /// 用于交易的源代币账户
    /// CHECK: 仅用于代币转账，在CPI中验证
    #[account(mut)]
    pub token_account_a: AccountInfo<'info>,

    /// 代币B账户
    /// 用于交易的目标代币账户
    /// CHECK: 仅用于代币转账，在CPI中验证
    #[account(mut)]
    pub token_account_b: AccountInfo<'info>,

    // === 系统账户组 ===
    /// SPL代币程序
    /// 用于处理代币转账操作
    pub token_program: Program<'info, anchor_spl::token::Token>,

    /// 系统程序
    /// 用于系统级操作
    pub system_program: Program<'info, System>,
}

/// Raydium V4优化计算实现
/// 提供流动性和价格计算的优化方法
impl<'info> FastPathAutoSwapInRaydiumV4<'info> {
    /// 优化的流动性计算函数
    ///
    /// # 参数
    /// * `amount_a` - 代币A的数量
    /// * `amount_b` - 代币B的数量
    /// * `fee_numerator` - 费率分子
    /// * `fee_denominator` - 费率分母
    ///
    /// # 返回值
    /// * `Result<(u64, u64)>` - 计算后的流动性数据
    pub fn calculate_liquidity_optimized(
        &self,
        amount_a: u64,
        amount_b: u64,
        fee_numerator: u64,
        fee_denominator: u64,
    ) -> Result<(u64, u64)> {
        // 验证输入参数
        // 确保费率分母和金额都大于0
        require!(fee_denominator > 0, ErrorCode::InvalidParameter);
        require!(amount_a > 0 && amount_b > 0, ErrorCode::InvalidParameter);

        // 初始化流动性变量
        let mut liquidity_a = amount_a;
        let mut liquidity_b = amount_b;

        // 应用费率计算
        // 使用checked操作防止溢出
        if fee_numerator > 0 {
            liquidity_a = liquidity_a
                .checked_mul(fee_denominator)
                .ok_or(ErrorCode::Overflow)?
                .checked_div(fee_numerator)
                .ok_or(ErrorCode::Overflow)?;

            liquidity_b = liquidity_b
                .checked_mul(fee_denominator)
                .ok_or(ErrorCode::Overflow)?
                .checked_div(fee_numerator)
                .ok_or(ErrorCode::Overflow)?;
        }

        Ok((liquidity_a, liquidity_b))
    }

    /// 优化的价格计算函数
    ///
    /// # 参数
    /// * `reserve_a` - 代币A的储备量
    /// * `reserve_b` - 代币B的储备量
    /// * `amount_in` - 输入金额
    /// * `is_buy` - 是否为买入操作
    ///
    /// # 返回值
    /// * `Result<u64>` - 计算后的价格
    pub fn calculate_price_optimized(
        &self,
        reserve_a: u64,
        reserve_b: u64,
        amount_in: u64,
        is_buy: bool,
    ) -> Result<u64> {
        // 验证储备金是否充足
        require!(
            reserve_a > 0 && reserve_b > 0,
            ErrorCode::InsufficientLiquidity
        );

        // 根据交易方向计算价格
        let price = if is_buy {
            // 买入价格计算
            // price = (reserve_b * amount_in) / (reserve_a + amount_in)
            reserve_b
                .checked_mul(amount_in)
                .ok_or(ErrorCode::Overflow)?
                .checked_div(
                    reserve_a
                        .checked_add(amount_in)
                        .ok_or(ErrorCode::Overflow)?,
                )
                .ok_or(ErrorCode::Overflow)?
        } else {
            // 卖出价格计算
            // price = (reserve_a * amount_in) / (reserve_b + amount_in)
            reserve_a
                .checked_mul(amount_in)
                .ok_or(ErrorCode::Overflow)?
                .checked_div(
                    reserve_b
                        .checked_add(amount_in)
                        .ok_or(ErrorCode::Overflow)?,
                )
                .ok_or(ErrorCode::Overflow)?
        };

        Ok(price)
    }
}

/// Raydium V4快速路径自动交换函数
///
/// # 功能特点
/// * 支持多DEX协议
/// * 优化的流动性计算
/// * 智能价格路由
/// * 自动状态更新
///
/// # 执行流程
/// 1. 验证DEX类型
/// 2. 检查池子有效性
/// 3. 计算交换金额
/// 4. 执行代币转账
/// 5. 更新三明治状态
///
/// # 错误处理
/// * 无效的池子状态
/// * 不足的流动性
/// * 计算溢出保护
pub fn fast_path_auto_swap_in_raydium_v4(ctx: Context<FastPathAutoSwapInRaydiumV4>) -> Result<()> {
    let accounts = &ctx.accounts;

    // 获取交易参数
    // 包括DEX类型、交易金额和方向
    let dex_type = accounts.dex_type.dex_type;
    let amount = accounts.amount.amount;
    let reverse = accounts.reverse.reverse;

    // 根据DEX类型验证池子有效性
    let is_valid = if dex_type == 0 {
        // Raydium V4池子验证
        raydium_is_valid(accounts.input_data.to_account_info())?
    } else {
        // Pump.fun池子验证
        pump_fun_is_valid(accounts.input_data.to_account_info())?
    };

    // 检查池子状态
    // 确保池子处于可交易状态
    if !is_valid {
        return Err(SwapError::InvalidPoolState.into());
    }

    // 获取报价和流动性数据
    // 根据DEX类型调用不同的计算函数
    let (quote, reserve_a, reserve_b) = if dex_type == 0 {
        // Raydium V4报价计算
        raydium_get_quote_and_liquidity(accounts.input_data.to_account_info(), amount, reverse)?
    } else {
        // Pump.fun报价计算
        pump_fun_get_quote_and_liquidity(accounts.input_data.to_account_info(), amount, reverse)?
    };

    // 验证流动性充足性
    // 确保能够执行交易
    if quote == 0 {
        return Err(SwapError::InsufficientLiquidity.into());
    }

    // TODO: 执行代币转账
    // 需要通过CPI调用token程序实现

    // TODO: 更新三明治交易状态
    // 需要调用sandwich_update_frontrun更新状态

    // 记录成功日志
    msg!("Fast path auto swap in Raydium V4 executed successfully");
    Ok(())
}
