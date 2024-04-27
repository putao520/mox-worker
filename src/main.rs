mod website;
mod gsc;
mod mox;
mod third;

use std::path::Path;
use anyhow::Result;
use calamine::{open_workbook, Reader, Xlsx};
use log::info;
use mimalloc::MiMalloc;
use redis::AsyncCommands;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use crate::gsc::config::system_config::get_system_config;
use crate::gsc::data_source::account::AccountType;
use crate::gsc::data_source::config_source::{auto_change_share_config, clear_share_config};
use crate::gsc::data_source::personal::PersonalType;
use crate::gsc::data_source::source_service::AsStringEnum;
use crate::gsc::mdl::redis::get_redis_connect;
// use crate::gsc::data_source::source_service::SourceService;
use crate::gsc::time_until::appointment_time_range;
use crate::mox::account::{auto_recover_account, clear_account, MoxAccount, MoxEndpoint, rebuild_account};
use crate::mox::appointment::start_appointment_task;
use crate::mox::helper::{appointment_type_2_ids, format_phone, gender_2_id, marital_status_2_id, state_2_id};
use crate::mox::personal::{clear_personal, EmergencyContact, Personal, reset_personal, VisaCenterDetails};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main()->Result<()>{
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Start Mox Master!");
    try_file_mode().await?;
    auto_recover_account();
    rebuild_account().await?;
    reset_personal().await?;
    let es = auto_change_share_config().await;
    start_appointment_task().await?;
    Ok(())
}

async fn load_accounts() -> Result<()> {
    println!("Current directory For Account.txt: {:?}", std::env::current_dir()?);

    let path = Path::new("file_mode/account.txt");
    let file = File::open(&path).await?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let mut cli = get_redis_connect().await?;
    while let Some(line) = lines.next_line().await? {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 2 {
            let email = parts[0].to_string().to_lowercase();
            let password = parts[1].to_string();
            println!("email: {}, password: {}", email, password);
            cli.lpush(AccountType::Valid.as_str(), MoxAccount::new(email.clone(), password.clone(), &MoxEndpoint::new_empty())).await?;
        }
    }
    Ok(())
}

async fn load_personal()->Result<()> {
    println!("Current directory For Personal.xlsx: {:?}", std::env::current_dir()?);
    let mut cli = get_redis_connect().await?;
    let path = Path::new("file_mode/personal.xlsx");
    let mut excel: Xlsx<_> = open_workbook(path)?;
    if let Ok(r) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
            if row[0] == "名字" {
                continue;
            }
            let (s,e) = appointment_time_range(row[14].to_string());
            let (a,b,c) = appointment_type_2_ids(row[13].to_string());
            let p = Personal {
                first_name: row[0].to_string(),
                name: row[1].to_string(),
                last_name: "".to_string(),
                log_message: "".to_string(),
                gender: gender_2_id(row[2].to_string()),
                birth_date: row[3].to_string(),
                marital_status: marital_status_2_id(row[9].to_string()),
                phone:  format_phone(row[4].to_string()),
                state_id: state_2_id(row[5].to_string()),
                country_id: 44,
                city_address: row[6].to_string(),
                passport: None,
                nut: None,
                appointment_start: s,
                appointment_end: e,
                priority: 0,
                emergency_contact: EmergencyContact{
                    first_name: row[10].to_string(),
                    name: row[11].to_string(),
                    last_name: "".to_string(),
                    phone_number: format_phone(row[12].to_string()),
                },
                visa_center_details: VisaCenterDetails{
                    procedure_id: a,
                    cat_id: b,
                    sub_id: c,
                },
                appointment_info: None,
            };
            cli.lpush(PersonalType::Valid.as_str(), p).await?;
        }
    }
    Ok(())
}

async fn try_file_mode() -> Result<()>{
    println!("Current directory For Folder: {:?}", std::env::current_dir()?);
    if !Path::new("file_mode").exists() {
        return Ok(());
    }

    info!("running in file mode!");

    clear_share_config().await?;
    get_system_config().await?;

    clear_account().await?;
    clear_personal().await?;

    load_accounts().await?;
    load_personal().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_try_file_mode() {
        let _ = try_file_mode().await.unwrap();
    }
}