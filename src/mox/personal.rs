use std::cell::OnceCell;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Commands, Value};
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::gsc::concurrency::transaction::Transaction;

use crate::gsc::data_source::personal::PersonalType;
use crate::gsc::data_source::source_service::AsStringEnum;
use crate::gsc::mdl::redis::{get_redis_connect, get_sync_redis_connect};

// 客户信息
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone, Debug)]
pub struct Personal {
    pub first_name: String,                     // 名字
    pub name: String,                           // 名字
    pub last_name: String,                      // 姓
    pub gender: u32,                            // 性别 1:女 2:男 3:其他
    pub birth_date: String,                     // 生日 YYYY-MM-DD    // 需要补0
    pub log_message: String,                    // 日志信息
    pub marital_status: u32,                    // 婚否 1: 单身 2: 已婚 3: 同居
    pub phone: String,                          // 电话
    pub state_id: u32,        // 所在省或直辖市
    pub country_id: u32,      // 所在国家
    pub city_address: String, // 城市地址
    pub passport: Option<String>,               // 护照号
    pub nut: Option<String>,                    // NUT号
    pub appointment_start: i64,                 // 预约日期开始日期[时间戳,秒]    -> 原来是 YYYY-MM-DD,现在是 YYYY-MM-DD HH:MM:SS 的时间戳
    pub appointment_end: i64,                   // 预约日期结束日期[时间戳](结束当天还会尝试预约) -> 同上
    pub priority: u32,                          // 优先级 0, 1
    pub emergency_contact: EmergencyContact,    // 紧急联系人信息
    pub visa_center_details: VisaCenterDetails, // 签证中心及预约信息
    pub appointment_info: Option<AppointmentInfo>,     // 预约成功后的票据id
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone, Debug)]
pub struct AppointmentInfo {
    pub ticket_id: String,
    pub appointment_id: String,
    pub appointment_time: i64,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone, Debug)]
pub struct EmergencyContact {
    pub first_name: String,   // 名字
    pub name: String,         // 名字
    pub last_name: String,    // 姓
    pub phone_number: String, // 电话 "+43 660 4506825"
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone, Debug)]
pub struct VisaCenterDetails {
    pub procedure_id: u32,
    pub cat_id: u32,                // type
    pub sub_id: u32,                // sub_type
}
/**
暂时居留
暂时居留-学生
非劳务访问
非劳务访问-10年
劳务访问
永久居留
工作许可签证
 */

impl Personal {
    pub fn new() -> Option<Personal> {
        let personal = Personal {
            first_name: "".to_string(),
            name: "".to_string(),
            last_name: "".to_string(),
            log_message: "".to_string(),
            gender: 1,
            birth_date: "".to_string(),
            marital_status: 0,
            phone: "".to_string(),
            country_id: 0,
            state_id: 0,
            city_address: "".to_string(),
            passport: None,
            nut: None,
            priority: 0,
            appointment_start: 0,
            appointment_end: 0,
            emergency_contact: EmergencyContact {
                name: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone_number: "".to_string(),
            },
            visa_center_details: VisaCenterDetails {
                procedure_id: 0,
                cat_id: 0,
                sub_id: 0,
            },
            appointment_info: None,
        };
        Some(personal)
    }
}

static IDS_COUNT: AtomicUsize = AtomicUsize::new(0);
static PERSONAL_GROUP: &str = "personal_group";
pub struct PersonalService {
    option: StreamReadOptions,
    cli: ConnectionManager,
    current: Option<(String, Personal)>,
}

static LOAD_PERSONAL_INIT: Lazy<Option<()>> = Lazy::new(|| {
    let mut cli = get_sync_redis_connect().unwrap();
    match cli.xgroup_create_mkstream::<&str, &str, &str, String>(PersonalType::Valid.as_str(), PERSONAL_GROUP, "0") {
        Ok(_) => {
            info!("创建消费组[personal_group]成功!");
        }
        Err(e) => {
            info!("创建消费组错误(不影响工作): {}", e);
        }
    }
    Some(())
});

static RESET_TRANSACTION: Lazy<Transaction> = Lazy::new(|| Transaction::new());
impl PersonalService {
    pub async fn new() -> Result<PersonalService> {
        let _ = LOAD_PERSONAL_INIT.deref();
        let mut cli = get_redis_connect().await?;
        let custom_id= IDS_COUNT.fetch_add(1, Ordering::SeqCst);
        let option = StreamReadOptions::default().group(PERSONAL_GROUP, custom_id.to_string()).count(1).noack();
        Ok(PersonalService { option, cli, current: None })
    }
    pub async fn get_valid_personal(&mut self) -> Result<Personal> {
        let mut err_no = 0;
        loop {
            let mut reply: StreamReadReply = self.cli.xread_options(&[PersonalType::Valid.as_str()], &[">"], &self.option).await?;
            match reply.keys.get(0) {
                Some(valid_stream) => {
                    for steam_id in &valid_stream.ids {
                        if let Some(v) = steam_id.map.get("-") {
                            match v {
                                Value::Data(data) => {
                                    return match serde_json::from_slice::<Personal>(data.as_ref()) {
                                        Ok(personal) => {
                                            self.current = Some((steam_id.id.clone(), personal.clone()));
                                            Ok(personal)
                                        },
                                        Err(e) => {
                                            Err(anyhow::anyhow!("无法解析用户流数据: {}", e))
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                }
                None => {
                    RESET_TRANSACTION.run(|| {
                        if let Ok(mut cli) = get_sync_redis_connect() {
                            if let Ok(res_str) = cli.xgroup_setid::<_, _, _, String>(PersonalType::Valid.as_str(), PERSONAL_GROUP, "0") {
                                info!("重置消费组[personal_group]成功: {}", res_str);
                            }
                        }
                    }).await;
                    if err_no > 0 {
                        return Err(anyhow::anyhow!("无可用客户!"));
                    }
                    err_no+= 1;
                }
            }
        }
    }
    pub async fn record_success_personal(&mut self) -> Result<()> {
        self.record_personal(PersonalType::Success.as_str()).await
    }

    pub async fn record_exception_personal(&mut self) -> Result<()> {
        self.record_personal(PersonalType::Exception.as_str()).await
    }
    async fn record_personal(&mut self, name: &str) -> Result<()> {
        if let Some((msg_id, personal)) = self.current.clone() {
            let r = self.cli.xdel(PersonalType::Valid.as_str(), &[msg_id]).await?;
            if r {
                let _: usize = self.cli.rpush(name, personal.phone.clone()).await?;
                self.cli.hset("personal_phone", personal.phone.clone(), personal).await?;
            }
        }
        Ok(())
    }
}

// 获得一个有效客户
// pub async fn get_valid_personal() -> Result<Personal> {
//     let mut cli = get_redis_connect().await?;
//
//     loop {
//         let v: Vec<Personal> = cli.lpop(PersonalType::Valid.as_str(), NonZeroUsize::new(1)).await?;
//         if v.is_empty() {
//             return Err(anyhow::anyhow!("无可用客户!"));
//         }
//         let r = v.get(0).unwrap().clone();
//         // 验证用户希望预约的时间是否在有效范围内
//         let now = chrono::Utc::now().timestamp();
//         if r.appointment_start > now || r.appointment_end < now {
//             record_exception_personal(&r).await?;
//             continue;
//         }
//         // cli.hset(PersonalType::Using.as_str(), r.phone.clone() ,r.clone()).await?;
//         return Ok(r);
//     }
// }

// 恢复一个有效客户
// pub async fn reset_valid_personal(p: &Personal) -> Result<usize> {
//     let mut cli = get_redis_connect().await?;
//     cli.hdel(PersonalType::Using.as_str(), p.phone.clone()).await?;
//     let n:usize = cli.rpush(PersonalType::Valid.as_str(), p).await?;
//     Ok(n)
// }

// 启动进程->重置客户
// pub async fn reset_personal() ->Result<()> {
//     let mut cli = get_redis_connect().await?;
//     // 遍历 PersonalType::Using 的 hash
//     let result: HashMap<String, Personal> = cli.hgetall(PersonalType::Using.as_str()).await?;
//     for (_, value) in result.iter() {
//         if value.priority > 0 {
//             cli.lpush(PersonalType::Valid.as_str(), value).await?;
//         } else {
//             cli.rpush(PersonalType::Valid.as_str(), value).await?;
//         }
//     }
//     cli.del(PersonalType::Using.as_str()).await?;
//     Ok(())
// }

// 清理全部个人有关数据
pub async fn clear_personal()->Result<()> {
    let mut cli = get_redis_connect().await?;
    cli.del(PersonalType::Valid.as_str()).await?;
    // cli.del(PersonalType::Using.as_str()).await?;
    cli.del(PersonalType::Success.as_str()).await?;
    cli.del(PersonalType::Exception.as_str()).await?;
    cli.del("personal_phone").await?;
    Ok(())
}

// 新增可用用户
pub async fn add_valid_personal(personal: &Personal) -> Result<()> {
    let mut cli = get_redis_connect().await?;
    cli.xadd(PersonalType::Valid.as_str(), "*", &[("-", personal)]).await?;
    Ok(())
}

// 记录预约成功的客户
// pub async fn record_success_personal(personal: &Personal) -> Result<()> {
//     let mut cli = get_redis_connect().await?;
//     let _: usize = cli.rpush(PersonalType::Success.as_str(), personal.phone.clone()).await?;
//     cli.hset(format!("{}_phone", PersonalType::Success.as_str()).as_str(), personal.phone.clone(), personal).await?;
//     cli.xdel(PersonalType::Success.as_str(), )
//         // .hdel(PersonalType::Using.as_str(), personal.phone.clone()).await?;
//     Ok(())
// }

// 记录预约失败的客户
// pub async fn record_exception_personal(personal: &Personal) -> Result<()> {
//     let mut cli = get_redis_connect().await?;
//     let _: usize = cli.rpush(PersonalType::Exception.as_str(), personal).await?;
//     cli.hset(format!("{}_phone", PersonalType::Exception.as_str()).as_str(), personal.phone.clone(), personal).await?;
//     cli.hdel(PersonalType::Using.as_str(), personal.phone.clone()).await?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use crate::gsc::time_until::delay_min_max_secs;
    use super::*;

    fn build_personal_info() -> Personal {
         Personal {
            name: "LI".to_string(),
            first_name: "LI".to_string(),
            last_name: "".to_string(),
            gender: 1,
            birth_date: "1997-06-25".to_string(),
            marital_status: 1,
            phone: "+86 152 1165 5528".to_string(),
            log_message: "".to_string(),
            passport: None,
            nut: None,
            priority: 1,
            state_id: 3649,
            country_id: 44,
            city_address: "-".to_string(),
            appointment_start: 1713542400,  // 2024-04-20 00:00:00
            appointment_end: 1716134400,    // 2024-05-20 00:00:00 + 1d
            emergency_contact: EmergencyContact {
                name: "LI".to_string(),
                first_name: "LEI".to_string(),
                last_name: "".to_string(),
                phone_number: "+86 139 1562 0092".to_string(),
            },
            visa_center_details: VisaCenterDetails{
                procedure_id: 31,
                cat_id: 10,
                sub_id: 17,
            },
            appointment_info: None,
        }
    }
    #[tokio::test]
    async fn test_personal_status_switch_ok() {
        clear_personal().await.unwrap();
        let personal = build_personal_info();
        add_valid_personal(&personal).await.unwrap();
        let mut personal2 = personal.clone();
        personal2.phone = "+86 152 1165 5529".to_string();
        add_valid_personal(&personal2).await.unwrap();
        let mut personal_service = PersonalService::new().await.unwrap();
        for _ in 0..10 {
            let p = personal_service.get_valid_personal().await.unwrap();
            println!("personal: {:?}", p.phone);
        }
    }

    #[tokio::test]
    async fn test_personal_status_switch_empty() {
        clear_personal().await.unwrap();
        let mut personal_service = PersonalService::new().await.unwrap();
        let res = personal_service.get_valid_personal().await;
        if let Err(e) = res {
            println!("error: {}", e);
        }
        // assert_eq!(res.is_err(), true);
    }
}