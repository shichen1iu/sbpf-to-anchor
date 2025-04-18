use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};

/// 交换类型的常量定义，与汇编代码中的哈希值对应
pub mod swap_type {
    /// 直接交换类型ID
    pub const DIRECT_SWAP_ID: u64 = 0xcf5a6693f6e05601;
    /// 路由交换类型ID
    pub const ROUTE_SWAP_ID: u64 = 0x5259294f8b5a2aa9;
    /// 池交换类型ID
    pub const POOL_SWAP_ID: u64 = 0x3fc30236c449d94b;

    /// 池交换类型二级ID
    pub const POOL_SWAP_ID2: u64 = 0x4c52a316ed907720;
    pub const POOL_SWAP_ID3: u64 = 0xa9a221f15c97b9a1;
    pub const POOL_SWAP_ID4: u64 = 0xcd8ab6f87decff0c;

    /// 路由交换类型二级ID
    pub const ROUTE_SWAP_ID2: u64 = 0x955bfd93aa502584;
    pub const ROUTE_SWAP_ID3: u64 = 0x930c92eba8e6acb5;
    pub const ROUTE_SWAP_ID4: u64 = 0x73ec200c69432e94;

    /// 直接交换类型二级ID
    pub const DIRECT_SWAP_ID2: u64 = 0xaa5b17bf6815db44;
    pub const DIRECT_SWAP_ID3: u64 = 0x3bffd2f597cb8951;
    pub const DIRECT_SWAP_ID4: u64 = 0xb0186dfdb62b5d65;

    /// 交换数据ID
    pub const ROUTE_SWAP_DATA_ID: u64 = 0xde331ec4da5abe8f;
    pub const DIRECT_SWAP_DATA_ID1: u64 = 0xad837f01a485e633;
    pub const DIRECT_SWAP_DATA_ID2: u64 = 0xeaebda01123d0666;
}

/// 执行交换指令的账户参数
#[derive(Accounts)]
pub struct ExecuteSwap<'info> {
    /// Token程序ID
    /// @account 用于执行token操作的程序
    pub token_program: AccountInfo<'info>,

    /// 交换程序ID
    /// @account 代表要调用的交换程序
    pub swap_program: AccountInfo<'info>,

    /// 当前程序的PDA账户，用于签名
    /// @account 程序派生地址用于签名交易
    #[account(
        seeds = [b"swap"],
        bump
    )]
    pub program_pda: AccountInfo<'info>,
}

/// 执行交换
///
/// 根据指令数据中的交换类型执行不同的交换操作:
/// 1. 如果是直接交换(DIRECT_SWAP_ID)，执行直接交换逻辑
/// 2. 如果是路由交换(ROUTE_SWAP_ID)，执行路由交换逻辑
/// 3. 如果是池交换(POOL_SWAP_ID)，执行池交换逻辑
///
/// 所有交换最终通过CPI调用token程序或其他程序完成实际的交换操作
pub fn execute_swap(ctx: Context<ExecuteSwap>, swap_data: Vec<u8>, bump: u8) -> Result<()> {
    // 获取账户信息
    let accounts = ctx.accounts;
    let remaining_accounts = ctx.remaining_accounts;

    // 从交换数据中解析出交换类型
    let swap_type = if swap_data.len() >= 8 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&swap_data[0..8]);
        u64::from_le_bytes(bytes)
    } else {
        return Err(ProgramError::InvalidInstructionData.into());
    };

    // 构建调用参数
    let mut invoke_accounts = Vec::with_capacity(remaining_accounts.len() + 2);
    invoke_accounts.push(accounts.token_program.to_account_info());
    invoke_accounts.push(accounts.swap_program.to_account_info());

    // 添加剩余账户
    for acc in remaining_accounts.iter() {
        invoke_accounts.push(acc.to_account_info());
    }

    // PDA签名种子
    let seeds = &[b"swap".as_ref(), &[bump]];

    // 根据交换类型执行不同的交换逻辑
    if swap_type == swap_type::DIRECT_SWAP_ID {
        // 直接交换逻辑
        // 验证二级标识符
        let secondary_id = if swap_data.len() >= 16 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&swap_data[8..16]);
            u64::from_le_bytes(bytes)
        } else {
            return Err(ProgramError::InvalidInstructionData.into());
        };

        if secondary_id != swap_type::DIRECT_SWAP_ID2
            || get_id_at_offset(&swap_data, 16) != swap_type::DIRECT_SWAP_ID3
            || get_id_at_offset(&swap_data, 24) != swap_type::DIRECT_SWAP_ID4
        {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        // 根据标志位选择数据ID
        let data_id = if swap_data.len() >= 32 && swap_data[31] == 0 {
            swap_type::DIRECT_SWAP_DATA_ID2
        } else {
            swap_type::DIRECT_SWAP_DATA_ID1
        };

        // 构建交换指令
        let data_len = 24; // 对应汇编中的 mov64 r1, 24
        let swap_ix =
            build_swap_instruction(data_id, data_len, &swap_data[32..], &accounts.swap_program);

        // 执行交叉程序调用
        invoke_signed(&swap_ix, &invoke_accounts, &[&seeds[..]])?;
    } else if swap_type == swap_type::ROUTE_SWAP_ID {
        // 路由交换逻辑
        // 验证二级标识符
        if get_id_at_offset(&swap_data, 8) != swap_type::ROUTE_SWAP_ID2
            || get_id_at_offset(&swap_data, 16) != swap_type::ROUTE_SWAP_ID3
            || get_id_at_offset(&swap_data, 24) != swap_type::ROUTE_SWAP_ID4
        {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        // 构建交换指令
        let data_id = swap_type::ROUTE_SWAP_DATA_ID;
        let data_len = 24; // 对应汇编中的 mov64 r1, 24
        let swap_ix =
            build_swap_instruction(data_id, data_len, &swap_data[32..], &accounts.swap_program);

        // 执行交叉程序调用
        invoke_signed(&swap_ix, &invoke_accounts, &[&seeds[..]])?;
    } else if swap_type == swap_type::POOL_SWAP_ID {
        // 池交换逻辑
        // 验证二级标识符
        if get_id_at_offset(&swap_data, 8) != swap_type::POOL_SWAP_ID2
            || get_id_at_offset(&swap_data, 16) != swap_type::POOL_SWAP_ID3
            || get_id_at_offset(&swap_data, 24) != swap_type::POOL_SWAP_ID4
        {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        // 构建交换指令
        let data_len = 17; // 对应汇编中的 mov64 r1, 17
                           // 这里的9对应汇编中的 mov64 r1, 9; stxb [r10-0x50], r1
        let swap_ix = build_swap_instruction(9, data_len, &swap_data[32..], &accounts.swap_program);

        // 执行交叉程序调用
        invoke_signed(&swap_ix, &invoke_accounts, &[&seeds[..]])?;
    } else {
        return Err(ProgramError::InvalidInstructionData.into());
    }

    Ok(())
}

/// 从指定偏移量获取ID
///
/// 从数据数组的指定偏移量处读取8个字节并转换为u64
fn get_id_at_offset(data: &[u8], offset: usize) -> u64 {
    if data.len() >= offset + 8 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[offset..offset + 8]);
        u64::from_le_bytes(bytes)
    } else {
        0
    }
}

/// 构建交换指令
///
/// 根据提供的数据ID、数据长度和指令数据构建交换指令
fn build_swap_instruction(
    data_id: u64,
    data_len: usize,
    instruction_data: &[u8],
    program_id: &AccountInfo,
) -> Instruction {
    // 创建包含数据ID的指令数据
    let mut data = Vec::with_capacity(8 + data_len);
    data.extend_from_slice(&data_id.to_le_bytes());
    if !instruction_data.is_empty() {
        data.extend_from_slice(
            &instruction_data[0..std::cmp::min(data_len, instruction_data.len())],
        );
    }

    // 填充剩余空间
    while data.len() < 8 + data_len {
        data.push(0);
    }

    // 构造指令
    Instruction {
        program_id: *program_id.key,
        accounts: vec![], // 在调用时会填充
        data,
    }
}
