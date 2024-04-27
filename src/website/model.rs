use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use crate::website::proto::AvailableProceduresResponse;

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Phone {
    /// 唯一标识符 - 示例值: 512773818
    pub id: u64,
    /// 电话号码 - 示例值: "+86 198 8541 1023"
    pub phone: String,
    /// 创建时间 - 示例值: "2024-03-31T08:33:28.000000Z"
    pub created_at: String,
    /// 更新时间 - 示例值: "2024-03-31T08:33:28.000000Z"
    pub updated_at: String,
    /// 删除时间（可选） - 示例值: 无
    pub deleted_at: Option<String>,
    /// Laravel 通过键 - 示例值: 508582770
    pub laravel_through_key: u64,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Person {
    /// 唯一标识符 - 示例值: 508582770
    pub id: u64,
    /// 全名 - 示例值: "YE LIU"
    pub fullName: String,
    /// 名字 - 示例值: "YE"
    pub name: String,
    /// 主要电话
    pub primary_phone: Phone,
    /// 额外电话（可选） - 示例值: 无
    pub extra_phone: Option<String>,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct User {
    /// 用户ID - 示例值: 182032718
    pub id: u64,
    /// 电子邮件地址 - 示例值: "reubenshow74152@gmail.com"
    pub email: String,
    /// 电子邮件验证时间 - 示例值: "2024-03-31T08:34:06.000000Z"
    pub email_verified_at: String,
    /// 是否禁用 - 示例值: false
    pub isDisabled: bool,
    /// 禁用时间（可选） - 示例值: 无
    pub isDisabled_at: Option<String>,
    /// 是否激活 - 示例值: true
    pub isActive: bool,
    /// 预约类型 - 示例值: 1
    pub typeAppointment: u8,
    /// 默认语言 - 示例值: "zh"
    pub langDefault: String,
    /// 个人ID - 示例值: 508582770
    pub person_id: u64,
    /// 预约配置文件类别ID - 示例值: 1
    pub cat_apmt_profiles_id: u8,
    /// 国家类别ID - 示例值: 44
    pub cat_country_id: u8,
    /// 国籍类别ID - 示例值: 44
    pub cat_nationality_id: u8,
    /// 州类别ID - 示例值: 329198
    pub cat_state_id: u64,
    /// 办公室类别ID - 示例值: 246
    pub cat_office_id: u16,
    /// 尝试次数 - 示例值: 0
    pub attempt_number: u8,
    /// 尝试锁定日期（可选） - 示例值: 无
    pub attempt_lock_date: Option<String>,
    /// 是否由呼叫中心创建（可选） - 示例值: 无
    pub isCreatedofCallCenter: Option<bool>,
    /// 是否被阻止 - 示例值: false
    pub isBlocked: bool,
    /// 是否认证 - 示例值: true
    pub isAuth: bool,
    /// 是否开始预约 - 示例值: false
    pub startAppointment: bool,
    /// 创建时间 - 示例值: "2024-03-31T08:33:28.000000Z"
    pub created_at: String,
    /// 更新时间 - 示例值: "2024-03-31T13:12:17.000000Z"
    pub updated_at: String,
    /// 删除时间（可选） - 示例值: 无
    pub deleted_at: Option<String>,
    /// 清洁电子邮件 - 示例值: "reubenshow74152@gmailcom"
    pub cleanEmail: String,
    /// 是否有活跃令牌 - 示例值: true
    pub has_active_token: bool,
    /// 令牌过期日期 - 示例值: "2024-03-31 08:12:17"
    pub token_exp_date: String,
    /// 当前令牌 - 示例值: "eyJpdiI6IlJBVTFiVlNSRVQzWEhvemowVU43bnc9PSIsInZhb..."
    pub current_token: String,
    /// 哈希值 - 示例值: "eyJpdiI6Ilh3d01TZkVXdEt6S1FXSUQ5Szh4c1E9PSIsInZhb..."
    pub hash: String,
    /// 个人信息
    pub person: Person,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Slot {
    pub id: u64, // ID编号，示例值: 227615267
    pub appointment_id: u64, // 预约编号，示例值: 2840963443
    pub date: String, // 日期，示例值: "2024-04-04"
    pub initialTime: String, // 初始时间，示例值: "09:00:00"
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Status {
    pub id: u8, // 状态ID，示例值: 1
    pub name: String, // 状态名称，示例值: "Programada"
    pub movement: String, // 状态变动信息，示例值: "Cita programada por el usuario"
    pub color_code: String, // 颜色代码，示例值: "#70D7FF"
    pub icon_name: String, // 图标名称，示例值: "fas fa-calendar-day"
    pub created_at: Option<String>, // 创建时间，示例值: null
    pub updated_at: Option<String>, // 更新时间，示例值: null
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Ticket {
    pub id: u64, // 票据ID，示例值: 2909806480
    pub folioAppointment: String, // 预约号，示例值: "EP040424090019400167"
    pub cat_date_status_id: u8, // 类别日期状态ID，示例值: 1
    pub created_at: String, // 创建时间，示例值: "2024-03-23T08:14:36.000000Z"
    pub updated_at: String, // 更新时间，示例值: "2024-03-23T08:14:36.000000Z"
    pub deleted_at: Option<String>, // 删除时间，示例值: null
    pub updated_by: Option<String>, // 更新者，示例值: null
    pub user_system_id: Option<String>, // 用户系统ID，示例值: null
    pub status: Status, // 状态，示例值: 见Status结构体
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Pivot {
    pub appointment_id: u64, // 预约编号，示例值: 2840963443
    pub person_id: u64, // 人员编号，示例值: 508288467
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Formalities {
    pub id: u64, // 手续ID，示例值: 320007455
    pub formalitites_id: u8, // 手续编号，示例值: 31
    pub formalitites_name: String, // 手续名称，示例值: "Visas"
    pub formalitites_type_id: u8, // 手续类型编号，示例值: 10
    pub formalitites_type_name: String, // 手续类型名称，示例值: "Sin permiso del INM "
    pub formalitites_subtype_id: u8, // 手续子类型编号，示例值: 18
    pub formalitites_subtype_name: String, // 手续子类型名称，示例值: "Visitante sin permiso para realizar actividades remuneradas larga duración"
    pub passportNumber: Option<String>, // 护照号码，示例值: null
    pub nud: Option<String>, // NUD号码，示例值: null
    pub validity_id: Option<String>, // 有效性编号，示例值: null
    pub validity_name: Option<String>, // 有效性名称，示例值: null
    pub discount_id: Option<String>, // 折扣编号，示例值: null
    pub discount_name: Option<String>, // 折扣名称，示例值: null
    pub amount: Option<String>, // 金额，示例值: null
    pub created_at: String, // 创建时间，示例值: "2024-03-23T08:14:36.000000Z"
    pub updated_at: String, // 更新时间，示例值: "2024-03-23T08:14:36.000000Z"
    pub deleted_at: Option<String>, // 删除时间，示例值: null
    pub pivot: Pivot, // 关联结构体，示例值: 见Pivot结构体
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Persons {
    pub id: u64, // 人员ID，示例值: 508288467
    pub curp: String, // CURP号码，示例值: ""
    pub fullName: String, // 全名，示例值: "YU YUE "
    pub name: String, // 名字，示例值: "YU"
    pub firstName: String, // 名，示例值: "YUE"
    pub lastName: String, // 姓，示例值: ""
    pub firstNameMarried: Option<String>, // 已婚者的名字，示例值: null
    pub birthdate: String, // 出生日期，示例值: "1989-03-14"
    pub statusCurp: String, // CURP状态，示例值: "0"
    pub docProbatorio: String, // 证明文件，示例值: "0"
    pub isValidateCurp: bool, // 是否验证CURP，示例值: false
    pub naturalized: bool, // 是否入籍，示例值: false
    pub disability: bool, // 是否残疾，示例值: false
    pub civilState: u8, // 婚姻状态，示例值: 2
    pub adoption: bool, // 是否领养，示例值: false
    pub cat_gender_id: u8, // 性别编号，示例值: 2
    pub cat_nationality_id: u8, // 国籍编号，示例值: 44
    pub cat_apmt_type_person_id: u8, // 预约类型人员编号，示例值: 1
    pub country_id: u8, // 国家编号，示例值: 44
    pub created_by_id: u64, // 创建者ID，示例值: 181392796
    pub aceptNotificacionPhone: bool, // 是否接受电话通知，示例值: false
    pub annotations: Option<String>, // 注释，示例值: null
    pub created_at: String, // 创建时间，示例值: "2024-03-23T08:14:36.000000Z"
    pub updated_at: String, // 更新时间，示例值: "2024-03-23T08:14:36.000000Z"
    pub deleted_at: Option<String>, // 删除时间，示例值: null
    pub pivot: Pivot, // 关联结构体，示例值: 见Pivot结构体
    pub formalities: Vec<Formalities>, // 手续列表，示例值: []
    pub documents: Vec<String>, // 文档列表，示例值: []
    pub accompanying_person: Vec<String>, // 陪同人员列表，示例值: []
}


#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Configuration {
    pub id: u64, // 配置ID，示例值: 743
    pub cat_office_id: u8, // 办公室类别ID，示例值: 74
    pub isCancelable: bool, // 是否可取消，示例值: true
    pub isReschedulable: bool, // 是否可重新安排，示例值: false
    pub isCancelableCallCenter: bool, // 是否可通过呼叫中心取消，示例值: false
    pub isReschedulableCallCenter: bool, // 是否可通过呼叫中心重新安排，示例值: false
    pub enableCaptcha: bool, // 是否启用验证码，示例值: true
    pub amountDates: u8, // 日期数量，示例值: 1
    pub amountProcedures: u8, // 手续数量，示例值: 1
    pub amountPeople: u8, // 人员数量，示例值: 1
    pub created_by: u8, // 创建者，示例值: 159
    pub updated_by: u8, // 更新者，示例值: 159
    pub created_at: String, // 创建时间，示例值: "2021-05-30T02:03:15.000000Z"
    pub updated_at: String, // 更新时间，示例值: "2021-07-02T15:07:40.000000Z"
    pub cat_jurisdiction_id: u8, // 司法管辖区ID，示例值: 1
    pub visible_cc: bool, // 是否在呼叫中心可见，示例值: false
    pub visible_pp: bool, // 是否在移动平台可见，示例值: false
    pub limit_by_day: Option<String>, // 每日限制，示例值: null
    pub visible_cb: bool, // 是否在前台柜台可见，示例值: false
    pub capture_line_enabled: bool, // 是否启用捕捉行，示例值: false
    pub hash_id: String, // 哈希ID，示例值: "eyJpdiI6Im9WQkMxeWxyMGhpTS81UHJNSzl4RWc9PSIsInZhbHVlIjoibUp4bGEwYjJkVjdRZDMwQnhnTVMvZz09IiwibWFjIjoiODM4ZjU5NDljMzJjZjBhY2QzNzdjNzEzMThkOGE0NTUyZmMzZDZiODliMjg2MzFiMWEyMWZkZjY1NjBhYzI0MiJ9"
}


#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Appointment {
    pub id: u64, // 预约编号，示例值: 2840963443
    pub typeAppointment: u8, // 预约类型，示例值: 1
    pub country_id: u8, // 国家编号，示例值: 13
    pub cat_office_id: u8, // 办公室类别编号，示例值: 74
    pub cat_office_name: String, // 办公室名称，示例值: "CANBERRA"
    pub created_by_id: u64, // 创建者ID，示例值: 181392796
    pub created_callcenter_by_id: u64, // 呼叫中心创建者ID，示例值: 181392796
    pub cat_apmt_type_appointment_id: u8, // 预约类型编号，示例值: 1
    pub ticket_id: u64, // 票据ID，示例值: 2909806480
    pub created_at: String, // 创建时间，示例值: "2024-03-23T08:14:36.000000Z"
    pub updated_at: String, // 更新时间，示例值: "2024-03-23T08:14:36.000000Z"
    pub deleted_at: Option<String>, // 删除时间，示例值: null
    pub pcm_event_name: Option<String>, // PCM事件名称，示例值: null
    pub pcm_event_address: String, // PCM事件地址，示例值: "Perth Avenue"
    pub ticket_origin_id: Option<String>, // 票据来源ID，示例值: null
    pub hash: String, // 哈希值，示例值: "eyJpdiI6InVLRmYvSE9IbHhhQ21EYU5hbk90Snc9PSIsInZhbHVlIjoiall3Rm9BNDF6WngrL3BGUU1EODJwQT09IiwibWFjIjoiOTAyZDAyMzQyY2ViZjJmZGQzOTAyNWQ0NzY1ZGIyOTQ5ZTBhY2QzYjNiYjdjNTJlODZiODc2NTZlNTAzZjAyNyJ9"
    pub slot: Slot, // 时间段，示例值: 见Slot结构体
    pub ticket: Ticket, // 票据信息，示例值: 见Ticket结构体
    pub persons: Vec<Persons>, // 人员信息列表，示例值: []
    pub configuration: Configuration, // 配置信息，示例值: 见Configuration结构体
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Office {
    pub event_id: Option<String>, // 事件ID，示例值: "WAS-SR-010424-010"
    pub cat_office_id: Option<u32>, // 办公室类别ID，示例值: 31
    pub cat_office_name: Option<String>, // 办公室类别名称，示例值: "WASHINGTON SOBRE RUEDAS"
    pub observations: Vec<String>, // 观察列表，示例值: ["DOCUMENTACION A MEXICANOS. PAGO CON TARJETA"]
    pub id: u32, // ID，示例值: 112527
    pub period_id: Option<u32>, // 时期ID，示例值: 1104266
    pub name: String, // 名称，示例值: "GATE CENTER, INDEPENDENCE"
    pub address: Option<String>, // 地址，示例值: "GATE CENTER 122 DAVIS ST INDEPENDENCE VA 24348"
    pub postal_code: Option<String>, // 邮政编码，示例值: "24348"
    pub lat: Option<String>, // 纬度，示例值: "36.6230198883549"
    pub lng: Option<String>, // 经度，示例值: "-81.15129991652854"
    pub updated_by: Option<u32>, // 更新者ID，示例值: 7945
    pub created_at: Option<String>, // 创建时间，示例值: "2024-03-26 08:58:40"
    pub updated_at: Option<String>, // 更新时间，示例值: "2024-03-26 08:58:40"
    pub deleted_at: Option<String>, // 删除时间，示例值: null
    pub aux_id: Option<String>, // 辅助ID，示例值: "31-112527"
    pub isMovil: bool, // 是否移动的，示例值: true
}

// #[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
// pub struct OfficeInProvince {
//     pub var_cad_num_interior: String, // 内部号码，示例值: ""
//     pub var_id_entidad_federativa: u32, // 实体ID，示例值: 3643
//     pub var_cad_localidad: Option<String>, // 地区描述，示例值: null
//     pub var_id_oficina: u32, // 办公室ID，示例值: 59
//     pub var_cad_codigo_postal: String, // 邮政编码，示例值: ""
//     pub var_cad_oficina: String, // 办公室名称，示例值: "BEIJING"
//     pub var_cad_correo_electronico: String, // 电子邮件，示例值: "comunicacion@embmx.cn"
//     pub var_id_municipio_alcaldia: Option<u32>, // 市政府ID，示例值: null
//     pub var_num_longitud: f64, // 经度，示例值: 116.458656
//     pub var_cad_municipio_alcaldia: String, // 市政府名称，示例值: ""
//     pub var_cad_calle: String, // 街道名称，示例值: "San Li Tun Dong Wu Jie"
//     pub var_cad_entidad_federativa: String, // 实体描述，示例值: "BEIJING"
//     pub var_id_localidad: Option<u32>, // 地区ID，示例值: null
//     pub var_id_codigo_postal: Option<u32>, // 邮政编码ID，示例值: null
//     pub var_cad_num_exterior: String, // 外部号码，示例值: ""
//     pub var_cad_colonia: String, // 社区名称，示例值: ""
//     pub is_ome: bool, // 是否OME，示例值: false
//     pub var_cad_num_telefono: String, // 电话号码，示例值: "-8610 6532-2070"
//     pub var_num_latitud: f64, // 纬度，示例值: 39.944392
// }

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Procedure {
    pub date: String, // 日期，示例值: "2024-04-03"
    pub cat_office_id: u32, // 办公室类别ID，示例值: 223
    pub cat_procedure_name: String, // 手续名称，示例值: "Pasaporte y/o credencial para votar"
    pub cat_procedure_id: u32, // 手续ID，示例值: 39
    pub cat_procedure_type_name: Option<String>, // 手续类型名称，示例值: null
    pub cat_procedure_type_id: Option<u32>, // 手续类型ID，示例值: null
    pub cat_procedure_subtype_name: Option<String>, // 手续子类型名称，示例值: null
    pub cat_procedure_subtype_id: Option<u32>, // 手续子类型ID，示例值: null
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficePreferences {
    pub isCancelable: bool, // 是否可取消，示例值: true
    pub isReschedulable: bool, // 是否可重新安排，示例值: false
    pub isCancelableCallCenter: bool, // 是否可通过呼叫中心取消，示例值: false
    pub isReschedulableCallCenter: bool, // 是否可通过呼叫中心重新安排，示例值: false
    pub capture_line_enabled: bool, // 是否启用捕捉行，示例值: false
    pub enableCaptcha: bool, // 是否启用验证码，示例值: true
    pub amountDates: u8, // 日期数量，示例值: 1
    pub amountProcedures: u8, // 手续数量，示例值: 1
    pub amountPeople: u8, // 人员数量，示例值: 1
    pub hash_id: String, // 哈希ID，示例值: "eyJpdiI6IlkwMnJ4MG52WFRNRk5KcWJKK2tSTmc9PSIsInZhbHVlIjoiVEhvRThPOWhiZE1tQ0hEMXB5QWZKQT09IiwibWFjIjoiZmIzZDgxMjI3MjVlMmYyMWI4NjIzOTFhNGM0ZjhkNGE5YmRhY2QyNDBkM2U4ZTYzMmFmYjM0Mzg0MjlkY2FhYiJ9"
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Genero {
    pub var_id_genero: u32, // 性别ID，示例值: 1
    pub var_cad_genero: String, // 性别描述，示例值: "Femenino","Masculino"
    pub var_cad_ine: String, // INE代码，示例值: "M"
    pub var_cad_abis: String, // ABIS代码，示例值: "F"
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Nacionalidad {
    pub var_cad_nacionalidad: String, // 国籍描述，示例值: "Apátrida"
    pub p_id_nacionalidad: u32, // 国籍ID，示例值: 0
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct EstadoCivil {
    pub var_edad_maxima: u32, // 最大年龄，示例值: 150
    pub var_cad_estado_civil: String, // 婚姻状况描述，示例值: "Soltero(a)"/"Casado(a)"
    pub var_num_edad_minima: u32, // 最小年龄，示例值: 0/14
    pub var_id_estado_civil: u32, // 婚姻状况ID，示例值: 1/2
}

// 有效期
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Vigencia {
    /// 有效期 ID（示例值：1）
    #[serde(rename = "tt_id_vigencia")]
    pub validity_id: u32,
    /// 有效期描述（示例值："1 año"）
    #[serde(rename = "tt_cad_descripcion")]
    pub description: String,
    /// 成本（示例值为空）
    #[serde(rename = "tt_costo")]
    pub cost: Option<f64>,
    /// 货币 ID（示例值为空）
    #[serde(rename = "tt_id_moneda")]
    pub currency_id: Option<u32>,
    /// 货币名称（示例值为空）
    #[serde(rename = "tt_cad_moneda")]
    pub currency_name: Option<String>,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Discount {
    // 定义折扣相关字段
    pub v: u32
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Tramite {
    /// 手续 ID（示例值：39）
    #[serde(rename = "t_id_tramite")]
    pub procedure_id: u32,
    /// 手续名称（示例值："Pasaporte y/o credencial para votar"）
    #[serde(rename = "t_cad_tramite")]
    pub procedure_name: String,
    /// 手续类型 ID（示例值为空）
    #[serde(rename = "t_id_tipo_tramite")]
    pub procedure_type_id: Option<u32>,
    /// 手续类型名称（示例值为空）
    #[serde(rename = "t_cad_tipo_tramite")]
    pub procedure_type_name: Option<String>,
    /// 手续子类型 ID（示例值为空）
    #[serde(rename = "t_id_subtipo_tramite")]
    pub procedure_subtype_id: Option<u32>,
    /// 手续子类型名称（示例值为空）
    #[serde(rename = "t_cad_subtipo_tramite")]
    pub procedure_subtype_name: Option<String>,
    /// 国籍类型 ID（示例值为空）
    #[serde(rename = "t_id_tipo_nacionalidad")]
    pub nationality_type_id: Option<u32>,
    /// 初始年龄（示例值：0）
    #[serde(rename = "t_num_edad_inicial")]
    pub initial_age: u32,
    /// 最终年龄（示例值：150）
    #[serde(rename = "t_num_edad_final")]
    pub final_age: u32,
    /// 有效期（示例值为空）
    #[serde(rename = "t_vigencias")]
    pub validities: Option<Vec<Vigencia>>,
    /// 折扣（示例值为空）
    #[serde(rename = "t_descuentos")]
    pub discounts: Option<Vec<Discount>>,
}


// Tramites
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Tramites {
    #[serde(rename = "type")]
    pub type_: String, // "json", "类型"
    pub value: Vec<Tramite>, // "值"
}

// 国家数据
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CountryData {
    pub idalpha3: String, // "AUT", "三字母国家代码"
    pub idalpha2: String, // "AT", "两字母国家代码"
    pub tiene_edos: u8, // 1, "是否有州"
    pub id_pais: u32, // 14, "国家ID"
    pub cad_nombre_es: String, // "Austria", "国家名称"
}

// 省/州数据
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct StateData {
    pub var_cad_tipo_entidad: String, // "Ciudad", "实体类型"
    pub var_id_tipo_entidad: u32, // 5, "实体类型ID"
    pub var_cad_entidad: String, // "Wien", "实体"
    pub var_oficina: bool, // true, "是否有办公室"
    pub var_id_pais: u32, // 14, "国家ID"
    pub var_id_entidad: u32, // 3380, "实体ID"
}

fn default_observations() -> Vec<String> {
    vec!["".to_string()]
}
// 分类办公室数据
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CatOfficeData {
    /// 内部编号（示例值为空）
    #[serde(rename = "var_cad_num_interior")]
    pub interior_number: Option<String>,
    /// 联邦实体 ID（示例值：3380）
    #[serde(rename = "var_id_entidad_federativa")]
    pub federal_entity_id: u32,
    /// 地点（示例值为空）
    #[serde(rename = "var_cad_localidad")]
    pub locality: Option<String>,
    /// 办公室 ID（示例值：223）
    #[serde(rename = "var_id_oficina")]
    pub office_id: u32,
    /// 邮政编码（示例值为空）
    #[serde(rename = "var_cad_codigo_postal")]
    pub postal_code: Option<String>,
    /// 办公室名称（示例值："VIENA"）
    #[serde(rename = "var_cad_oficina")]
    pub office_name: String,
    /// 电子邮件地址（示例值："embaustria@sre.gob.mx"）
    #[serde(rename = "var_cad_correo_electronico")]
    pub email: Option<String>,
    /// 市/区 ID（示例值为空）
    #[serde(rename = "var_id_municipio_alcaldia")]
    pub municipality_id: Option<u32>,
    /// 经度（示例值：16.3668846266）
    #[serde(rename = "var_num_longitud")]
    pub longitude: f64,
    /// 市/区名称（示例值为空）
    #[serde(rename = "var_cad_municipio_alcaldia")]
    pub municipality_name: Option<String>,
    /// 街道地址（示例值："RENNGASSE 5, TOP 6, 1010, WIEN"）
    #[serde(rename = "var_cad_calle")]
    pub street_address: String,
    /// 联邦实体名称（示例值："WIEN"）
    #[serde(rename = "var_cad_entidad_federativa")]
    pub federal_entity_name: String,
    /// 地点 ID（示例值为空）
    #[serde(rename = "var_id_localidad")]
    pub location_id: Option<u32>,
    /// 邮政编码 ID（示例值为空）
    #[serde(rename = "var_id_codigo_postal")]
    pub postal_code_id: Option<u32>,
    /// 外部编号（示例值为空）
    #[serde(rename = "var_cad_num_exterior")]
    pub exterior_number: Option<String>,
    /// 社区（示例值为空）
    #[serde(rename = "var_cad_colonia")]
    pub colony: Option<String>,
    /// 是否 OME（示例值：false）
    #[serde(rename = "is_ome")]
    pub is_ome: bool,
    /// 电话号码（示例值："00431310738335"）
    #[serde(rename = "var_cad_num_telefono")]
    pub phone_number: Option<String>,
    /// 纬度（示例值：48.2127274675）
    #[serde(rename = "var_num_latitud")]
    pub latitude: f64,
    /// 观察结果（示例值为空）
    #[serde(rename = "observations", default = "default_observations")]
    pub observations: Vec<String>,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct EvidentiaryTramites {
    pub id_tramite: String, // 事务ID，示例值："31"
    pub id_tipo_tramite: String, // 事务类型ID，示例值："10"
    pub id_subtipo_tramite: String, // 子类型事务ID，示例值："17"
    pub id_rol: u32, // 角色ID，示例值：1
}

// #[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
// pub struct TempData {
//     pub t_id_tramite: String,   // 办理ID，示例值："31"
//     pub t_cad_tramite: String,  // 办理分类，示例值："Visas"
//     pub data: Vec<Tramite>,
// }

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct PersonsFormalities {
    /// 标识符 - 示例值: null
    pub id: Option<i32>,
    /// 手续编号 - 示例值: "31"
    pub formalitites_id: String,
    /// 手续名称 - 示例值: "Visas"
    pub formalitites_name: String,
    /// 手续类型编号 - 示例值: "10"
    pub formalitites_type_id: String,
    /// 手续类型名称 - 示例值: "Sin permiso del INM"
    pub formalitites_type_name: String,
    /// 手续子类型编号 - 示例值: "17"
    pub formalitites_subtype_id: Option<String>,
    /// 手续子类型名称 - 示例值: "Visitante sin permiso para realizar actividades remuneradas"
    pub formalitites_subtype_name: Option<String>,
    /// 护照号码 - 示例值: null
    pub passportNumber: Option<String>,
    /// NUD - 示例值: null
    pub nud: Option<String>,
    /// 有效性编号 - 示例值: null
    pub validity_id: Option<String>,
    /// 有效性名称 - 示例值: null
    pub validity_name: Option<String>,
    /// 折扣编号 - 示例值: null
    pub discount_id: Option<String>,
    /// 折扣名称 - 示例值: null
    pub discount_name: Option<String>,
    /// 折扣手续编号 - 示例值: null
    pub discount_formalitites_id: Option<String>,
    /// 折扣手续名称 - 示例值: null
    pub discount_formalitites_name: Option<String>,
    /// 金额 - 示例值: null
    pub amount: Option<f32>,
    /// 货币名称 - 示例值: null
    pub coin_name: Option<String>,
    /// 货币编号 - 示例值: null
    pub coin_id: Option<String>,
    /// 文件手续编号 - 示例值: null
    pub document_formalitites_id: Option<String>,
    /// 文件手续名称 - 示例值: null
    pub document_formalitites_name: Option<String>,
    /// 临时数据 - 示例值: {TempData}
    pub temp_data: Option<ApmtPersonsSelectTmpFormalities>,
    // /// 事务编号 - 示例值: "10"
    // pub id_tramite: String,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ApmtPersonsDataAddressHome {
    /// 国家 ID（示例值为空）
    #[serde(rename = "country_id")]
    pub country_id: Option<u32>,
    /// 邮政编码（示例值为空）
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    /// 州/省 ID（示例值为空）
    #[serde(rename = "state_id")]
    pub state_id: Option<u32>,
    /// 市/区 ID（示例值为空）
    #[serde(rename = "municipality_id")]
    pub municipality_id: Option<u32>,
    /// 社区 ID（示例值为空）
    #[serde(rename = "colony_id")]
    pub colony_id: Option<u32>,
    /// 详细地址（示例值为空）
    #[serde(rename = "direction")]
    pub direction: Option<String>,
    /// 街道名称（示例值为空）
    #[serde(rename = "street")]
    pub street: Option<String>,
    /// 门牌号（示例值为空）
    #[serde(rename = "outdoor_number")]
    pub outdoor_number: Option<String>,
    /// 室内号（示例值为空）
    #[serde(rename = "interior_number")]
    pub interior_number: Option<String>,
}

impl ApmtPersonsDataAddressHome {
    pub fn new_empty() -> Self {
        ApmtPersonsDataAddressHome {
            country_id: None,
            postal_code: None,
            state_id: None,
            municipality_id: None,
            colony_id: None,
            direction: None,
            street: None,
            outdoor_number: None,
            interior_number: None,
        }
    }
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ApmtPersonsDataAddressEmergency {
    /// 姓氏（示例值为空字符串）
    pub name: String,
    /// 名字（示例值为空字符串）
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// 姓字（示例值为空字符串）
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// 电子邮件地址（示例值为空）
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// 电话号码（示例值为空）
    #[serde(rename = "phone")]
    pub phone: Option<String>,
    /// 手机号码（示例值为空）
    #[serde(rename = "cell_phone")]
    pub cell_phone: Option<String>,
    /// 手机号码（示例值为空）
    // #[serde(rename = "cellPhoneFormatInternational")]
    // pub cell_phone_format_international: Option<String>,
    /// 是否与预约人住在同一地址（示例值为空）
    #[serde(rename = "sameDirection")]
    pub same_direction: Option<bool>,
    /// 国家 ID（示例值为空）
    #[serde(rename = "country_id")]
    pub country_id: Option<u32>,
    /// 邮政编码（示例值为空）
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    /// 州/省 ID（示例值为空）
    #[serde(rename = "state_id")]
    pub state_id: Option<u32>,
    /// 市/区 ID（示例值为空）
    #[serde(rename = "municipality_id")]
    pub municipality_id: Option<u32>,
    /// 社区 ID（示例值为空）
    #[serde(rename = "colony_id")]
    pub colony_id: Option<u32>,
    /// 详细地址（示例值为空）
    #[serde(rename = "direction")]
    pub address: Option<String>,
    /// 街道名称（示例值为空）
    #[serde(rename = "street")]
    pub street: Option<String>,
    /// 门牌号（示例值为空）
    #[serde(rename = "outdoor_number")]
    pub outdoor_number: Option<String>,
    /// 室内号（示例值为空）
    #[serde(rename = "interior_number")]
    pub interior_number: Option<String>,
}

impl ApmtPersonsDataAddressEmergency {
    pub fn new_empty() -> Self {
        ApmtPersonsDataAddressEmergency {
            name: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: None,
            phone: None,
            cell_phone: None,
            // cell_phone_format_international: None,
            same_direction: None,
            country_id: None,
            postal_code: None,
            state_id: None,
            municipality_id: None,
            colony_id: None,
            address: None,
            street: None,
            outdoor_number: None,
            interior_number: None,
        }
    }
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ApmtPersonsAdditional {
    /// 标识 ID（示例值为空）
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// 墨西哥人口注册码（示例值为空）
    #[serde(rename = "curp")]
    pub curp: Option<String>,
    /// 名称（示例值为空字符串）
    #[serde(rename = "name")]
    pub name: String,
    /// 名（示例值为空字符串）
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// 姓（示例值为空字符串）
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// 出生日期（示例值为空）
    #[serde(rename = "birthdate")]
    pub birthdate: Option<String>,
    /// 预约性别类别 ID（示例值为空）
    #[serde(rename = "cat_apmt_gender_id")]
    pub gender_id: Option<u32>,
    /// 国家 ID（示例值为空）
    #[serde(rename = "country_id")]
    pub country_id: Option<u32>,
    /// 国籍类别 ID（示例值为空）
    #[serde(rename = "cat_nationality_id")]
    pub nationality_id: Option<u32>,
    /// 亲属关系 ID（示例值为空）
    #[serde(rename = "parentesco_id")]
    pub kinship_id: Option<u32>,
    /// 亲属关系名称（示例值为空）
    #[serde(rename = "parentesco_name")]
    pub kinship_name: Option<String>,
    /// 补充文档 ID（示例值为空）
    #[serde(rename = "doc_complementario_id")]
    pub complementary_doc_id: Option<String>,
    /// 补充文档名称（示例值为空）
    #[serde(rename = "doc_complementario_name")]
    pub complementary_doc_name: Option<String>,
    /// 证明文档 ID（示例值为空）
    #[serde(rename = "doc_probatorio_id")]
    pub probatory_doc_id: Option<String>,
    /// 证明文档名称（示例值为空）
    #[serde(rename = "doc_probatorio_name")]
    pub probatory_doc_name: Option<String>,
    /// 国籍文档 ID（示例值为空）
    #[serde(rename = "doc_nacionalidad_id")]
    pub nationality_doc_id: Option<String>,
    /// 国籍文档名称（示例值为空）
    #[serde(rename = "doc_nacionalidad_name")]
    pub nationality_doc_name: Option<String>,
    /// 动态补充文档模型（示例值为空对象）
    #[serde(rename = "modelo_rubros_dinamico_doc_complementario")]
    pub dynamic_complementary_doc_model: serde_json::Value,
    /// 动态证明文档模型（示例值为空对象）
    #[serde(rename = "modelo_rubros_dinamico_doc_probatorio")]
    pub dynamic_probatory_doc_model: serde_json::Value,
    /// 动态国籍文档模型（示例值为空对象）
    #[serde(rename = "modelo_rubros_dinamico_doc_nacionalidad")]
    pub dynamic_nationality_doc_model: serde_json::Value,
    /// 动态临时出生证明文档模型（示例值为空对象）
    #[serde(rename = "modelo_rubros_dinamico_doc_acta_extemporanea")]
    pub dynamic_birth_certificate_model: serde_json::Value,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ApmtPersonsDocuments {
    /// 补充文档 ID（示例值为空）
    #[serde(rename = "doc_complementario_id")]
    pub complementary_doc_id: Option<String>,
    /// 补充文档名称（示例值为空）
    #[serde(rename = "doc_complementario_name")]
    pub complementary_doc_name: Option<String>,
    /// 证明文档 ID（示例值为空）
    #[serde(rename = "doc_probatorio_id")]
    pub probatory_doc_id: Option<String>,
    /// 证明文档名称（示例值为空）
    #[serde(rename = "doc_probatorio_name")]
    pub probatory_doc_name: Option<String>,
    /// 国籍文档 ID（示例值为空）
    #[serde(rename = "doc_nacionalidad_id")]
    pub nationality_doc_id: Option<String>,
    /// 国籍文档名称（示例值为空）
    #[serde(rename = "doc_nacionalidad_name")]
    pub nationality_doc_name: Option<String>,
    /// 动态补充文档模型（示例值为空对象）
    #[serde(rename = "modelo_rubros_dinamico_doc_complementario")]
    pub dynamic_complementary_doc_model: serde_json::Value,
    /// 动态证明文档模型（示例值为空对象）
    #[serde(rename = "modelo_rubros_dinamico_doc_probatorio")]
    pub dynamic_probatory_doc_model: serde_json::Value,
    /// 动态国籍文档模型（示例值为空对象）
    #[serde(rename = "modelo_rubros_dinamico_doc_nacionalidad")]
    pub dynamic_nationality_doc_model: serde_json::Value,
    /// 动态临时出生证明文档模型（示例值为空对象）
    #[serde(rename = "modelo_rubros_dinamico_doc_acta_extemporanea")]
    pub dynamic_birth_certificate_model: serde_json::Value,
}

impl ApmtPersonsDocuments {
    pub fn new_empty() -> Self {
        ApmtPersonsDocuments {
            complementary_doc_id: None,
            complementary_doc_name: None,
            probatory_doc_id: None,
            probatory_doc_name: None,
            nationality_doc_id: None,
            nationality_doc_name: None,
            dynamic_complementary_doc_model: serde_json::json!({}),
            dynamic_probatory_doc_model: serde_json::json!({}),
            dynamic_nationality_doc_model: serde_json::json!({}),
            dynamic_birth_certificate_model: serde_json::json!({}),
        }
    }
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SubtipoTramite {
    pub t_id_subtipo_tramite: String,
    pub t_cad_subtipo_tramite: String,
    pub data: Vec<Tramite>,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct TipoTramite {
    pub t_id_tipo_tramite: String,
    pub t_cad_tipo_tramite: String,
    pub data: Vec<SubtipoTramite>,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ApmtPersonsSelectTmpFormalities {
    pub t_id_tramite: String,
    pub t_cad_tramite: String,
    pub data: Vec<TipoTramite>,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SaveDataPeople {
    pub id: Option<u32>,
    pub curp: String,
    pub fullName: String,
    pub name: String,
    pub firstName: String,
    pub lastName: String,
    pub birthdate: String,
    pub age: u32,
    pub statusCurp: Option<String>,
    pub docProbatorio: Option<String>,
    pub isValidateCurp: Option<bool>,
    pub additional_person: bool,
    pub naturalized: Option<bool>,
    pub disability: Option<bool>,
    pub civilState: u32,
    pub firstNameMarried: String,
    pub adoption: Option<bool>,
    pub cat_gender_id: u32,
    pub cat_nationality_id: u32,
    pub created_by_id: Option<u32>,
    pub cat_apmt_type_person_id: Option<u32>,
    pub apmt_persons_suet_formalities_status: bool,
    pub showForm: bool,
    pub apmt_persons_tmp_renapo_search_curp: bool,
    pub country_id: u32,
    pub state_id: u32,
    pub municipality_id: Option<u32>,
    pub locality_id: Option<u32>,
    pub colony_id: Option<u32>,
    pub location: Option<String>,
    pub postalCode: Option<String>,
    pub street: Option<String>,
    pub outdoorNumber: Option<String>,
    pub interiorNumber: Option<String>,
    pub passportNummber: Option<String>,
    pub email: String,
    pub phone: String,
    pub cell_phone: Option<String>,
    pub annotations: Option<String>,
    pub step: Option<String>,
    pub aceptNotificacionPhone: bool,
    pub persons_formalities: Vec<PersonsFormalities>,
    pub apmt_persons_suet_tmp_formalities: Vec<Tramite>,
    pub apmt_persons_data_address_home: ApmtPersonsDataAddressHome,
    pub apmt_persons_data_address_emergency: ApmtPersonsDataAddressEmergency,
    pub apmt_persons_additional: ApmtPersonsAdditional,
    pub apmt_persons_second_additional: ApmtPersonsAdditional,
    pub apmt_persons_documents: ApmtPersonsDocuments,
}

impl ApmtPersonsAdditional {
    pub fn new_empty() -> Self {
        ApmtPersonsAdditional {
            id: None,
            curp: None,
            name: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            birthdate: None,
            gender_id: None,
            country_id: None,
            nationality_id: None,
            kinship_id: None,
            kinship_name: None,
            complementary_doc_id: None,
            complementary_doc_name: None,
            probatory_doc_id: None,
            probatory_doc_name: None,
            nationality_doc_id: None,
            nationality_doc_name: None,
            dynamic_complementary_doc_model: serde_json::json!({}),
            dynamic_probatory_doc_model: serde_json::json!({}),
            dynamic_nationality_doc_model: serde_json::json!({}),
            dynamic_birth_certificate_model: serde_json::json!({}),
        }
    }
}
//
// #[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
// pub struct SaveDataPeopleAndSelect {
//     pub id: Option<u32>, // ID，示例值：Some(123)
//     pub curp: String, // CURP（唯一身份证号），示例值："ABC123456DEF789XYZ"
//     pub fullName: String, // 全名，示例值："张三 李四"
//     pub name: String, // 姓，示例值："张"
//     pub firstName: String, // 名，示例值："三"
//     pub lastName: String, // 母亲的姓氏，示例值："李"
//     pub birthdate: String, // 出生日期，示例值："1990-01-01"
//     pub age: u32, // 年龄，示例值：30
//     pub statusCurp: Option<String>, // CURP状态，示例值：Some("有效")
//     pub docProbatorio: Option<String>, // 证明文件，示例值：Some("身份证")
//     pub isValidateCurp: Option<bool>, // 是否验证CURP，示例值：Some(true)
//     pub additional_person: bool, // 是否额外的人员，示例值：true
//     pub naturalized: Option<bool>, // 是否入籍，示例值：Some(false)
//     pub disability: Option<bool>, // 是否残疾，示例值：Some(false)
//     pub civilState: u32, // 婚姻状态，示例值：1
//     pub firstNameMarried: String, // 配偶的名字，示例值："小红"
//     pub adoption: Option<bool>, // 是否收养，示例值：Some(false)
//     pub cat_gender_id: u32, // 性别ID，示例值：1
//     pub cat_nationality_id: u32, // 国籍ID，示例值：14
//     pub created_by_id: Option<u32>, // 创建者ID，示例值：Some(456)
//     pub cat_apmt_type_person_id: Option<u32>, // 类型人员ID，示例值：Some(789)
//     pub apmt_persons_suet_formalities_status: bool, // 认证状态，示例值：true
//     pub showForm: bool, // 是否显示表单，示例值：true
//     pub apmt_persons_tmp_renapo_search_curp: bool, // 是否在RENAPO中搜索CURP，示例值：true
//     pub country_id: u32, // 国家ID，示例值：1
//     pub state_id: u32, // 省份ID，示例值：23
//     pub municipality_id: Option<u32>, // 市镇ID，示例值：Some(456)
//     pub locality_id: Option<u32>, // 地区ID，示例值：Some(789)
//     pub colony_id: Option<u32>, // 殖民地ID，示例值：Some(123)
//     pub location: Option<String>, // 地点，示例值：Some("街道1")
//     pub postalCode: Option<String>, // 邮政编码，示例值：Some("123456")
//     pub street: Option<String>, // 街道，示例值：Some("街道2")
//     pub outdoorNumber: Option<String>, // 户外编号，示例值：Some("100")
//     pub interiorNumber: Option<String>, // 室内编号，示例值：Some("101")
//     pub passportNummber: Option<String>, // 护照编号，示例值：Some("P1234567")
//     pub persons_formalities: Vec<PersonsFormalities>,
//     pub email: String, // 电子邮件，示例值："example@example.com"
//     pub phone: String, // 电话号码，示例值："123-456-7890"
//     pub cell_phone: Option<String>, // 手机号码，示例值：Some("123-456-7890")
//     pub annotations: Option<String>, // 注释，示例值：Some("备注")
//     pub step: Option<String>, // 步骤，示例值：Some("步骤1")
//     pub aceptNotificacionPhone: bool, // 是否接受电话通知，示例值：true
//     pub apmt_persons_suet_tmp_formalities: Vec<Tramite>, // 认证临时手续列表，示例值：[]
//     pub apmt_persons_data_address_home: ApmtPersonsDataAddressHome, // 家庭地址数据
//     pub apmt_persons_data_address_emergency: ApmtPersonsDataAddressEmergency, // 紧急地址数据
//     pub apmt_persons_additional: ApmtPersonsAdditional, // 附加人员数据
//     pub apmt_persons_second_additional: ApmtPersonsAdditional, // 第二附加人员数据
//     pub apmt_persons_documents: ApmtPersonsDocuments, // 人员文件数据
//     pub apmt_persons_select_tmp_formalities: Vec<ApmtPersonsSelectTmpFormalities>, // 选择临时手续列表
// }


#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ProcessJson {
    pub id: Option<String>,
    pub origin: u8,
    pub typeAppointment: u8,
    pub awaitStep: bool,
    pub folioProcedureInitial: Option<String>,
    pub country_id: u32,
    pub country_data: CountryData,
    pub state_id: u32,
    pub state_data: StateData,
    pub cat_office_id: u32,
    pub cat_office_data: CatOfficeData,
    pub folioAppointment: Option<String>,
    pub dateAppointment: Option<String>,
    pub hourStarAppointment: Option<String>,
    pub hourEndAppointment: Option<String>,
    pub created_by_id: Option<String>,
    pub cat_apmt_type_appointment_id: Option<String>,
    pub office_selected: bool,
    pub people: serde_json::Value,
    pub officeConfigData: OfficePreferences,
    pub setTempFormalitiesConsole: AvailableProceduresResponse,
    pub step: u8,
    pub step_token: String,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Color {
    pub id: u8,
    pub cat_jurisdiction_id: u8,
    pub description: String,
    pub color: String,
    pub percentage: u8,
    pub created_by: Option<u32>,
    pub updated_by: u32,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeProcedure {
    pub cat_procedure_id: String,
    pub cat_procedure_type_id: String,
    pub cat_procedure_subtype_id: String,
    pub cat_procedure_name: String,
    pub cat_procedure_type_name: String,
    pub cat_procedure_subtype_name: String,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct DateData {
    pub cat_office_id: u32,
    pub pcm_event_id: Option<String>,
    pub procedures: Vec<OfficeProcedure>,
    pub amountFormaliti: u32,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Interval {
    pub startDate: String, // "2024-04-01"
    pub endDate: String, // "2024-05-12"
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Event {
    pub date: String, // "2024-04-23"
    pub total_by_day: u32, // 7
    pub availables_by_day: u32, // 6
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct DayEvent {
    /// 日期
    pub date: String,
    /// 初始时间
    pub initialTime: String,
    /// 结束时间
    pub endTime: String,
    /// 产品ID
    pub cat_procedure_id: u32,
    /// 产品类型ID
    pub cat_procedure_type_id: Option<u32>,
    /// 产品子类型ID
    pub cat_procedure_subtype_id: Option<u32>,
    /// 哈希ID
    pub hash_id: u64,
    /// 时期ID
    pub period_id: u32,
    /// 每个块的总数
    pub total_by_block: u32,
    /// 每个块的可用数量
    pub availables_by_block: u32,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Date {
    pub id: u64, // 228058110
    pub start: String, // "2024-04-23T11:10:00"
    pub end: String, // "2024-04-23T11:25:00"
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct NewSchedule {
    pub id: u64, // 228058110
    pub newDateSelected: String, // "2024-04-23"
    pub newTimeSelected: String, // "11:10"
    pub newEndTimeSelected: String, // "11:25"
    pub mainProcedureSelected: String, // ""
    pub fullDate: String, // "2024-04-23T11:10:00"
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SaveAppointmentForm {
    pub id: Option<String>, // null
    pub origin: u8, // 1
    pub typeAppointment: u8, // 1
    pub awaitStep: bool, // false
    pub folioProcedureInitial: String, // "131032024-17165"
    pub country_id: u32, // 14
    pub country_data: CountryData, // {CountryData}
    pub state_id: u32, // 3380
    pub state_data: StateData, // {StateData}
    pub cat_office_id: u32, // 223
    pub cat_office_data: CatOfficeData, // {CatOfficeData}
    pub folioAppointment: Option<String>, // null
    pub dateAppointment: Option<String>, // null
    pub hourStarAppointment: Option<String>, // null
    pub hourEndAppointment: Option<String>, // null
    pub created_by_id: Option<String>, // null
    pub cat_apmt_type_appointment_id: Option<String>, // null
    pub office_selected: bool, // true
    pub people: Vec<SaveDataPeople>, // [SaveDataPeople]
    pub officeConfigData: OfficePreferences, // {OfficePreferences}
    pub setTempFormalitiesConsole: AvailableProceduresResponse, // {AvailableProceduresResponse}
    pub step: u8, // 4
    pub step_token: String, // "546125"
    pub trakingId: String, // "eyJpdiI6Im5WM3JYMXQrcmtjMzdvcGJEUk41M2c9PSIsInZhbHVlIjoiUjFEZVlFK0g0MDk5cDRoY1NHUnYzZz09IiwibWFjIjoiNGQwYWEyYzYzZDc0MGIxNzBlODEwYmNkMTM0YThlMGRjY2QyNzI4NzE0ODVkYThkNmRmNWY5OGVjMGRlZDA4NyJ9"
    pub diffMin: u32, // 3469
    pub GrecaptchaResponse: Option<String>, // null
    pub newSchedule: NewSchedule, // {NewSchedule}
    pub program: bool, // true
}
