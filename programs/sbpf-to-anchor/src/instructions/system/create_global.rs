use anchor_lang::prelude::*;

// 全局状态账户结构
#[account]
pub struct GlobalState {
    // 魔数 0x8001c27439650c5c
    pub magic: u64,
    // 从输入数据复制的24字节(3个u64)
    pub data_1: u64,
    pub data_2: u64,
    pub data_3: u64,
    // 从输入数据复制的160字节
    pub remaining_data: [u8; 160],
}

// 创建全局状态账户的指令参数
#[derive(Accounts)]
pub struct CreateGlobal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // 要创建的全局状态账户
    #[account(
        init,
        space = 8 + 320, // 8字节discriminator + 320字节数据
        payer = payer,
        seeds = [b"global", b"state"],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    pub system_program: Program<'info, System>,
}

// 创建全局状态账户的指令
pub fn create_global(ctx: Context<CreateGlobal>, input_data: Vec<u8>) -> Result<()> {
    // 检查输入数据长度
    require!(input_data.len() >= 184, ErrorCode::InvalidDataLength);

    let global_state = &mut ctx.accounts.global_state;

    // 设置魔数
    global_state.magic = 0x8001c27439650c5c;

    // 复制前24字节数据到data_1/2/3
    global_state.data_1 = u64::from_le_bytes(input_data[0..8].try_into().unwrap());
    global_state.data_2 = u64::from_le_bytes(input_data[8..16].try_into().unwrap());
    global_state.data_3 = u64::from_le_bytes(input_data[16..24].try_into().unwrap());

    // 复制剩余160字节数据
    global_state
        .remaining_data
        .copy_from_slice(&input_data[24..184]);

    Ok(())
}
