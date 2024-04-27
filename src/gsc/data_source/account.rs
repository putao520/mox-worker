// 提供账号数据服务


use crate::gsc::data_source::source_service::AsStringEnum;

/**
有效账号:
    可以正常使用的账号
name: valid_account

使用账号:
    正在使用的账号
name: use_account

过载账号:
    超过使用限制的账号(已经预约满的账号, 或者冷却的)
name: overload_account

封号账号:
    被封禁的账号
name: ban_account
 */

pub enum AccountType {
    Valid,
    Using,
    // Overload,
    Ban,
}

impl AsStringEnum for  AccountType {
    fn as_str(&self) -> &'static str {
        match *self {
            AccountType::Valid => "valid_account",      // list
            AccountType::Using => "using_account",      // hash
            // AccountType::Overload => "overload_account",
            AccountType::Ban => "ban_account",          // list
        }
    }
}