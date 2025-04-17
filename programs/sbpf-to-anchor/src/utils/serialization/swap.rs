use anchor_lang::prelude::*;

/// 密钥类型枚举
/// 根据汇编代码中的返回值定义
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyType {
    /// 对应汇编中返回值 0
    Type0,
    /// 对应汇编中返回值 1
    Type1,
    /// 对应汇编中返回值 3 (默认/未知类型)
    Unknown,
}

/// 反序列化交换数据
/// 该函数根据输入的数据反序列化交换信息
///
/// # 参数说明
/// * `data` - 输入数据 (对应汇编中的 r1)
/// * `output` - 输出缓冲区 (对应汇编中的 r2)
/// * `swap_direction` - 交换方向 (对应汇编中的 r3)
/// * `swap_data` - 交换数据 (对应汇编中的 r4)
pub fn deserialize_swap(
    data: &[u8],
    output: &mut [u8],
    swap_direction: bool,
    swap_data: &[u8],
) -> Result<bool> {
    // 保存寄存器状态 (stxdw [r10-0x18], r4; mov64 r7, r2; stxdw [r10-0x8], r1)
    let mut success = false;

    // 读取第一个8字节 (ldxdw r2, [r9+0x0]; ldxdw r0, [r2+0x0])
    let first_value = u64::from_le_bytes(data[0..8].try_into().unwrap());

    // 检查第一种类型 (lddw r2, 0xcf5a6693f6e05601; jeq r0, r2)
    if first_value == 0xcf5a6693f6e05601 {
        // 设置类型标识 (mov64 r2, 12; stxb [r1+0x0], r2)
        output[0] = 12;

        // 解析Pump流动性数据
        if !pump_fun_parse_liquidity(data, &mut output[168..], swap_data)? {
            return Ok(false);
        }

        // 设置成功标志
        success = true;

        // 复制内存数据 (memcpy调用序列)
        // ... (此处省略大量memcpy操作，实际实现需要完整复制)
    }
    // 检查第二种类型 (lddw r5, 0x5259294f8b5a2aa9; jeq r0, r5)
    else if first_value == 0x5259294f8b5a2aa9 {
        // 设置类型标识 (mov64 r2, 13; stxb [r1+0x0], r2)
        output[0] = 13;

        // 解析Raydium CP流动性数据
        if !raydium_cp_parse_liquidity(data, &mut output[112..], &mut output[168..])? {
            return Ok(false);
        }

        // 设置成功标志
        success = true;

        // 复制内存数据 (memcpy调用序列)
        // ... (此处省略大量memcpy操作，实际实现需要完整复制)
    }
    // 检查第三种类型 (lddw r5, 0x3fc30236c449d94b; jne r0, r5)
    else if first_value == 0x3fc30236c449d94b {
        // 设置类型标识 (mov64 r2, 17; stxb [r1+0x0], r2)
        output[0] = 17;

        // 解析Raydium V4流动性数据
        if !raydium_v4_parse_liquidity(data, &mut output[112..], &mut output[168..])? {
            return Ok(false);
        }

        // 设置成功标志
        success = true;

        // 复制内存数据 (memcpy调用序列)
        // ... (此处省略大量memcpy操作，实际实现需要完整复制)
    }

    Ok(success)
}

/// 获取密钥类型（优化版本）
/// 该函数是get_key_type的优化实现，减少了不必要的比较和分支
///
/// # 参数说明
/// * `key_data` - 32字节的密钥数据 (对应汇编中的 r1)
pub fn get_key_type_optimised(key_data: &[u8]) -> Result<KeyType> {
    // 确保输入数据长度正确
    if key_data.len() < 32 {
        return Ok(KeyType::Unknown);
    }

    // 读取第一个8字节 (ldxdw r2, [r1+0x0])
    let first_value = u64::from_le_bytes(key_data[0..8].try_into().unwrap());

    // 检查第一种类型 (lddw r3, 0xcf5a6693f6e05601; jeq r2, r3, lbb_3959)
    if first_value == 0xcf5a6693f6e05601 {
        // 检查第二个8字节 (ldxdw r2, [r1+0x8]; lddw r3, 0xaa5b17bf6815db44)
        let second_value = u64::from_le_bytes(key_data[8..16].try_into().unwrap());
        if second_value != 0xaa5b17bf6815db44 {
            return Ok(KeyType::Unknown);
        }

        // 检查第三个8字节 (ldxdw r2, [r1+0x10]; lddw r3, 0x3bffd2f597cb8951)
        let third_value = u64::from_le_bytes(key_data[16..24].try_into().unwrap());
        if third_value == 0x3bffd2f597cb8951 {
            return Ok(KeyType::Type0);
        }
        return Ok(KeyType::Unknown);
    }

    // 检查第二种类型 (lddw r3, 0x5259294f8b5a2aa9; jeq r2, r3, lbb_3945)
    if first_value == 0x5259294f8b5a2aa9 {
        // 检查第二个8字节 (ldxdw r2, [r1+0x8]; lddw r3, 0x955bfd93aa502584)
        let second_value = u64::from_le_bytes(key_data[8..16].try_into().unwrap());
        if second_value != 0x955bfd93aa502584 {
            return Ok(KeyType::Unknown);
        }

        // 检查第三个8字节 (ldxdw r2, [r1+0x10]; lddw r3, 0x930c92eba8e6acb5)
        let third_value = u64::from_le_bytes(key_data[16..24].try_into().unwrap());
        if third_value != 0x930c92eba8e6acb5 {
            return Ok(KeyType::Unknown);
        }

        // 设置返回值为Type1 (mov64 r0, 1)
        let result = KeyType::Type1;

        // 检查第四个8字节 (ldxdw r1, [r1+0x18]; lddw r2, 0x73ec200c69432e94)
        let fourth_value = u64::from_le_bytes(key_data[24..32].try_into().unwrap());
        if fourth_value == 0x73ec200c69432e94 {
            return Ok(result);
        }
        return Ok(KeyType::Unknown);
    }

    // 检查第三种类型 (lddw r3, 0x3fc30236c449d94b; jne r2, r3, lbb_3967)
    if first_value == 0x3fc30236c449d94b {
        // 检查第二个8字节 (ldxdw r2, [r1+0x8]; lddw r3, 0x4c52a316ed907720)
        let second_value = u64::from_le_bytes(key_data[8..16].try_into().unwrap());
        if second_value != 0x4c52a316ed907720 {
            return Ok(KeyType::Unknown);
        }

        // 检查第三个8字节 (ldxdw r2, [r1+0x10]; lddw r3, 0xa9a221f15c97b9a1)
        let third_value = u64::from_le_bytes(key_data[16..24].try_into().unwrap());
        if third_value != 0xa9a221f15c97b9a1 {
            return Ok(KeyType::Unknown);
        }

        // 设置返回值为Type0 (mov64 r0, 0)
        let result = KeyType::Type0;

        // 检查第四个8字节 (ldxdw r1, [r1+0x18]; lddw r2, 0xcd8ab6f87decff0c)
        let fourth_value = u64::from_le_bytes(key_data[24..32].try_into().unwrap());
        if fourth_value == 0xcd8ab6f87decff0c {
            return Ok(result);
        }
        return Ok(KeyType::Unknown);
    }

    // 如果都不匹配，返回未知类型 (mov64 r0, 3)
    Ok(KeyType::Unknown)
}

/// 获取密钥类型（原始版本）
/// 该函数根据输入的密钥数据判断其类型
///
/// # 参数说明
/// * `key_data` - 32字节的密钥数据 (对应汇编中的 r1)
pub fn get_key_type(key_data: &[u8]) -> Result<KeyType> {
    // 确保输入数据长度正确
    if key_data.len() < 32 {
        return Ok(KeyType::Unknown);
    }

    // 读取并比较第一个8字节 (ldxdw r2, [r1+0x0])
    let first_value = u64::from_le_bytes(key_data[0..8].try_into().unwrap());

    // 检查第一种类型 (lddw r3, 0xcf5a6693f6e05601; jeq r2, r3)
    if first_value == 0xcf5a6693f6e05601 {
        // 检查第二个8字节 (ldxdw r2, [r1+0x8]; lddw r3, 0xaa5b17bf6815db44)
        let second_value = u64::from_le_bytes(key_data[8..16].try_into().unwrap());
        if second_value != 0xaa5b17bf6815db44 {
            return Ok(KeyType::Unknown);
        }

        // 检查第三个8字节 (ldxdw r2, [r1+0x10]; lddw r3, 0x3bffd2f597cb8951)
        let third_value = u64::from_le_bytes(key_data[16..24].try_into().unwrap());
        if third_value == 0x3bffd2f597cb8951 {
            return Ok(KeyType::Type0);
        }
        return Ok(KeyType::Unknown);
    }

    // 检查第二种类型 (lddw r3, 0x5259294f8b5a2aa9; jeq r2, r3)
    if first_value == 0x5259294f8b5a2aa9 {
        // 检查第二个8字节 (ldxdw r2, [r1+0x8]; lddw r3, 0x955bfd93aa502584)
        let second_value = u64::from_le_bytes(key_data[8..16].try_into().unwrap());
        if second_value != 0x955bfd93aa502584 {
            return Ok(KeyType::Unknown);
        }

        // 检查第三个8字节 (ldxdw r2, [r1+0x10]; lddw r3, 0x930c92eba8e6acb5)
        let third_value = u64::from_le_bytes(key_data[16..24].try_into().unwrap());
        if third_value != 0x930c92eba8e6acb5 {
            return Ok(KeyType::Unknown);
        }

        // 检查第四个8字节 (ldxdw r1, [r1+0x18]; lddw r2, 0x73ec200c69432e94)
        let fourth_value = u64::from_le_bytes(key_data[24..32].try_into().unwrap());
        if fourth_value == 0x73ec200c69432e94 {
            return Ok(KeyType::Type1);
        }
        return Ok(KeyType::Unknown);
    }

    // 检查第三种类型 (lddw r3, 0x3fc30236c449d94b; jne r2, r3)
    if first_value == 0x3fc30236c449d94b {
        // 检查第二个8字节 (ldxdw r2, [r1+0x8]; lddw r3, 0x4c52a316ed907720)
        let second_value = u64::from_le_bytes(key_data[8..16].try_into().unwrap());
        if second_value != 0x4c52a316ed907720 {
            return Ok(KeyType::Unknown);
        }

        // 检查第三个8字节 (ldxdw r2, [r1+0x10]; lddw r3, 0xa9a221f15c97b9a1)
        let third_value = u64::from_le_bytes(key_data[16..24].try_into().unwrap());
        if third_value != 0xa9a221f15c97b9a1 {
            return Ok(KeyType::Unknown);
        }

        // 检查第四个8字节 (ldxdw r1, [r1+0x18]; lddw r2, 0xcd8ab6f87decff0c)
        let fourth_value = u64::from_le_bytes(key_data[24..32].try_into().unwrap());
        if fourth_value == 0xcd8ab6f87decff0c {
            return Ok(KeyType::Type0);
        }
        return Ok(KeyType::Unknown);
    }

    // 如果都不匹配，返回未知类型 (mov64 r0, 3)
    Ok(KeyType::Unknown)
}
