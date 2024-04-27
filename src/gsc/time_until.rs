use std::ops::Add;
use once_cell::sync::Lazy;
use chrono::{Datelike, DateTime, Duration, FixedOffset, NaiveDate, TimeZone, Utc};

static TZ: Lazy<FixedOffset> = Lazy::new(|| FixedOffset::east_opt(8 * 3600).unwrap());

pub async fn delay_secs(seconds: u32) {
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds as u64)).await;
}

pub async fn delay_ms(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

pub async fn delay_max_secs(seconds: u32) {
    let random = 1 + (rand::random::<u32>() % seconds);
    tokio::time::sleep(tokio::time::Duration::from_secs(random as u64)).await;
}

pub async fn delay_max_ms(ms: u32) {
    let random = 1 + (rand::random::<u32>() % ms);
    tokio::time::sleep(tokio::time::Duration::from_millis(random as u64)).await;
}

pub async fn delay_min_max_secs(min: u32, max: u32) {
    let random = min + (rand::random::<u32>() % max);
    tokio::time::sleep(tokio::time::Duration::from_secs(random as u64)).await;
}

// 判断 yyyy-mm-dd 是否在 start_date 和 end_date 之间
// 判断 yyyy-mm-dd 是否在 start_date 和 end_date 之间
pub fn data_is_between(date: &str, start_date: &str, end_date: &str) -> bool {
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap()
        .and_hms_opt(0, 0, 0).unwrap()
        .and_local_timezone(TZ.clone()).unwrap();
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap()
        .and_hms_opt(23, 59, 59).unwrap()
        .and_local_timezone(TZ.clone()).unwrap();
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
        .and_hms_opt(0, 0, 0).unwrap()
        .and_local_timezone(TZ.clone()).unwrap();
    date >= start && date <= end
}

// 从 YYYY-MM-DD 计算年龄
pub fn age_from_birth_date(birth_date: &str) -> i32 {
    let birth = NaiveDate::parse_from_str(birth_date, "%Y-%m-%d").unwrap()
        .and_hms_opt(0, 0, 0).unwrap()
        .and_local_timezone(TZ.clone()).unwrap();
    let now = chrono::Utc::now().naive_utc().and_local_timezone(TZ.clone()).unwrap();
    let age = now.year() - birth.year();
    if now.month() < birth.month() || (now.month() == birth.month() && now.day() < birth.day()) {
        age - 1
    } else {
        age
    }
}

// 获得东8时区的明天的日期, 格式为 YYYY-MM-DD
pub fn get_next_day(off_day: u32) -> String {
    let tomorrow = chrono::Utc::now().naive_utc().and_local_timezone(TZ.clone()).unwrap()
        .date_naive().add(chrono::Duration::days(off_day as i64));
    tomorrow.to_string()
}

// 返回秒级时间戳
pub fn ymd_to_timestamp(date_str: &str) -> i64 {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let datetime = Utc.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0).unwrap();
    datetime.timestamp()
}

// 时间戳转 YYYY-MM-DD, 时区为东8
pub fn timestamp_to_ymd(timestamp: i64) -> String {
    let datetime = Utc.timestamp_opt(timestamp, 0).unwrap().with_timezone(&TZ.clone());
    datetime.format("%Y-%m-%d").to_string()
}

pub fn timestamp_to_date(timestamp: i64) -> NaiveDate {
    let datetime = Utc.timestamp_opt(timestamp, 0).unwrap().with_timezone(&TZ.clone());
    datetime.date_naive()
}

// 返回秒级时间戳
// time_zone = +08:00
pub fn ymd_hms_to_timestamp(date_str: &str, time_zone: &str) -> i64 {
    let dt_str = format!("{}{}", date_str, time_zone);
    let datetime_res = DateTime::parse_from_str(dt_str.as_str(), "%Y-%m-%dT%H:%M:%S%z");
    match datetime_res {
        Ok(datetime) => datetime.timestamp(),
        Err(e) => {
            println!("ymd_hms_to_timestamp error: {:?}", e);
            panic!("{}", e);
        },
    }
}

static GIRD_MAX: u32 = 42;

// 根据月份获得网格日期范围
pub fn get_gird_month_range(year: i32, month: u32) -> (NaiveDate, NaiveDate) {
    let current_date = (Utc::now() + TZ.clone()).date_naive();
    let mut start_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let week_day = start_date.weekday().num_days_from_monday();
    start_date = start_date - Duration::days(week_day as i64);  // 计算出 start_date
    let end_date = start_date + Duration::days((GIRD_MAX-1) as i64);
    if year == current_date.year() && month == current_date.month() {
        start_date = current_date + Duration::days(1);
    }
    (start_date, end_date)
}

// 获得每个月的天数
pub fn get_days_of_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub struct DateUntil {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl DateUntil {
    pub fn new() -> Self {
        let now = (Utc::now() + TZ.clone()).date_naive();
        DateUntil {
            year: now.year(),
            month: now.month(),
            day: now.day(),
        }
    }

    pub fn from(year: i32, month: u32, day: u32) -> Self {
        DateUntil {
            year,
            month,
            day,
        }
    }

    pub fn next_month(&mut self) {
        if self.month == 12 {
            self.year += 1;
            self.month = 1;
        } else {
            self.month += 1;
        }
        self.day = 1;
    }

    pub fn next_day(&mut self) {
        let days = get_days_of_month(self.year, self.month);
        if self.day == days {
            self.next_month();
        } else {
            self.day += 1;
        }
    }
}

pub fn appointment_time_range(str: String)->(i64, i64) {
    let parts: Vec<&str> = str.split("-").collect();
    if parts.len() != 2 {
        panic!("预约时间段格式错误 标准格式:YYYY年MM月DD日-YYYY年MM月DD日");
    }
    // 把 YYYY年MM月DD日 转换为时间戳(MM或者DD不足两位的补0)
    let start = parts[0].replace("年", "-").replace("月", "-").replace("日", "");
    let end = parts[1].replace("年", "-").replace("月", "-").replace("日", "");
    let start = NaiveDate::parse_from_str(start.as_str(), "%Y-%m-%d").unwrap()
        .and_hms_opt(0, 0, 0).unwrap()
        .and_local_timezone(TZ.clone()).unwrap();
    let end = NaiveDate::parse_from_str(end.as_str(), "%Y-%m-%d").unwrap()
        .and_hms_opt(23, 59, 59).unwrap()
        .and_local_timezone(TZ.clone()).unwrap();
    (start.timestamp(), end.timestamp())
}

#[cfg(test)]
mod tests {
    use crate::gsc::time_until::get_gird_month_range;
    use super::*;

    #[test]
    fn test_get_gird_month_range() {
        for i in (1..=6).rev() {
            let (s, e) = get_gird_month_range(2024, i);
            println!("m: {}=>s: {}, e: {}", i, s, e);
        }

        assert!(true);
    }

    #[test]
    fn test_p () {
        let s = 1713542400i64;
        let e = 1716134400i64;
        let current_timestamp = Utc::now().timestamp();
        let start_timestamp = if current_timestamp > s {
            current_timestamp
        } else {
            s
        };
        let personal_start_date = timestamp_to_date(start_timestamp);
        let personal_end_date = timestamp_to_date(e);
        let mut date_helper = DateUntil::from(personal_start_date.year(), personal_start_date.month(), personal_start_date.day());
        let (mut start_date, end_date) = get_gird_month_range(date_helper.year, date_helper.month);
        println!("{}", start_date.format("%Y-%m-%d").to_string())
    }

    #[test]
    fn test_ymd () {
        let r = ymd_hms_to_timestamp("2024-05-13T10:05:00", "+08:00");
        println!("{}", r);
    }
}