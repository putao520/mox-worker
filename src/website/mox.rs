use anyhow::{anyhow, Result};
use log::info;
use reqwest::Proxy;
use serde_json::{Error, Value};
use crate::gsc;

use crate::gsc::config::system_config::MoxClientConfig;
use crate::gsc::time_until::{delay_min_max_secs, delay_secs};
use crate::mox::account::MoxAccount;
use crate::third::interface_ip_pool::IpPoolServices;
use crate::website::model::{Appointment, CountryData};
use crate::website::mox_client::MoxClient;
use crate::website::mox_countries::get_local_countries;
use crate::website::proto::{AppointmentRequest, AppointmentResponse, AvailableProceduresRequest, AvailableProceduresResponse, CancelAppointmentData, CancelAppointmentRequest, CancelAppointmentResponse, CaptchaResponse, CloseAppointmentRequest, ConfiguredOfficesRequest, ConfiguredOfficesResponse, GenerateDocumentsRequest, GenerateDocumentsResponse, GetGeneroAndNationalitiesAndMaritalStatusAndOfficeIdentifierRequest, GetGeneroAndNationalitiesAndMaritalStatusAndOfficeIdentifierResponse, GetProcessRequest, GetProcessResponse, LoginRequest, LoginResponse, LogoutResponse, OfficeDataRequest, OfficeDataResponse, OfficeDayEventsRequest, OfficeDayEventsResponse, OfficeEventsRequest, OfficeEventsResponse, OfficeInProvinceRequest, OfficePreferencesRequest, OfficePreferencesResponse, OfficeResponse, SaveAppointmentRequest, SaveAppointmentResponse, SaveDataResponse, SearchNudRequest, SearchNudResponse, ServiceGetDocumentEvidentiaryRequest, ServiceGetDocumentEvidentiaryResponse, ServiceNoCurpSuetRequest, ServiceNoCurpSuetResponse, StateOrProvinceRequest, StateOrProvinceResponse, ValidateCaptchaRequest, ValidateCaptchaResponse, VerifyUserDataResponse};
use crate::website::sec_until::encode_email;

pub struct Mox{
    client: MoxClient,
    email: Option<String>,
    console_api: String,
}

impl Mox{
    pub async fn new<P: IpPoolServices>(account: MoxAccount, cfg: MoxClientConfig, proxy_impl: Option<P>, console_key: &String) -> Self {
        let client = match proxy_impl {
            Some(p) => {
                let url = p.get_ip().await.unwrap();
                #[cfg(debug_assertions)]
                info!("get_ip: {:?}", url);
                let mut proxy = Proxy::all(url).unwrap();
                if !p.is_auth() {
                    let (name, password) = p.get_auth();
                    #[cfg(debug_assertions)]
                    info!("auth: {:?}-{:?}", name, password);
                    proxy = proxy.basic_auth(name.as_str(), password.as_str());
                }
                MoxClient::new("https://citasapi.sre.gob.mx", cfg, Some(proxy))
            },
            None =>  {
                MoxClient::new("https://citasapi.sre.gob.mx", cfg, None)
            }
        };
        Mox{
            client,
            email: Option::from(account.email.clone()),
            console_api: console_key.clone(),
        }
    }

    // 获得验证码
    pub async fn get_captcha(&mut self, email: &str) -> Result<CaptchaResponse>{
        let email = encode_email(email);
        let path = format!("/api/appointment/lang/get-captcha/{}", email);
        let json_value = self.client.get(&path, &None).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 登录
    pub async fn login(&mut self, input: &LoginRequest) -> Result<Option<LoginResponse>>{
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/appointment/auth/login", &Option::from(json)).await?;
        let data: Result<LoginResponse, Error> = serde_json::from_value(json_value.clone());
        match data{
            Ok(res) => {
                self.email = Some(res.user.email.clone());
                self.client.common_headers.insert("Authorization".to_string(), format!("Bearer {}", res.citas_token.clone()));
                Ok(Some(res))
            },
            Err(_err) => {
                let blocking_res = json_value["blocking"].as_bool();
                if let Some(true) = blocking_res {
                    return Err(anyhow!("给定的账号被屏蔽"));
                }
                let message_res = json_value["message"].as_str();
                if let Some(message) = message_res {
                    // return Err(anyhow!("IP被屏蔽或者验证码失败"));
                    return if message == "El Captcha no es correcto" {
                        // 验证码错误
                        Ok(None)
                    } else {
                        Err(anyhow!(message.to_string()))
                    }
                }
                Err(anyhow!("给定的账号无法登录"))
            },  // 表示账号异常
        }
    }

    // 获得当前已申请未到期的预约
    pub async fn get_appointment(&mut self) -> Result<AppointmentResponse>{
        let json = serde_json::to_value(AppointmentRequest{
            data: None,
        })?;
        let json_value = self.client.post("/api/appointment/v1/get-appointment", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得预约历史(仅仅是已经过期的预约)
    pub async fn get_appointment_historical(&mut self) -> Result<AppointmentResponse>{
        let json = serde_json::to_value(AppointmentRequest{
            data: None,
        })?;
        let json_value = self.client.post("/api/appointment/v1/get-appointment-historical", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 取消取悦
    pub async fn close_appointment(&mut self, id: String) ->Result<()> {
        let json = serde_json::to_value(CloseAppointmentRequest{
            id
        })?;
        let mut err_no = 0;
        loop {
            match self.client.post("/api/appointment/v1/close-appointment", &Option::from(json.clone())).await {
                Ok(_) => {
                    return Ok(())
                },
                Err(err) => {
                    if err_no > 5 {
                        return Err(err);
                    }
                    err_no += 1;
                    delay_min_max_secs(3, 8).await;
                }
            }
        }
    }

    // 搜索Nut, 用于验证nut数据
    pub async fn search_nut(&mut self, input: &SearchNudRequest) -> Result<SearchNudResponse>{
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/appointment/v1/search-nud", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 验证用户数据
    pub async fn verify_user_data(&mut self) -> Result<VerifyUserDataResponse>{
        let json_value = self.client.post("/api/appointment/v1/verify-user-data", &None).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得已经配置过的领事馆
    pub async fn get_configured_offices(&mut self, input: &ConfiguredOfficesRequest) -> Result<ConfiguredOfficesResponse>{
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/console/get-configured-offices", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }
    // 获得国家数据
    // pub async fn get_countries(&mut self) -> Result<CountriesResponse>{
    //     let c = get_local_countries();
    //     if c.len() > 0 {
    //         return Ok(CountriesResponse{
    //             success: true,
    //             message: "".to_string(),
    //             data: CountriesData{
    //                 countries: LOCAL_COUNTRIES.clone(),
    //             }
    //         });
    //     }
    //     let input: CountriesRequest = CountriesRequest{
    //         countries: true,
    //         auth: false,
    //     };
    //     let json = serde_json::to_value(input)?;
    //     let json_value = self.client.set_force_no_auth(true).post("/api/catalog/v1/get-catalog", Option::from(json)).await?;
    //     serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    // }
    pub fn get_countries(&mut self)->&Vec<CountryData> {
        get_local_countries()
    }

    // 获得省份信息
    pub async fn get_provinces(&mut self, country_id: u32) -> Result<StateOrProvinceResponse>{
        let input: StateOrProvinceRequest = StateOrProvinceRequest{
            states: true,
            country_id,
        };
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/catalog/v1/get-catalog", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得省内领事馆信息
    pub async fn get_offices_by_provinces(&mut self, country_id: u32, state_id: u32) -> Result<OfficeResponse>{
        let input: OfficeInProvinceRequest = OfficeInProvinceRequest{
            offices: true,
            country_id,
            state_id,
        };
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/catalog/v1/get-catalog", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得领事馆可以办理的签证类型
    pub async fn get_general(&mut self, office_id: u32) -> Result<AvailableProceduresResponse>{
        let input = AvailableProceduresRequest{
            api_key: self.console_api.clone(),
            cat_system_id: 1,
            pcm_event_id: None,
            officeId: office_id
        };
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/console/get-general", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得领事馆偏好设置
    pub async fn get_office_preferences(&mut self, office_id: u32) -> Result<OfficePreferencesResponse>{
        let input = OfficePreferencesRequest{
            api_key: self.console_api.clone(),
            officeId: office_id,
        };
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/console/office-preferences", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得预约表单的多个输入的基础候选项
    pub async fn get_obteners(&mut self, p_id_oficina: u32) ->Result<GetGeneroAndNationalitiesAndMaritalStatusAndOfficeIdentifierResponse> {
        let input = GetGeneroAndNationalitiesAndMaritalStatusAndOfficeIdentifierRequest{
            obtenerGenero: true,
            obtenerCatalogoNacionalidades: true,
            obtenerEstatosCiviles: true,
            p_id_oficina
        };
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/catalog/v1/get-catalog", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 检查是否在公共服务系统重复预约
    pub async fn service_no_curp_suet(&mut self, input: &ServiceNoCurpSuetRequest)->Result<ServiceNoCurpSuetResponse> {
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/suet/v1/service-no-curp-suet", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 保存数据第一步
    pub async fn save_data_init(&mut self, json: &Value) -> Result<SaveDataResponse> {
        let json_value = self.client.post("/api/appointment/v1/save-data", &Option::from(json.clone())).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }
    // 保存数据
    pub async fn save_data(&mut self, json: &Value) -> Result<SaveDataResponse> {
        let json_value = self.client.post("/api/appointment/v1/save-data", &Option::from(json.clone())).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得当前进度
    pub async fn get_process(&mut self, id: &str) -> Result<GetProcessResponse> {
        let input = GetProcessRequest {
            id: id.to_string(),
        };
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/appointment/v1/get-process", &Option::from(json)).await?;
        if let Some(_) = json_value["expiredToken"].as_bool() {
            return Err(anyhow!(gsc::error::Error{no:0x30091, msg: "提交的数据异常,导致账号被封!".to_string() }))
        }
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得证据性服务文件(主要判断当前用户是否重复预约)
    pub async fn service_get_document_evidentiary(&mut self, input: &ServiceGetDocumentEvidentiaryRequest) -> Result<ServiceGetDocumentEvidentiaryResponse> {
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/suet/v1/service-get-document-evidentiary", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 校验验证码
    pub async fn validate_captcha(&mut self, captcha: &str) -> Result<ValidateCaptchaResponse>{
        let input = ValidateCaptchaRequest{
            captcha: captcha.to_string(),
        };
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/appointment/v1/validate-captcha", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得领事馆的详细数据
    pub async fn get_office_data(&mut self, input: &OfficeDataRequest)->Result<OfficeDataResponse>{
        let json = serde_json::to_value(input)?;
        let json_value = self.client.get("/api/console/office-data", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得领事馆日程
    pub async fn get_office_events(&mut self, input: &OfficeEventsRequest)->Result<OfficeEventsResponse> {
        let json = serde_json::to_value(input)?;
        let json_value = self.client.get("/api/console/office-events", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 获得领事馆指定日期的日程
    pub async fn get_office_day_events(&mut self, input: &OfficeDayEventsRequest)->Result<OfficeDayEventsResponse> {
        let json = serde_json::to_value(input)?;
        let json_value = self.client.get("/api/console/office-day-events", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 提交申请
    pub async fn save_appointment(&mut self, input: &SaveAppointmentRequest)->Result<SaveAppointmentResponse>{
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/appointment/v1/save-appointment", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 下载预约证明
    pub async fn generate_documents(&mut self, input: &GenerateDocumentsRequest)->Result<GenerateDocumentsResponse>{
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/appointment/v1/generate-documents", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 取消预约
    pub async fn cancel_appointment(&mut self, appointment: &Appointment)->Result<CancelAppointmentResponse>{
        let input = CancelAppointmentRequest{
            data: CancelAppointmentData{
                data: appointment.clone(),
            }
        };
        let json = serde_json::to_value(input)?;
        let json_value = self.client.post("/api/appointment/v1/cancel-appointment", &Option::from(json)).await?;
        serde_json::from_value(json_value).map_err(|err| anyhow!(err))
    }

    // 退出登录
    pub async fn logout(&mut self) -> Result<LogoutResponse>{
        let mut rr_no = 0;
        loop {
            match self.client.post("/api/appointment/v1/logout", &None).await {
                Ok(json_value) => {
                    if self.client.common_headers.contains_key("Authorization"){
                        self.client.common_headers.remove("Authorization");
                    }
                    return serde_json::from_value(json_value).map_err(|err| anyhow!(err))
                },
                Err(_) => {
                    if rr_no > 5 {
                        return Err(anyhow!("退出登录失败"));
                    }
                    rr_no += 1;
                    delay_min_max_secs(3,5).await;
                }
            }
        }
    }

    // 判断是否登录
    pub fn is_login(&self) -> bool {
        self.client.common_headers.contains_key("Authorization")
    }
}