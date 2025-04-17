use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 通用DEX交易数据反序列化的账户结构
/// 支持多种DEX协议的数据解析，包括Raydium V4、Raydium CP和Pump.fun
#[derive(Accounts)]
pub struct DeserializeSwap<'info> {
    /// 源数据账户
    /// 包含程序ID，用于识别DEX类型
    /// CHECK: 仅用于读取数据，无需验证所有权
    pub source_data: AccountInfo<'info>,

    /// DEX特定的数据账户
    /// 可能是Raydium流动性池、Pump.fun债券曲线等
    /// CHECK: 仅用于读取数据，无需验证所有权
    pub dex_data: AccountInfo<'info>,

    /// 输出数据账户
    /// 用于存储标准化后的交易数据
    #[account(mut)]
    pub output: Account<'info, SwapData>,

    /// 存储区域账户
    /// 用于临时数据处理，初始化为空以防止数据污染
    #[account(zero)] // 使用zero确保初始为空,防止残留数据
    pub storage: AccountLoader<'info, StorageData>,
}

/// 通用DEX交易数据反序列化函数
///
/// # 功能特点
/// * 支持多种DEX协议
/// * 自动识别DEX类型
/// * 统一的数据输出格式
/// * 安全的数据处理
///
/// # 支持的DEX类型
/// * Raydium V4 (DEX类型 17)
///   - 程序ID: [0xaa5b17bf6815db44, 0x3bffd2f597cb8951, 0xb0186dfdb62b5d65]
/// * Raydium CP (DEX类型 13)
///   - 程序ID: 0x5259294f8b5a2aa9
/// * Pump.fun (DEX类型 12)
///   - 程序ID: 0xcf5a6693f6e05601
///
/// # 返回值
/// * `Result<bool>` - 成功解析返回true，不支持的DEX或解析失败返回false
pub fn deserialize_swap(ctx: Context<DeserializeSwap>) -> Result<bool> {
    let accounts = &ctx.accounts;

    // 获取所需账户的引用
    let source_data_account = accounts.source_data.to_account_info();
    let dex_data_account = accounts.dex_data.to_account_info();
    let output = &mut ctx.accounts.output;
    let storage_data = &mut accounts.storage.load_mut()?;

    // 读取源数据账户内容
    // 用于验证程序ID和识别DEX类型
    let source_data_bytes = source_data_account.try_borrow_data()?;

    // 验证数据长度
    // 确保至少包含24字节的程序ID
    if source_data_bytes.len() < 24 {
        return Ok(false);
    }

    // 提取程序ID
    // 使用小端字节序读取ID部分
    let first_id = u64::from_le_bytes(source_data_bytes[0..8].try_into().unwrap());
    let second_id = u64::from_le_bytes(source_data_bytes[8..16].try_into().unwrap());
    let third_id = u64::from_le_bytes(source_data_bytes[16..24].try_into().unwrap());

    // 根据程序ID识别和处理不同的DEX类型
    if first_id == 0xaa5b17bf6815db44
        && second_id == 0x3bffd2f597cb8951
        && third_id == 0xb0186dfdb62b5d65
    {
        // === Raydium V4 处理逻辑 ===
        output.dex_type = 17; // 设置为Raydium V4类型

        // 解析流动性数据
        let mut liquidity_buffer = [0u64; 2];
        if !raydium_v4_parse_liquidity(dex_data_account.clone(), &mut liquidity_buffer)? {
            return Ok(false);
        }

        // 读取DEX数据
        let dex_data_bytes = dex_data_account.try_borrow_data()?;

        // 设置基本参数
        output.is_reverse = false;
        output.reserve_a = liquidity_buffer[0];
        output.reserve_b = liquidity_buffer[1];

        // 提取代币地址和费率信息
        if dex_data_bytes.len() >= 32 {
            output.token_a_address = Pubkey::new(&dex_data_bytes[0..32]);
        }
        if dex_data_bytes.len() >= 64 {
            output.token_b_address = Pubkey::new(&dex_data_bytes[32..64]);
        }
        if dex_data_bytes.len() >= 72 {
            output.fee_numerator = u64::from_le_bytes(dex_data_bytes[64..72].try_into().unwrap());
        }
        if dex_data_bytes.len() >= 80 {
            output.fee_denominator = u64::from_le_bytes(dex_data_bytes[72..80].try_into().unwrap());
        }

        // 复制原始数据
        let copy_len = std::cmp::min(dex_data_bytes.len(), output.data.len());
        if copy_len > 0 {
            output.data[..copy_len].copy_from_slice(&dex_data_bytes[..copy_len]);
        }
        Ok(true)
    } else if first_id == 0x5259294f8b5a2aa9 {
        // === Raydium CP 处理逻辑 ===
        output.dex_type = 13; // 设置为Raydium CP类型

        // 解析CP特定的流动性数据
        let mut liquidity_buffer = [0u64; 2];
        if !raydium_cp_parse_liquidity(dex_data_account.clone(), &mut liquidity_buffer)? {
            return Ok(false);
        }

        // 读取DEX数据
        let dex_data_bytes = dex_data_account.try_borrow_data()?;

        // 设置基本参数
        output.is_reverse = false;
        output.reserve_a = liquidity_buffer[0];
        output.reserve_b = liquidity_buffer[1];

        // TODO: 实现CP特定的代币地址和费率提取
        // output.token_a_address = ...;
        // output.token_b_address = ...;
        // output.fee_numerator = ...;
        // output.fee_denominator = ...;

        // 复制原始数据
        let copy_len = std::cmp::min(dex_data_bytes.len(), output.data.len());
        if copy_len > 0 {
            output.data[..copy_len].copy_from_slice(&dex_data_bytes[..copy_len]);
        }
        Ok(true)
    } else if first_id == 0xcf5a6693f6e05601 {
        // === Pump.fun 处理逻辑 ===
        output.dex_type = 12; // 设置为Pump.fun类型

        // 解析Pump.fun特定的流动性数据
        let mut liquidity_buffer = [0u64; 2];
        if !pump_fun_parse_liquidity(dex_data_account.clone(), &mut liquidity_buffer)? {
            return Ok(false);
        }

        // 读取DEX数据
        let dex_data_bytes = dex_data_account.try_borrow_data()?;

        // 设置基本参数
        output.is_reverse = false;
        output.reserve_a = liquidity_buffer[0];
        output.reserve_b = liquidity_buffer[1];

        // TODO: 实现Pump.fun特定的代币地址和费率提取
        // output.token_a_address = ...;
        // output.token_b_address = ...;
        // output.fee_numerator = ...;
        // output.fee_denominator = ...;

        // 复制原始数据
        let copy_len = std::cmp::min(dex_data_bytes.len(), output.data.len());
        if copy_len > 0 {
            output.data[..copy_len].copy_from_slice(&dex_data_bytes[..copy_len]);
        }
        Ok(true)
    } else {
        // 不支持的DEX类型
        Ok(false)
    }
}
