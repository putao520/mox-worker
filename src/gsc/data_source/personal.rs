use crate::gsc::data_source::source_service::AsStringEnum;

/**
有效客户:
    可以正常使用的客户信息
name: valid_personal

使用客户:
    正在使用的客户信息
name: use_personal

成功客户:
    预约成功的客户信息
name: success_personal

异常客户:
    预约失败的客户信息
name: exception_personal
 */

pub struct PersonSource<'a> {
    pub key_name: &'a str,
}

pub enum PersonalType {
    Valid,
    // Using,
    Success,
    Exception,
}

impl AsStringEnum for PersonalType {
    fn as_str(&self) -> &'static str {
        match *self {
            PersonalType::Valid => "valid_personal",        // list
            // PersonalType::Using => "using_personal",        // hash
            PersonalType::Success => "success_personal",    // list
            PersonalType::Exception => "exception_personal",// list
        }
    }
}