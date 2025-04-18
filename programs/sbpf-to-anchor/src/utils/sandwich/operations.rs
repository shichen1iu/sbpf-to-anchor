use anchor_lang::prelude::*;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;

/// 创建三明治账户
/// 此函数实现了sBPF汇编中的create_sandwich_函数
/// 通过系统程序调用创建一个新账户并初始化它的数据
pub fn create_sandwich(
    payer: &AccountInfo,
    sandwich_account: &AccountInfo,
    system_program: &AccountInfo,
    rent: &AccountInfo,
) -> Result<()> {
    // 创建账户 - 对应汇编中的 call create_account_
    // 设置账户大小为160字节 (来自汇编中的 mov64 r4, 160)
    let space = 160;
    let lamports = Rent::get()?.minimum_balance(space as usize);

    invoke(
        &system_instruction::create_account(
            payer.key,
            sandwich_account.key,
            lamports,
            space,
            &crate::ID,
        ),
        &[
            payer.clone(),
            sandwich_account.clone(),
            system_program.clone(),
        ],
    )?;

    // 初始化账户数据 - 对应汇编中设置特定值的部分
    // 初始化数据为0x8f5c570f55dd7921 (对应汇编中的lddw r2, 0x8f5c570f55dd7921)
    let data = &mut sandwich_account.try_borrow_mut_data()?;
    let value: u64 = 0x8f5c570f55dd7921;
    data[0..8].copy_from_slice(&value.to_le_bytes());

    Ok(())
}

/// 关闭三明治账户
/// 此函数实现了sBPF汇编中的close_sandwich_函数
/// 记录账户信息并关闭账户
pub fn close_sandwich(
    sandwich_account: &AccountInfo,
    destination_account: &AccountInfo,
) -> Result<()> {
    // 记录账户数据 - 对应汇编中的syscall sol_log_data
    // 原汇编中获取账户的数据指针和大小并记录
    solana_program::log::sol_log_data(&[&sandwich_account.data.borrow()[..]]);

    // 关闭账户 - 对应汇编中的call close_account_
    // 将账户余额转移到目标账户并关闭
    let dest_starting_lamports = destination_account.lamports();
    **destination_account.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(sandwich_account.lamports())
        .ok_or(ProgramError::ArithmeticOverflow)?;
    **sandwich_account.lamports.borrow_mut() = 0;

    // 清空账户数据
    let mut data = sandwich_account.try_borrow_mut_data()?;
    for byte in data.iter_mut() {
        *byte = 0;
    }

    Ok(())
}

/// 计算租金
/// 此函数实现了sBPF汇编中的calculate_rent函数
fn calculate_rent(space: u64) -> u64 {
    // 简单实现，实际应该调用Rent::get()?.minimum_balance(space as usize)
    Rent::get().unwrap().minimum_balance(space as usize)
}

/// 给小费账户充值
/// 此函数实现了sBPF汇编中的topup_tipper_函数
fn topup_tipper(
    amount_a: u64,
    amount_b: u64,
    account_1: &AccountInfo,
    account_2: &AccountInfo,
    accounts_array: &[AccountInfo],
) -> Result<u8> {
    // 向小费账户转账
    let tipper_account = &accounts_array[3]; // 根据汇编代码偏移推断

    // 如果金额不足，返回错误
    if amount_a == 0 && amount_b == 0 {
        return Ok(1); // 表示没有足够的余额
    }

    // 转移资金到小费账户
    let tipper_starting_lamports = tipper_account.lamports();
    **tipper_account.lamports.borrow_mut() = tipper_starting_lamports
        .checked_add(
            amount_a
                .checked_add(amount_b)
                .ok_or(ProgramError::ArithmeticOverflow)?,
        )
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(0) // 成功
}

/// 关闭多个三明治账户并给小费账户充值
/// 此函数实现了sBPF汇编中的close_sandwiches_and_topup_tipper函数
pub fn close_sandwiches_and_topup_tipper(accounts: &[AccountInfo]) -> Result<()> {
    // 初始化变量 - 对应汇编中的 mov64 r8, 0 和 mov64 r9, 0
    let mut amount_b: u64 = 0;
    let mut amount_a: u64 = 0;

    // 取出账户信息的数组 - 对应汇编中的 ldxdw r7, [r1+0x0]
    let accounts_data = &accounts[0];

    // 记录日志 - 对应汇编中的 syscall sol_log_
    msg!("Processing sandwich accounts");

    // 遍历账户 - 对应汇编的循环逻辑
    let accounts_count = if accounts.len() > 10 {
        10
    } else {
        accounts.len()
    };

    // 计算总金额 - 对应汇编中从下标9开始遍历的循环
    for i in 9..accounts_count {
        let account = &accounts[i];

        // 检查账户是否有效 - 对应汇编中的 jeq r4, 0, lbb_8893
        if account.key == &Pubkey::default() {
            continue;
        }

        // 读取金额数据 - 对应汇编中的 ldxdw 指令序列
        if let Ok(data) = account.try_borrow_data() {
            if data.len() >= 0x9A + 1 {
                let value_a = u64::from_le_bytes(data[0x88..0x90].try_into().unwrap_or_default());
                let value_b = u64::from_le_bytes(data[0x90..0x98].try_into().unwrap_or_default());
                let flag = data[0x9A];

                // 根据标志位分配金额 - 对应汇编中的条件跳转逻辑
                if flag == 0 {
                    amount_b = amount_b.saturating_add(value_a.saturating_add(value_b));
                } else {
                    amount_a = amount_a.saturating_add(value_a.saturating_add(value_b));
                }
            }
        }
    }

    // 记录计算的金额 - 对应汇编中的 syscall sol_log_64_
    solana_program::log::sol_log_64(amount_a, amount_b, 0, 0, 0);

    // 调用小费充值函数 - 对应汇编中的 call topup_tipper_
    let account_1 = if accounts.len() > 7 {
        &accounts[7]
    } else {
        &accounts[0]
    };
    let account_2 = if accounts.len() > 4 {
        &accounts[4]
    } else {
        &accounts[0]
    };

    let result = topup_tipper(amount_a, amount_b, account_1, account_2, accounts)?;

    // 如果充值失败，直接返回 - 对应汇编中的 jeq r0, 0, lbb_8941
    if result != 0 {
        return Ok(());
    }

    // 计算租金 - 对应汇编中的 call calculate_rent
    let rent_amount = calculate_rent(165);

    // 转移租金 - 对应汇编中的减法和加法操作
    if accounts.len() > 2 {
        let fee_account = &accounts[2];
        let fee_lamports = fee_account.lamports();
        **fee_account.lamports.borrow_mut() = fee_lamports.saturating_sub(rent_amount);

        if accounts.len() > 1 {
            let destination_account = &accounts[1];
            let dest_lamports = destination_account.lamports();
            **destination_account.lamports.borrow_mut() = dest_lamports.saturating_add(rent_amount);
        }
    }

    // 关闭三明治账户 - 对应汇编中的第二个循环
    for i in 9..accounts_count {
        let account = &accounts[i];

        // 检查账户是否有效
        if let Ok(data) = account.try_borrow_data() {
            if data.len() >= 0x210 + 8 {
                let ptr_value =
                    u64::from_le_bytes(data[0x208..0x210].try_into().unwrap_or_default());

                if ptr_value != 0 {
                    // 获取账户数据并记录 - 对应汇编中的 syscall sol_log_data
                    let data_value = if data.len() >= 0x218 {
                        u64::from_le_bytes(data[0x210..0x218].try_into().unwrap_or_default())
                    } else {
                        0
                    };

                    // 记录账户数据
                    let log_data = [ptr_value.to_le_bytes(), data_value.to_le_bytes()].concat();
                    solana_program::log::sol_log_data(&[&log_data]);

                    // 关闭账户 - 对应汇编中的 call close_account_
                    let sandwich_account = if i + 9 < accounts.len() {
                        &accounts[i + 9]
                    } else {
                        continue;
                    };

                    let destination_account = if accounts.len() > 3 {
                        &accounts[3]
                    } else {
                        continue;
                    };

                    // 关闭账户操作
                    let dest_starting_lamports = destination_account.lamports();
                    **destination_account.lamports.borrow_mut() = dest_starting_lamports
                        .checked_add(sandwich_account.lamports())
                        .ok_or(ProgramError::ArithmeticOverflow)?;
                    **sandwich_account.lamports.borrow_mut() = 0;

                    // 清空账户数据
                    if let Ok(mut account_data) = sandwich_account.try_borrow_mut_data() {
                        for byte in account_data.iter_mut() {
                            *byte = 0;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// 注册三明治跟踪器
/// 用于记录时间戳和更新计数器
pub fn sandwich_tracker_register(tracker_data: &AccountInfo, timestamp: u64) -> Result<()> {
    // 初始化变量 - 对应汇编中的mov64 r3, 0
    let mut slot_addr: u64 = 0;

    // 从账户数据中读取基准时间戳 - 对应汇编中的ldxdw r4, [r1+0x10]
    let base_timestamp = match tracker_data.try_borrow_data() {
        Ok(data) => {
            if data.len() < 0x18 {
                return Ok(());
            }
            u64::from_le_bytes(data[0x10..0x18].try_into().unwrap_or_default())
        }
        Err(_) => return Ok(()),
    };

    // 计算最大时间戳 - 对应汇编中的add64 r5, 432000
    let max_timestamp = base_timestamp.saturating_add(432000);

    // 检查时间戳是否在有效范围内 - 对应汇编中的一系列比较和跳转
    if timestamp < 4 {
        return Ok(());
    }

    let adj_timestamp = timestamp.saturating_sub(4);

    // 第一个检查点 - 对应汇编中的条件判断代码块
    if base_timestamp > adj_timestamp || adj_timestamp >= max_timestamp {
        return Ok(());
    }

    // 计算第一个时间槽偏移 - 对应汇编中的减法、移位和掩码操作
    let time_diff = adj_timestamp.saturating_sub(base_timestamp);
    let slot_offset = (time_diff >> 1) & 0x7ffffffffffffffe;

    // 计算第一个槽地址和检查有效性 - 对应汇编中的地址计算和检查
    let mut data = tracker_data.try_borrow_mut_data()?;
    let slot_index_addr = 65560 + slot_offset as usize;

    if slot_index_addr + 2 > data.len() {
        return Ok(());
    }

    let slot_index = u16::from_le_bytes(
        data[slot_index_addr..slot_index_addr + 2]
            .try_into()
            .unwrap_or_default(),
    );

    // 检查槽索引是否在有效范围内 - 对应汇编中的比较操作
    if slot_index > 2047 {
        return Ok(());
    }

    // 第二个检查点 - 对应汇编中的第二个条件判断代码块
    if base_timestamp > timestamp || timestamp >= max_timestamp {
        return Ok(());
    }

    // 计算第二个时间槽偏移
    let time_diff2 = timestamp.saturating_sub(base_timestamp);
    let slot_offset2 = (time_diff2 >> 1) & 0x7ffffffffffffffe;

    // 计算第二个槽地址和检查有效性
    let slot_index_addr2 = 65560 + slot_offset2 as usize;

    if slot_index_addr2 + 2 > data.len() {
        return Ok(());
    }

    let slot_index2 = u16::from_le_bytes(
        data[slot_index_addr2..slot_index_addr2 + 2]
            .try_into()
            .unwrap_or_default(),
    );

    // 检查第二个槽索引是否在有效范围内
    if slot_index2 > 2047 {
        return Ok(());
    }

    // 更新第一个计数器 - 对应汇编中的最后部分
    let counter_base_addr = 281560;
    let counter_offset1 = (slot_index as u64 * 2) as usize;

    if counter_base_addr + counter_offset1 + 2 <= data.len() {
        let counter1 = u16::from_le_bytes(
            data[counter_base_addr + counter_offset1..counter_base_addr + counter_offset1 + 2]
                .try_into()
                .unwrap_or_default(),
        );
        let new_counter1 = counter1.saturating_add(1);
        data[counter_base_addr + counter_offset1..counter_base_addr + counter_offset1 + 2]
            .copy_from_slice(&new_counter1.to_le_bytes());
    }

    // 更新第二个计数器
    let counter_offset2 = (slot_index2 as u64 * 2) as usize;

    if counter_base_addr + counter_offset2 + 2 <= data.len() {
        let counter2 = u16::from_le_bytes(
            data[counter_base_addr + counter_offset2..counter_base_addr + counter_offset2 + 2]
                .try_into()
                .unwrap_or_default(),
        );
        let new_counter2 = counter2.saturating_add(1);
        data[counter_base_addr + counter_offset2..counter_base_addr + counter_offset2 + 2]
            .copy_from_slice(&new_counter2.to_le_bytes());
    }

    Ok(())
}

/// 更新代币数据前置操作
/// 此函数实现了sBPF汇编中的token_data_update_frontrun函数
/// 用于更新代币数据账户的信息
pub fn token_data_update_frontrun(
    token_data: &AccountInfo,     // r7
    source_account: &AccountInfo, // r9
    ptr_account: &AccountInfo,    // r6
    opt_account: &AccountInfo,    // r8
) -> Result<()> {
    // 读取标志位 - 对应汇编中的 ldxb r1, [r7+0x8]
    let mut data = token_data.try_borrow_mut_data()?;
    let flag = data[0x8];

    // 如果标志位为0，则初始化账户 - 对应汇编中的 jne r1, 0, lbb_9207
    if flag == 0 {
        // 设置标志位为1 - 对应汇编中的 mov64 r1, 1 和 stxb [r7+0x8], r1
        data[0x8] = 1;

        // 内存拷贝常量数据 - 对应汇编中的 call memcpy
        // 源地址为0x10001a328，目标地址为token_data+296，大小为96字节
        let constant_data = [0u8; 96]; // 实际应该是某个特定的常量数据
        if data.len() >= 296 + 96 {
            data[296..296 + 96].copy_from_slice(&constant_data);
        }

        // 从source_account复制数据 - 对应汇编中的ldxdw/stxdw序列
        if let Ok(source_data) = source_account.try_borrow_data() {
            if source_data.len() >= 0x20 && data.len() >= 0x30 {
                // 复制4个u64值，偏移分别为0x0, 0x8, 0x10, 0x18
                data[0x10..0x18].copy_from_slice(&source_data[0x0..0x8]);
                data[0x18..0x20].copy_from_slice(&source_data[0x8..0x10]);
                data[0x20..0x28].copy_from_slice(&source_data[0x10..0x18]);
                data[0x28..0x30].copy_from_slice(&source_data[0x18..0x20]);
            }
        }

        // 从ptr_account复制数据 - 对应汇编中的ldxdw/stxdw
        if let Ok(ptr_data) = ptr_account.try_borrow_data() {
            if ptr_data.len() >= 0x8 && data.len() >= 0x210 {
                data[0x208..0x210].copy_from_slice(&ptr_data[0x0..0x8]);
            }
        }
    }

    // 如果opt_account不为空，则从中复制数据 - 对应汇编中的jeq r8, 0, lbb_9216
    if opt_account.key != &Pubkey::default() {
        if let Ok(opt_data) = opt_account.try_borrow_data() {
            if opt_data.len() >= 0x20 && data.len() >= 0x50 {
                // 复制4个u64值，但顺序与原始字段相反
                data[0x48..0x50].copy_from_slice(&opt_data[0x18..0x20]);
                data[0x40..0x48].copy_from_slice(&opt_data[0x10..0x18]);
                data[0x38..0x40].copy_from_slice(&opt_data[0x8..0x10]);
                data[0x30..0x38].copy_from_slice(&opt_data[0x0..0x8]);
            }
        }
    }

    Ok(())
}

/// 更新代币数据后置操作
/// 此函数实现了sBPF汇编中的token_data_update_backrun函数
/// 用于更新代币数据账户的后置信息并处理报价
pub fn token_data_update_backrun(
    token_data: &AccountInfo,     // r6
    quote_account: &AccountInfo,  // r8
    source_account: &AccountInfo, // r7
) -> Result<()> {
    // 读取状态标志位 - 对应汇编中的 ldxb r2, [r7+0x99]
    let mut data = token_data.try_borrow_mut_data()?;

    if source_account.data_len() < 0x9A {
        return Ok(());
    }

    let source_data = source_account.try_borrow_data()?;
    let flag = source_data[0x99];

    // 初始化变量 - 对应汇编中的 mov64 r0, 0 等指令
    let mut r0: u64 = 0;
    let mut r9: u64 = 1;
    let mut r1: u64 = 1;

    // 条件逻辑 - 对应汇编中的 jne r2, 0, lbb_9228 等指令
    if flag != 0 {
        r1 = 0;
    }

    if flag == 0 {
        r9 = 0;
    }

    // 读取第二个状态标志 - 对应汇编中的 ldxb r2, [r7+0x9b]
    let flag2 = if source_data.len() > 0x9B {
        source_data[0x9B]
    } else {
        0
    };

    // 处理额外数据 - 对应汇编中的 jeq r2, 0, lbb_9233 和后续指令
    if flag2 != 0 {
        r9 = r1;
    }

    // 读取和存储数据 - 对应汇编中的 ldxdw/stxdw 操作
    let mut r4: u64 = 0;
    if flag2 != 0 && source_data.len() >= 0x1000 {
        // 这里简化处理，实际应该获取特定偏移的数据
        // 对应汇编中的 ldxdw r4, [r5-0x1000]
    }

    // 存储值到token_data - 对应汇编中的 stxdw [r6+0x50], r4
    if data.len() >= 0x58 {
        data[0x50..0x58].copy_from_slice(&r4.to_le_bytes());
    }

    // 获取额外数据 - 对应汇编中的 ldxdw r1, [r5-0xff8]
    let r1: u64 = 0; // 简化实现

    // 获取报价 - 对应汇编中的 call get_quote
    if r4 != 0 && data.len() >= 0x60 {
        let r3 = r9 & 1;
        // 这里应该调用get_quote函数，简化实现
        // r0 = get_quote(quote_account, r4, r3)?;
    }

    // 存储报价结果 - 对应汇编中的 stxdw [r6+0x58], r0
    if data.len() >= 0x60 {
        data[0x58..0x60].copy_from_slice(&r0.to_le_bytes());
    }

    // 获取第二个报价 - 对应汇编中的后续指令
    r9 = !r9 & 1;
    // 简化实现，应该调用get_quote
    // r0 = get_quote(quote_account, 1000000000, r9)?;

    // 存储第二个报价结果到特定位置 - 对应汇编中的 stxdw [r6+0x2b0], r0
    if data.len() >= 0x2B8 {
        data[0x2B0..0x2B8].copy_from_slice(&r0.to_le_bytes());

        // 更新最小值逻辑 - 对应汇编中的比较和条件存储
        if data.len() >= 0x2B0 {
            let r1 = u64::from_le_bytes(data[0x2A8..0x2B0].try_into().unwrap_or_default());
            if r0 <= r1.saturating_sub(1) {
                data[0x2A8..0x2B0].copy_from_slice(&r0.to_le_bytes());
            }
        }

        // 更新第二个最小值 - 对应汇编中的第二个比较和条件存储
        if data.len() >= 0x2A8 {
            let r1 = u64::from_le_bytes(data[0x2A0..0x2A8].try_into().unwrap_or_default());
            if r0 <= r1.saturating_sub(1) {
                data[0x2A0..0x2A8].copy_from_slice(&r0.to_le_bytes());
            }
        }
    }

    // 更新累计值 - 对应汇编中的加法和存储操作
    if source_data.len() >= 0x80 && data.len() >= 0x70 {
        // 读取源账户数据
        let r1 = u64::from_le_bytes(source_data[0x78..0x80].try_into().unwrap_or_default());
        // 读取目标账户当前数据
        let r2 = u64::from_le_bytes(data[0x60..0x68].try_into().unwrap_or_default());
        // 计算新值并存储
        let new_value = r2.saturating_add(r1);
        data[0x60..0x68].copy_from_slice(&new_value.to_le_bytes());

        // 读取第二个源账户数据
        let r1 = u64::from_le_bytes(source_data[0x70..0x78].try_into().unwrap_or_default());
        // 读取第二个目标账户当前数据
        let r2 = u64::from_le_bytes(data[0x68..0x70].try_into().unwrap_or_default());
        // 计算新值并存储
        let new_value = r2.saturating_add(r1);
        data[0x68..0x70].copy_from_slice(&new_value.to_le_bytes());
    }

    // 获取下一个目标 - 对应汇编中的 call pg_get_next_goal
    // 这部分需要一个专门的函数，简化实现
    // let mut r7 = r6 + 296;
    // let mut r0 = pg_get_next_goal(r7)?;

    // 最后的循环处理 - 对应汇编中的最后部分循环
    // 简化实现，实际应该处理目标值和更新

    Ok(())
}
