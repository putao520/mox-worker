use dashmap::DashMap;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn gender_2_id(str: String) ->u32 {
    match str.as_str() {
        "女" => 1,
        "男" => 2,
        _ => 3,
    }
}

pub fn marital_status_2_id(str: String)->u32 {
    match str.as_str() {
        "单身" => 1,
        "已婚" => 2,
        _ => 3,         // 同居
    }
}

static STATE_ID_MAP: Lazy<DashMap<String, u32>> = Lazy::new(||{
    let data = r#"[
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Anhui",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3642
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Beijing",
            "var_oficina": true,
            "var_id_pais": 44,
            "var_id_entidad": 3643
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Chongqing",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3644
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Fujian",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3645
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Gansu",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3646
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Guangdong",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3647
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Guangxi",
            "var_oficina": true,
            "var_id_pais": 44,
            "var_id_entidad": 3648
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Guizhou",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3649
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Hainan",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3650
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Hebei",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3651
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Heilongjiang",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3652
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Henan",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3653
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Hong Kong",
            "var_oficina": true,
            "var_id_pais": 44,
            "var_id_entidad": 3654
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Hubei",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3655
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Hunan",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3656
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Inner Mongolia",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3657
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Jiangsu",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3658
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Jiangxi",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3659
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Jilin",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3660
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Liaoning",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3661
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Macau",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3662
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Ningxia",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3663
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Qinghai",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3664
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Shaanxi",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3665
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Shandong",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3666
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Shanghai",
            "var_oficina": true,
            "var_id_pais": 44,
            "var_id_entidad": 3667
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Shanxi",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3668
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Sichuan",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3669
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Tianjin",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3670
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Tibet",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3671
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Xinjiang",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3672
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Yunnan",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3673
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Zhejiang",
            "var_oficina": false,
            "var_id_pais": 44,
            "var_id_entidad": 3674
          },
          {
            "var_cad_tipo_entidad": "Entidad Federativa",
            "var_id_tipo_entidad": 1,
            "var_cad_entidad": "Guangzhou",
            "var_oficina": true,
            "var_id_pais": 44,
            "var_id_entidad": 329198
          }
        ]"#;
    let json_value: serde_json::Value = serde_json::from_str(data).unwrap();
    let map: DashMap<String, u32> = DashMap::new();
    for item in json_value.as_array().unwrap() {
        let key = item["var_cad_entidad"].as_str().unwrap().to_uppercase();
        let value = item["var_id_entidad"].as_u64().unwrap() as u32;
        map.insert(key.to_string(), value);
    }
    map
});
pub fn state_2_id(str: String)->u32 {
    let key = str.to_uppercase();
    let value = STATE_ID_MAP.get(&key).unwrap();
    *value
}

static APPOINTMENT_TYPE_MAP: Lazy<DashMap<String, (u32,u32,u32)>> = Lazy::new(||{
    let map: DashMap<String, (u32,u32,u32)> = DashMap::new();
    map.insert("暂时居留".to_string(), (31, 10, 20));
    map.insert("暂时居留-学生".to_string(), (31, 10, 21));
    map.insert("非劳务访问".to_string(), (31, 10, 17));
    map.insert("非劳务访问-10年".to_string(), (31, 10, 18));
    map.insert("劳务访问".to_string(), (31, 10, 19));
    map.insert("永久居留".to_string(), (31, 10, 22));
    map.insert("工作许可签证".to_string(), (31, 11, 2));
    map
});
pub fn appointment_type_2_ids(name: String) -> (u32,u32,u32) {
    let key = name.to_uppercase();
    let value = APPOINTMENT_TYPE_MAP.get(&key).unwrap();
    *value
}

pub fn office_id_2_state_id(office_id: u32) -> u32 {
    match office_id {
        246 => 329198,
        164 => 3667,
        167 => 3648,
        97 => 3654,
        _ => 3643,  // 59
    }
}

// 输入 13312345678,转成 +86 133 1234 5678
pub fn format_phone(phone: String) -> String {
    // 正则验证手机号是否符合 1[3-9]\d{9}
    let re = Regex::new(r"^1[3-9]\d{9}$").unwrap();
    if !re.is_match(&phone) {
        panic!("手机号码不正确:{}", phone)
    }
    let mut phone = phone.replace(" ", "");
    if phone.starts_with("+86") {
        phone = phone.replacen("+86", "", 1);
    }
    let mut result = "+86 ".to_string();
    result.push_str(&phone[0..3]);
    result.push_str(" ");
    result.push_str(&phone[3..7]);
    result.push_str(" ");
    result.push_str(&phone[7..]);
    result
}