use super::*;
use crate::error::*;
use crate::states::*;
use crate::utils::*;
use anchor_lang::prelude::*;

/// 全局状态更新账户结构
/// 用于管理全局配置和费用设置的更新操作
#[derive(Accounts)]
pub struct UpdateGlobal<'info> {
    /// 管理员账户
    /// 必须是签名者且有权限执行更新操作
    #[account(mut)]
    pub admin: Signer<'info>,

    /// 全局状态账户
    /// 存储系统的全局配置和费用设置
    #[account(mut)]
    pub global_state: Account<'info, GlobalState>,

    /// 更新数据账户
    /// 包含要更新的新配置和费用数据
    pub global_update_data: Account<'info, GlobalUpdateData>,
}

/// 全局状态更新函数
///
/// # 功能特点
/// * 支持费用参数更新
/// * 支持配置数据更新
/// * 权限控制
/// * 参数验证
///
/// # 更新内容
/// * 费用设置：费率分子/分母、小费费用
/// * 配置数据：最大160字节的配置信息
///
/// # 安全考虑
/// * 管理员权限验证
/// * 参数有效性检查
/// * 数据大小限制
pub fn update_global(ctx: Context<UpdateGlobal>) -> Result<()> {
    let accounts = &ctx.accounts;

    // 验证管理员权限
    // 确保只有授权的管理员可以执行更新
    require!(
        accounts.admin.key() == accounts.global_state.admin,
        ErrorCode::InvalidAuthority
    );

    // 获取更新标志
    // 确定需要更新的内容类型
    let update_fee_flag = accounts.global_update_data.update_fee_flag;
    let update_config_flag = accounts.global_update_data.update_config_flag;

    // 处理费用设置更新
    if update_fee_flag {
        msg!("Updating global fee settings");

        // 获取新的费用参数
        let fee_numerator = accounts.global_update_data.fee_numerator;
        let fee_denominator = accounts.global_update_data.fee_denominator;
        let tipper_fee = accounts.global_update_data.tipper_fee;

        // 验证费用参数的有效性
        // 确保分母不为0且分子不大于分母
        require!(fee_denominator > 0, ErrorCode::InvalidParameter);
        require!(
            fee_numerator <= fee_denominator,
            ErrorCode::InvalidParameter
        );

        // 更新全局状态中的费用设置
        let global_state = &mut ctx.accounts.global_state;
        global_state.fee_numerator = fee_numerator;
        global_state.fee_denominator = fee_denominator;
        global_state.tipper_fee = tipper_fee;

        // 记录更新后的费用设置
        msg!(
            "Updated fees - Numerator: {}, Denominator: {}, Tipper Fee: {}",
            fee_numerator,
            fee_denominator,
            tipper_fee
        );
    }

    // 处理配置数据更新
    if update_config_flag {
        msg!("Updating global configuration");

        // 验证配置数据大小
        // 确保不超过160字节的限制
        let config_data = &accounts.global_update_data.config_data;
        require!(config_data.len() <= 160, ErrorCode::InvalidParameter);

        // 更新配置数据
        // 将新的配置数据复制到全局状态中
        let global_state = &mut ctx.accounts.global_state;
        global_state.config_data.copy_from_slice(config_data);

        msg!("Updated configuration data");
    }

    // 记录更新完成
    msg!("Global state updated successfully");
    Ok(())
}
