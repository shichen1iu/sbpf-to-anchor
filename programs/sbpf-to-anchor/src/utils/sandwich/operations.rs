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
/// 此函数实现了sBPF汇编中的sandwich_tracker_register函数
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
