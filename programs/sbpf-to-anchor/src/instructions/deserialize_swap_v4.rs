use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// Raydium V4交易数据反序列化的账户结构
/// 用于解析和验证Raydium V4协议的交易数据
#[derive(Accounts)]
pub struct DeserializeSwapV4<'info> {
    /// 源数据账户
    /// 包含需要验证的程序ID和其他元数据
    /// CHECK: 仅用于读取数据，无需验证所有权
    pub source_data: AccountInfo<'info>,

    /// Raydium V4协议的数据账户
    /// 包含流动性池和交易参数信息
    /// CHECK: 仅用于读取数据，无需验证所有权
    pub raydium_data: AccountInfo<'info>,

    /// 输出数据账户
    /// 用于存储解析后的交易数据
    #[account(mut)]
    pub output: Account<'info, SwapData>,

    /// 存储区域账户
    /// 用于临时数据存储和处理
    #[account(mut)]
    pub storage: Account<'info, StorageData>,
}

/// 反序列化Raydium V4交易数据的函数
///
/// # 功能特点
/// * 验证Raydium V4程序ID
/// * 解析流动性池数据
/// * 提取代币地址和费率信息
/// * 转换为标准交易数据格式
///
/// # 安全考虑
/// * 验证程序ID的正确性
/// * 检查数据长度和有效性
/// * 安全的数据复制和转换
///
/// # 魔术数字验证
/// * 第一部分: 0xaa5b17bf6815db44
/// * 第二部分: 0x3bffd2f597cb8951
/// * 第三部分: 0xb0186dfdb62b5d65
///
/// # 返回值
/// * `Result<bool>` - 成功返回true，失败返回false
pub fn deserialize_swap_v4(ctx: Context<DeserializeSwapV4>) -> Result<bool> {
    // 获取账户引用
    let source_data = ctx.accounts.source_data.clone();
    let raydium_data = ctx.accounts.raydium_data.clone();
    let output = &mut ctx.accounts.output;

    // 读取源数据账户内容
    // 用于验证程序ID和其他元数据
    let source_data_bytes = source_data.try_borrow_data()?;

    // 验证数据长度
    // 确保至少包含24字节的程序ID
    if source_data_bytes.len() < 24 {
        return Ok(false);
    }

    // 提取并验证Raydium V4的程序ID
    // 使用小端字节序读取三个8字节的ID部分
    let first_id = u64::from_le_bytes(source_data_bytes[0..8].try_into().unwrap());
    let second_id = u64::from_le_bytes(source_data_bytes[8..16].try_into().unwrap());
    let third_id = u64::from_le_bytes(source_data_bytes[16..24].try_into().unwrap());

    // 验证程序ID的第一部分和第二部分
    if first_id != 0xaa5b17bf6815db44 || second_id != 0x3bffd2f597cb8951 {
        return Ok(false);
    }

    // 验证程序ID的第三部分
    if third_id != 0xb0186dfdb62b5d65 {
        return Ok(false);
    }

    // 设置DEX类型为Raydium V4
    output.dex_type = 17;

    // 初始化流动性缓冲区
    // 用于存储代币A和代币B的流动性数据
    let mut liquidity_buffer = [0u64; 2];

    // 解析Raydium V4的流动性数据
    // 使用专门的解析函数处理
    if !raydium_v4_parse_liquidity(raydium_data.clone(), &mut liquidity_buffer)? {
        return Ok(false);
    }

    // 读取Raydium数据账户内容
    let raydium_data_bytes = raydium_data.try_borrow_data()?;

    // 设置交易方向和流动性数据
    output.is_reverse = false; // 设置默认交易方向
    output.reserve_a = liquidity_buffer[0]; // 设置代币A的流动性
    output.reserve_b = liquidity_buffer[1]; // 设置代币B的流动性

    // 从Raydium数据中提取代币地址和费率信息
    // 注意：偏移量基于协议规范，需要定期验证
    if raydium_data_bytes.len() >= 32 {
        // 提取代币A的地址（0-32字节）
        output.token_a_address = Pubkey::new(&raydium_data_bytes[0..32]);
    }
    if raydium_data_bytes.len() >= 64 {
        // 提取代币B的地址（32-64字节）
        output.token_b_address = Pubkey::new(&raydium_data_bytes[32..64]);
    }
    if raydium_data_bytes.len() >= 72 {
        // 提取费率分子（64-72字节）
        output.fee_numerator = u64::from_le_bytes(raydium_data_bytes[64..72].try_into().unwrap());
    }
    if raydium_data_bytes.len() >= 80 {
        // 提取费率分母（72-80字节）
        output.fee_denominator = u64::from_le_bytes(raydium_data_bytes[72..80].try_into().unwrap());
    }

    // 复制原始Raydium数据到输出缓冲区
    // 用于保留完整的交易信息
    let copy_len = std::cmp::min(raydium_data_bytes.len(), output.data.len());
    if copy_len > 0 {
        output.data[..copy_len].copy_from_slice(&raydium_data_bytes[..copy_len]);
    }

    // 返回成功标志
    Ok(true)
}
