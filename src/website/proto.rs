use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use crate::website::model::{Appointment, CatOfficeData, Color, CountryData, Date, DateData, DayEvent, EstadoCivil, Event, Genero, Interval, Nacionalidad, NewSchedule, Office, OfficePreferences, Procedure, ProcessJson, SaveDataPeople, StateData, Tramites, User};
// 空请求参数
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct EmptyRequest {
    pub data: Option<String>,   // None = 空 null
}

// 通用应答参数
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
    pub data: String,
}

// POST /api/appointment/v1/verify-user-data
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct VerifyUserDataResponse {
    pub success: bool,
    pub message: String,
}

// 验证码
// GET /api/appointment/lang/get-captcha/Base64(EMAIL)
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CaptchaResponse {
    pub img: String,    // data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAFwF…
}

// 登录
// POST /api/appointment/auth/login
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct LoginRequest {
    pub email: String, // "reubenshow74152@gmail.com"
    pub password: String, // "Qi12012hai**"
    pub location: String, // "ext"
    pub broser: String, // "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Geck…"
    pub platform: String, // "Win32"
    pub lang: String, // "zh"
    pub atuh_login: bool, // true
    pub captcha: String, // "h3n9ag4zb"
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct LoginResponse {
    pub citas_token: String, // "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwOi8vY2l0YXNhcGkuc3JlLmdvYi5teC9hcGkvYXBwb2ludG1lbnQvYXV0aC9sb2dpbiIsImlhdCI6MTcxMTg5MDczNywiZXhwIjoxNzExODk0MzM3LCJuYmYiOjE3MTE4OTA3MzcsImp0aSI6ImNSaDFESFdtNWRUU0pWMUciLCJzdWIiOjE4MjAzMjcxOCwicHJ2IjoiZDc5YmM5NTM0OWRlMWM0NTVkYmE3NTgwZTk5NWIwMDZlZTMzMTdjOSJ9.mORZpq0UKIt1bKcPdCFAjvKDMk3D0e3j1nwGmCPjAyc"
    pub user: User,
}

// 获得当前已申请未到期的预约
// POST /api/appointment/v1/get-appointment
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct AppointmentRequest {
    pub data: Option<String>,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct AppointmentResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<serde_json::Value>,
}

// 获得预约历史(仅仅是已经过期的预约)
// POST /api/appointment/v1/get-appointment-historical
// 空参数
// AppointmentResponse

// 验证用户数据
// POST /api/appointment/v1/verify-user-data
// 无参数
// MessageResponse message = "Start

// 获得已经配置过的领事馆
// POST /api/console/get-configured-offices
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ConfiguredOfficesRequest {
    pub jurisdictionId: u32, // 1
    pub system_id: u32, // 1
    pub api_key: String, // "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk"
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ConfiguredOfficesResponse {
    pub success: bool, // true
    pub errors: bool, // false
    pub offices: Vec<Office>,
    pub source: String, // "002C"
}

// 获得国家信息
// POST /api/catalog/v1/get-catalog
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CountriesRequest {
    pub countries: bool, // true
    pub auth: bool, // false
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CountriesData {
    pub countries: Vec<CountryData>, // 国家列表
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CountriesResponse {
    pub success: bool, // 成功 (示例值: true)
    pub data: CountriesData, // 数据
    pub message: String, // 消息
}

// 获得省份信息
// POST /api/catalog/v1/get-catalog
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct StateOrProvinceRequest {
    pub states: bool,
    pub country_id: u32, // 44
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct StateOrProvinceData {
    pub states: Vec<StateData>,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct StateOrProvinceResponse {
    pub success: bool, // true
    pub data: StateOrProvinceData,
    pub message: String, // "Éxito"
}

// 获得省内领事馆信息
// POST /api/catalog/v1/get-catalog
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeInProvinceRequest {
    pub offices: bool,  // true
    pub state_id: u32,  // 3643
    pub country_id: u32, // 44
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeInProvinceData {
    pub offices: Vec<CatOfficeData>,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeResponse {
    pub success: bool, // true
    pub data: OfficeInProvinceData,
    pub message: String, // "Éxito"
}

// 获得可以办理的签证类型
// POST /api/console/get-general
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct AvailableProceduresRequest {
    pub officeId: u32, // 223
    pub pcm_event_id: Option<String>, // null
    pub cat_system_id: u8, // 1
    pub api_key: String, // "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk"
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct AvailableProceduresResponse {
    pub success: bool, // true
    pub errors: bool, // false
    pub availableProcedures: Vec<Vec<Procedure>>,
}


// 验证NUT(工作申请签证)
// POST /api/appointment/v1/search-nud
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SearchNudRequest {
    pub nut: String, // nut 号
    pub passport: String,   // 护照号 EL1555539
    pub nombres: String, // ZHU 名字第一个字
    pub apellidos: String, // YE 名字第二个字
    pub sexo: String, // "Femenino" 性别
    pub fechaNacimiento: String, // "32/04/1992" 生日
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SearchNudResponse {
    pub status: bool, // 请求状态
    pub validate: bool,   // NUT有效性
    pub message: String, // 接口信息
    pub errors: Option<String>, // 错误信息
}

// 取消申请
// POST /api/appointment/v1/close-appointment
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CloseAppointmentRequest {
    pub id: String, // 申请编号
}

// 领事馆个性设置
// POST /api/console/office-preferences
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficePreferencesRequest {
    pub officeId: u32, // 223
    pub api_key: String, // "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk"
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficePreferencesResponse {
    pub success: bool, // true
    pub errors: bool, // false
    pub office_preferences: OfficePreferences,
}

// 获得预约表单的多个输入的基础候选项
// POST /api/catalog/v1/get-catalog
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GetGeneroAndNationalitiesAndMaritalStatusAndOfficeIdentifierRequest {
    pub obtenerGenero: bool, // true
    pub obtenerCatalogoNacionalidades: bool, // true
    pub obtenerEstatosCiviles: bool, // true
    pub p_id_oficina: u32, // 223
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GeneroResponse {
    pub success: bool, // true
    pub message: String, // "success"
    pub code: String, // "000"
    pub lResult: Vec<Genero>,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct NacionalidadResponse {
    pub success: bool, // true
    pub message: String, // "success"
    pub code: String, // "000"
    pub lResult: Vec<Nacionalidad>,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct EstadoCivilResponse {
    pub success: bool, // true
    pub message: String, // "success"
    pub code: String, // "000"
    pub lResult: Vec<EstadoCivil>,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GetGeneroAndNationalitiesAndMaritalStatusAndOfficeIdentifierData {
    pub obtenerGenero: GeneroResponse,
    pub obtenerCatalogoNacionalidades: NacionalidadResponse,
    pub obtenerEstatosCiviles: EstadoCivilResponse,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GetGeneroAndNationalitiesAndMaritalStatusAndOfficeIdentifierResponse {
    pub success: bool, // true
    pub data: GetGeneroAndNationalitiesAndMaritalStatusAndOfficeIdentifierData,
    pub message: String, // "Éxito"
}

// 检查是否在公共服务系统重复预约
// POST /api/suet/v1/service-no-curp-suet
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ServiceNoCurpSuetRequest {
    pub p_nombre: String, // "MA" - 名字
    pub p_ap_paterno: String, // "JACK" - 父姓
    pub p_ap_materno: String, // "" - 母姓
    pub p_fec_nacimiento: String, // "1990-09-22" - 出生日期
    pub p_id_oficina: u32, // 223 - 办公室编号
    pub p_id_entidad_nacimiento: u32, // 3372 - 出生地省份编号
    pub p_id_pais_nacimiento: u32, // 14 - 出生国家编号
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ServiceNoCurpSuetResult {
    pub tramites: Tramites, // "事务"
    pub out_bol_tiene_expediente: bool, // false, "是否有文件"
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ServiceNoCurpSuetResponse {
    pub success: bool, // true, "成功"
    pub message: String, // "success", "消息"
    pub code: String, // "000", "代码"
    pub lResult: Vec<ServiceNoCurpSuetResult>, // "结果"
}

// 暂存申请数据
// POST /api/appointment/v1/save-data
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SaveDataRequest {
    pub id: Option<String>, // 请求ID，可以为空
    pub origin: u8, // 来源，1 表示...
    pub typeAppointment: u8, // 预约类型，1 表示...
    pub awaitStep: bool, // 是否等待步骤，true 表示...
    pub folioProcedureInitial: Option<String>, // 初始流程号，例如 "131032024-17165"
    pub country_id: u32, // 国家 ID，例如 14
    pub country_data: CountryData, // 国家数据，包含 {CountryData}
    pub state_id: u32, // 州/省 ID，例如 3380
    pub state_data: StateData, // 州/省数据，包含 {StateData}
    pub cat_office_id: u32, // 办公室类别 ID，例如 223
    pub cat_office_data: CatOfficeData, // 办公室类别数据，包含 {CatOfficeData}
    pub folioAppointment: Option<String>, // 预约流程号，可以为空
    pub dateAppointment: Option<String>, // 预约日期，可以为空
    pub hourStarAppointment: Option<String>, // 预约开始时间，可以为空
    pub hourEndAppointment: Option<String>, // 预约结束时间，可以为空
    pub created_by_id: Option<String>, // 创建者 ID，可以为空
    pub cat_apmt_type_appointment_id: Option<String>, // 预约类型 ID，可以为空
    pub office_selected: bool, // 是否选择了办公室，true 表示...
    pub people: Vec<SaveDataPeople>, // 参与人员列表，例如 [SaveDataPeople]
    pub officeConfigData: OfficePreferences, // 办公室配置数据，包含 {OfficePreferences}
    pub setTempFormalitiesConsole: AvailableProceduresResponse, // 可用流程列表，包含 {AvailableProceduresResponse}
    pub step: u8, // 当前步骤，例如 2
}

// #[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
// pub struct SaveDataFullRequest {
//     pub id: Option<String>, // null
//     pub newSchedule: Option<NewSchedule>, // {NewSchedule}
//     pub origin: u8, // 1
//     pub typeAppointment: u8, // 1
//     pub awaitStep: bool, // true
//     pub folioProcedureInitial: String, // "131032024-17165"
//     pub GrecaptchaResponse: Option<String>, // null
//     pub program: Option<bool>,
//     pub country_id: u32, // 14
//     pub country_data: CountryData, // {CountryData}
//     pub state_id: u32, // 3380
//     pub state_data: StateData, // {StateData}
//     pub cat_office_id: u32, // 223
//     pub cat_office_data: CatOfficeData, // {CatOfficeData}
//     pub folioAppointment: Option<String>, // null
//     pub dateAppointment: Option<String>, // null
//     pub hourStarAppointment: Option<String>, // null
//     pub hourEndAppointment: Option<String>, // null
//     pub created_by_id: Option<String>, // null
//     pub cat_apmt_type_appointment_id: Option<String>, // null
//     pub office_selected: bool, // true
//     pub people: serde_json::Value, // [SaveDataPeople]
//     pub officeConfigData: OfficePreferences, // {OfficePreferences}
//     pub setTempFormalitiesConsole: AvailableProceduresResponse, // {AvailableProceduresResponse}
//     pub step: u8, // 2
//     pub step_token: Option<String>, // "245215"
//     pub trakingId: String, // "eyJpdiI6Im5WM3JYMXQrcmtjMzdvcGJEUk41M2c9PSIsInZhbHVlIjoiUjFEZVlFK0g0MDk5cDRoY1NHUnYzZz09IiwibWFjIjoiNGQwYWEyYzYzZDc0MGIxNzBlODEwYmNkMTM0YThlMGRjY2QyNzI4NzE0ODVkYThkNmRmNWY5OGVjMGRlZDA4NyJ9"
//     pub diffMin: u32, // 3598
// }
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SaveDataResponse {
    pub success: bool, // true
    pub message: String, // "success"
    pub data: serde_json::Value, // {SaveDataRequest}
    pub id: Option<String>, // null
}

// 获得当前进度
// POST /api/appointment/v1/get-process
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GetProcessRequest {
    pub id: String,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GetProcessResponseData {
    pub id: u64, // 554813149
    pub typeAppointment: u8, // 1
    pub folioProcedureInitial: String, // "131032024-17165"
    pub cat_office_id: u32, // 223
    pub cat_date_statuses_id: u8, // 8
    pub step: u8, // 2
    pub json: ProcessJson, // {ProcessJson}
    pub isActive: bool, // true
    pub created_by_id: u64, // 182032718
    pub appointment_id: Option<String>, // null
    pub origin: u8, // 1
    pub created_at: String, // "2024-03-31T13:14:14.000000Z"
    pub updated_at: String, // "2024-03-31T13:14:14.000000Z"
    pub deleted_at: Option<String>, // null
    pub request_lc: Option<String>, // null
    pub tried: Option<String>, // null
    pub step_token: String, // "245215"
    pub tried_calendar: Option<String>, // null
    pub trakingId: String, // "eyJpdiI6"
    pub hash: String, // "eyJpdiI"
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GetProcessResponse {
    pub success: bool, // true
    pub message: String, // "Éxito"
    pub data: GetProcessResponseData, // {GetProcessResponseData}
    pub diffMin: String, // "0"
    pub segundos: u32, // 3598
}

// 获得证据性服务文件(主要判断当前用户是否重复预约)
// POST /api/suet/v1/service-get-document-evidentiary
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ServiceGetDocumentEvidentiaryRequest {
    pub p_tramites: String, // "待处理的申请事务列表，JSON数组格式，示例值：[{\"id_tramite\":\"31\",\"id_tipo_tramite\":\"10\",\"id_subtipo_tramite\":\"17\",\"id_rol\":1}]"
    pub p_edad: u32, // 年龄，示例值：33
    pub p_id_nacionalidad: u32, // 国籍ID，示例值：14
    pub p_bol_naturalizado: bool, // 是否入籍，示例值：false
    pub p_bol_discapacidad: bool, // 是否残疾，示例值：false
    pub p_bol_asistencia: bool, // 是否需要协助，示例值：false
    pub p_id_edo_civil: u32, // 婚姻状况ID，示例值：1
    pub p_bol_apellido_conyuge: bool, // 是否有配偶姓氏，示例值：false
    pub p_id_oficina: u32, // 办公室ID，示例值：223
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ServiceGetDocumentEvidentiaryResponse {
    pub success: bool, // true
    pub message: String, // "success"
    pub code: String, // "000"
    pub lResult: Vec<String>, // [] 返回空数组表示没有重复预约
}

// 校验验证码
// POST /api/appointment/v1/validate-captcha
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ValidateCaptchaRequest {
    pub captcha: String,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ValidateCaptchaResponse {
    pub status: bool,
    pub error: String,
}

// 领事馆数据
// GET /api/console/office-data
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeDataRequest {
    /// 领事馆 ID, 示例值: 223
    pub cat_office_id: u32,

    /// 进程 ID, 示例值: "eyJpdiI6Im5WM3JYMXQrcmtjMzdvcGJEUk41M2c9PSIsInZhbHVlIjoiUjFEZVlFK0g0MDk5cDRoY1NHUnYzZz09IiwibWFjIjoiNGQwYWEyYzYzZDc0MGIxNzBlODEwYmNkMTM0YThlMGRjY2QyNzI4NzE0ODVkYThkNmRmNWY5OGVjMGRlZDA4NyJ9"
    pub procces_id: String,

    /// 管辖区 ID, 示例值: 1
    pub jurisdictionId: u32,

    /// 验证码值, 示例值: "0ujuxutqp"
    pub captcha_value: String,

    /// 用户, 示例值: "eyJpdiI6Ilh3d01TZkVXdEt6S1FXSUQ5Szh4c1E9PSIsInZhbHVlIjoieXRkeE5PbS85TEl2b0dkR1ordm5QZz09IiwibWFjIjoiMTRmZGNlNGU3MjdmYzQzZDkyNTM4MmY5NGRjZGUzZDljODY0NmM1Mjk0MDk3MDAwNTA2OGQ1Zjk2NzhhMjQ4NyJ9"
    pub usr: String,

    /// API 密钥, 示例值: "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk"
    pub api_key: String,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeDataResponse {
    pub success: bool, // true
    pub availability_colors: Vec<Color>, // [{Color}]
    pub offDays: Vec<String>, // ["2021-01-01"]
    pub full_colors: Vec<Color>, // [{Color}]
}

// 领事馆日程
// GET /api/console/office-events
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeEventsRequest {
    pub currentView: String, // "dayGridMonth"
    pub interval: Interval, // {Interval}
    pub dateData: DateData, // {DateData}
    pub cat_system_id: u8, // 1
    pub procces_id: String, // "eyJpdiI6Im5WM3JYMXQrcmtjMzdvcGJEUk41M2c9PSIsInZhbHVlIjoiUjFEZVlFK0g0MDk5cDRoY1NHUnYzZz09IiwibWFjIjoiNGQwYWEyYzYzZDc0MGIxNzBlODEwYmNkMTM0YThlMGRjY2QyNzI4NzE0ODVkYThkNmRmNWY5OGVjMGRlZDA4NyJ9"
    pub captcha_value: String, // "0ujuxutqp"
    pub usr: String, // "eyJpdiI6Ilh3d01TZkVXdEt6S1FXSUQ5Szh4c1E9PSIsInZhbHVlIjoieXRkeE5PbS85TEl2b0dkR1ordm5QZz09IiwibWFjIjoiMTRmZGNlNGU3MjdmYzQzZDkyNTM4MmY5NGRjZGUzZDljODY0NmM1Mjk0MDk3MDAwNTA2OGQ1Zjk2NzhhMjQ4NyJ9"
    pub api_key: String, // "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk"
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeEventsResponse {
    pub success: bool, // true
    pub events: Vec<Event>, // [{Event}]
    pub existsConfig: bool, // true
}

// 领事馆指定日期的日程
// GET /api/console/office-events
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeDayEventsRequest {
    pub selectedDate: String, // "2024-04-23"
    pub dateData: DateData, // {DateData}
    pub cat_system_id: u8, // 1
    pub procces_id: String, // "eyJpdiI6Im5WM3JYMXQrcmtjMzdvcGJEUk41M2c9PSIsInZhbHVlIjoiUjFEZVlFK0g0MDk5cDRoY1NHUnYzZz09IiwibWFjIjoiNGQwYWEyYzYzZDc0MGIxNzBlODEwYmNkMTM0YThlMGRjY2QyNzI4NzE0ODVkYThkNmRmNWY5OGVjMGRlZDA4NyJ9"
    pub captcha_value: String, // "0ujuxutqp"
    pub usr: String, // "eyJpdiI6Ilh3d01TZkVXdEt6S1FXSUQ5Szh4c1E9PSIsInZhbHVlIjoieXRkeE5PbS85TEl2b0dkR1ordm5QZz09IiwibWFjIjoiMTRmZGNlNGU3MjdmYzQzZDkyNTM4MmY5NGRjZGUzZDljODY0NmM1Mjk0MDk3MDAwNTA2OGQ1Zjk2NzhhMjQ4NyJ9"
    pub api_key: String, // "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk"
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct OfficeDayEventsResponse {
    pub success: bool, // true
    pub events: serde_json::Value, // [{DayEvent}]
    pub existsConfig: bool, // true
    pub errors: bool, // false
}

// 提交申请
// POST /api/appointment/v1/save-appointment
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SaveAppointmentRequest {
    pub date: Date, // {Date}
    pub form: serde_json::Value, // {SaveAppointmentForm}
    pub newSchedule: NewSchedule, // {NewSchedule}
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SaveAppointmentResponse {
    pub success: bool, // true
    pub message: String, // "Éxito"
    pub ticket: String, // "EP230424111042000247"   // 编号,下载用的到
    pub date: String, // "2024-04-23"
    pub hour: String, // "11:10:00"
}

// 下载预约证明
// POST /api/appointment/v1/generate-documents
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GenerateDocumentsRequest {
    pub folio: String, // "EP180424115045507001"
    pub form: serde_json::Value, // {SaveAppointmentForm}
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct GenerateDocumentsResponse {
    pub success: bool, // true
    pub message: String, // "Éxito"
    pub file: String, // "JVB"
    pub pago: Option<String>, // null
    pub cero: Option<String>, // null
    pub uno: Option<String>, // null
    pub dos: Option<String>, // null
    pub tres: Option<String>, // null
    pub cuatro: Option<String>, // null
    pub cinco: Option<String>, // null
    pub seis: Option<String>, // null
    pub siete: Option<String>, // null
    pub ocho: Option<String>, // null
    pub diez: Option<String>, // null
    pub once: Option<String>, // null
    pub doce: Option<String>, // null
}
// 取消预约
// POST /api/appointment/v1/cancel-appointment
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CancelAppointmentData {
    pub data: Appointment,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CancelAppointmentRequest {
    pub data: CancelAppointmentData,
}
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CancelAppointmentResponse {
    pub success: bool, // true
    pub message: String, // "success"
}

// 退出登录
// POST /api/appointment/v1/logout
// 空参数
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct LogoutResponse {
    pub authenticated: bool, // false
    pub message: String, // "Éxito"
}
