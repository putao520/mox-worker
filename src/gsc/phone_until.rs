pub struct PhoneInfo {
    // 国家区号
    pub country_code: String,
    // 电话号码
    pub phone_number: String,
}

// full_phone: +43 660 4506825
pub fn get_phone_info(full_phone: &str) -> PhoneInfo {
    let mut phone_info = PhoneInfo {
        country_code: "".to_string(),
        phone_number: "".to_string(),
    };
    let phone: Vec<&str> = full_phone.split(" ").collect();
    phone_info.country_code = phone[0].to_string().clone();
    // 从第二个元素开始join, 用空格连接
    phone_info.phone_number = phone[1..].join(" ").clone();
    phone_info
}