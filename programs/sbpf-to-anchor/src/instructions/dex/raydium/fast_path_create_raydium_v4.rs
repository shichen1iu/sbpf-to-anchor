use crate::instructions::dex::raydium::state::FastPathCreateRaydiumParams;
use anchor_lang::prelude::*;

/// Raydium快速路径创建池指令
///
/// 此指令用于创建Raydium V4池账户和初始化相关数据
/// 包含账户创建和数据初始化逻辑
#[derive(Accounts)]
pub struct FastPathCreateRaydiumV4<'info> {
    /// 账户创建的付款者
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// 将要创建的Raydium池账户
    #[account(init, 
        payer = payer, 
        space = 2360, 
        owner = system_program.key()
    )]
    pub raydium_pool: AccountInfo<'info>,
    
    /// 用于存储池数据的账户
    #[account(init,
        payer = payer,
        space = 2360,
        owner = system_program.key()
    )]
    pub pool_data_account: AccountInfo<'info>,
    
    /// 系统程序，用于创建账户
    pub system_program: Program<'info, System>,
}

/// 创建Raydium V4池账户和初始化相关数据
///
/// 此函数与sBPF中的fast_path_create_raydium_v4对应
/// 处理池账户创建和数据初始化
pub fn fast_path_create_raydium_v4(
    ctx: Context<FastPathCreateRaydiumV4>,
    params: FastPathCreateRaydiumParams,
) -> Result<()> {
    // 获取参数
    let pool_id = params.pool_id;
    let config_data = params.config_data;

    // 由于使用了init宏，Anchor已经自动创建了账户，
    // 所以我们不需要再手动创建账户了

    // 初始化池数据
    let pool_data = &ctx.accounts.pool_data_account;

    // 写入池ID
    let mut pool_data_bytes = pool_data.try_borrow_mut_data()?;
    // 设置池ID在偏移量0x930处
    let offset = 0x930;
    if pool_data_bytes.len() >= offset + 4 {
        pool_data_bytes[offset..offset + 4].copy_from_slice(&pool_id.to_le_bytes());
    }

    // 初始化池数据头部
    let signature_bytes = [0xc6, 0x44, 0x0c, 0xf5, 0xfd, 0xda, 0x26, 0x3f];
    if pool_data_bytes.len() >= 8 {
        pool_data_bytes[0..8].copy_from_slice(&signature_bytes);
    }

    // 初始化模板数据
    // 复制从偏移量8到960的952字节数据
    // 从汇编中可以看出，这是从地址0x100019250开始的952字节数据
    if pool_data_bytes.len() >= 960 {
        // 示例：复制模板数据
        // 这里需要实际的模板数据
        // 从原始sBPF代码看，是复制了预设的模板数据
        // pool_data_bytes[8..960].copy_from_slice(...);
    }

    // 初始化1800到2072的空间
    // 先清零512字节，然后复制272字节模板数据
    if pool_data_bytes.len() >= 2072 {
        // 清零512字节
        for i in 1800..2312 {
            if i < pool_data_bytes.len() {
                pool_data_bytes[i] = 0;
            }
        }

        // 复制模板数据
        // 从汇编看，是从地址0x100019608开始的272字节数据
        // pool_data_bytes[1800..2072].copy_from_slice(...);
    }

    // 设置一些额外的字段
    if pool_data_bytes.len() >= 0x928 + 8 {
        // 设置字段值17在偏移量0x928和0x918
        pool_data_bytes[0x928..0x928 + 8].copy_from_slice(&17u64.to_le_bytes());
        pool_data_bytes[0x918..0x918 + 8].copy_from_slice(&17u64.to_le_bytes());
        
        // 设置偏移量0x910和0x908的值
        // 从汇编中看，这些是一些内存地址值
        // 因为这些是特定的内存地址，在Anchor中需要替换成适当的值
        pool_data_bytes[0x910..0x910 + 8].copy_from_slice(&0u64.to_le_bytes());
        pool_data_bytes[0x908..0x908 + 8].copy_from_slice(&0u64.to_le_bytes());
        
        // 设置偏移量0x920的值为0
        pool_data_bytes[0x920..0x920 + 8].copy_from_slice(&0u64.to_le_bytes());
    }

    Ok(())
}
