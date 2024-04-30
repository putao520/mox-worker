use std::panic;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use base64::Engine;
use base64::engine::general_purpose;
use bytes::Bytes;
use chrono::{Datelike, Utc};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::Semaphore;

use crate::gsc::config::file_config::{FileConfig, load_file_config};
use crate::gsc::config::system_config::SystemConfig;
use crate::gsc::data_source::config_source::{get_share_config, set_share_config};
use crate::gsc::error::Error;
use crate::gsc::mdl::minio::load_s3;
use crate::gsc::phone_until::get_phone_info;
use crate::gsc::time_until::{age_from_birth_date, DateUntil, delay_max_ms, delay_max_secs, delay_min_max_secs, delay_ms, delay_secs, get_gird_month_range, get_next_day, timestamp_to_date, ymd_hms_to_timestamp, ymd_to_timestamp};
use crate::mox::account::{ban_account, get_valid_account, MoxAccount};
use crate::mox::birth::format_for_nut;
use crate::mox::gender::gender_to_string;
use crate::mox::helper::office_id_2_state_id;
use crate::mox::offices_assign::offices_assign;
use crate::mox::personal::{AppointmentInfo, Personal, PersonalService};
use crate::third::cloud_code::CloudCode;
use crate::third::ddd_code::DDDCode;
use crate::third::interface_identifying_captcha::IdentifyingCaptcha;
use crate::third::interface_ip_pool::IpPoolServices;
use crate::third::ipidea::IpIdea;
use crate::third::rox_labs::RoxLabs;
use crate::website::http_until::{HTTP_LANG, HTTP_PLATFORM, HTTP_USER_AGENT_HALF};
use crate::website::model::{ApmtPersonsAdditional, ApmtPersonsDataAddressEmergency, ApmtPersonsDataAddressHome, ApmtPersonsDocuments, ApmtPersonsSelectTmpFormalities, CatOfficeData, CountryData, Date, DateData, DayEvent, Event, EvidentiaryTramites, Interval, NewSchedule, OfficePreferences, OfficeProcedure, PersonsFormalities, Procedure, SaveDataPeople, StateData, SubtipoTramite, TipoTramite, Tramite, User};
use crate::website::mox::Mox;
use crate::website::proto::{AvailableProceduresResponse, ConfiguredOfficesRequest, GenerateDocumentsRequest, LoginRequest, OfficeDayEventsRequest, OfficeEventsRequest, SaveAppointmentRequest, SaveDataRequest, SearchNudRequest, ServiceGetDocumentEvidentiaryRequest, ServiceNoCurpSuetRequest};

/**
 该模块负责完成预约相关的业务逻辑
    思路:
        1: 尝试按正常流程去预约,但是目标国家,省份和领事馆的数据是固定的,来自账号信息
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProcessToken {
    step_token: String,
    trakingId: String,
    diff: u32,
}

pub struct MoxAppointment<C: IdentifyingCaptcha> {
    pub account: MoxAccount,
    pub mox: Mox,
    pub config: SystemConfig,
    pub local: FileConfig,
    user: Option<User>,
    appointment_id: Option<String>,
    identifying_captcha: C,
}

impl<C> MoxAppointment<C>
where
    C: IdentifyingCaptcha,
{
    pub async fn new<P: IpPoolServices>(account: &MoxAccount, config: &SystemConfig, local: &FileConfig, captcha: C, pool: Option<P>) -> Self {
        let mox = Mox::new(account.clone(), config.mox_client.clone(), pool, &config.mox_client.offices_console_keys).await;
        MoxAppointment {
            account: account.clone(),
            mox,
            config: config.clone(),
            local: local.clone(),
            user: None,
            appointment_id: None,
            identifying_captcha: captcha,
        }
    }

    async fn login(&mut self) -> Result<bool> {
        // #[cfg(debug_assertions)]
        // info!("申请登录验证码: {}", self.account.email);
        loop {
            let captcha_res = self.mox.get_captcha(self.account.email.as_str()).await?;
            let captcha = self.identifying_captcha.identifying(captcha_res.img.as_str()).await?;
            if captcha.is_empty() {
                error!("验证码识别失败(为空): {} ", self.account.email);
                return Err(anyhow!(Error{no:0x30001, msg: "验证码识别失败".to_string() }))
            }

            // #[cfg(debug_assertions)]
            // info!("登录->验证码识别成功: {} -> {}", self.account.email, captcha);

            let input = LoginRequest{
                email: self.account.email.clone(),
                password: self.account.password.clone(),
                location: "ext".to_string(),
                broser: HTTP_USER_AGENT_HALF.to_string(),
                platform: HTTP_PLATFORM.to_string(),
                lang: HTTP_LANG.to_string(),
                atuh_login: true,
                captcha,
            };
            let login_res = self.mox.login(&input).await?;
            if login_res.is_some() {
                let login_res = login_res.unwrap();
                self.user = Some(login_res.user);
                break;
            } else {
                // 验证码错误
                delay_max_ms(5).await;
            }
        }
        Ok(true)
    }

    async fn logout(&mut self) -> Result<bool> {
        if !self.mox.is_login() {
            return Ok(true)
        }
        let res = self.mox.logout().await?;
        Ok(res.authenticated)
    }

    // 判断账号是否正常
    async fn check_account(&mut self) -> Result<bool> {
        let appointment_res = self.mox.get_appointment().await?;
        if !appointment_res.success {
            return Err(anyhow!(Error{no:0x30003, msg: "给定的账号获取预约数据失败".to_string() }))
        }
        if appointment_res.data.len() >= self.config.account.max as usize {
            // return Err(anyhow!(Error{no:0x30004, msg: "给定的账号预约量已满".to_string() }))
            return Ok(false);
        }
        Ok(true)
    }

    fn get_available_procedures(&mut self, personal: &Personal, general_res: &AvailableProceduresResponse) -> Result<Vec<Procedure>> {
        let mut valid_procedures: Vec<Procedure> = Vec::new();
        for x in &general_res.availableProcedures {
            for p in x {
                //判断产品类型
                if p.cat_procedure_id != personal.visa_center_details.procedure_id {
                    continue;
                }
                if p.cat_procedure_type_id.is_none() || p.cat_procedure_subtype_id.is_none() {
                    continue;
                }
                if p.cat_procedure_type_id.unwrap() != personal.visa_center_details.cat_id {
                    continue;
                }
                // if p.cat_procedure_type_id? == 11 {
                //     // 如果是 INM 业务,原则上需要确认 NUT 是否有效
                //     error!("不支持工作商务签=> {} -> {}", p.cat_procedure_type_id?, personal.visa_center_details.cat_id);
                //     panic!("签证类型不匹配");
                // }
                if p.cat_procedure_subtype_id.unwrap() != personal.visa_center_details.sub_id {
                    continue;
                }
                // 判断日期是否在范围内
                let current_timestamp = ymd_to_timestamp(p.date.as_str());
                if current_timestamp > personal.appointment_start && current_timestamp < personal.appointment_end {
                    valid_procedures.push(p.clone());
                }
            }
        }
        Ok(valid_procedures)
    }

    // 判断 nut 与 passport 和 个人其他信息 是否匹配
    async fn try_check_nut(&mut self, personal: &Personal) -> Result<bool> {
        if personal.visa_center_details.cat_id != 11 {
            return Ok(true)
        }
        let nut = personal.nut.as_ref().ok_or(anyhow!(Error{no:0x30012, msg: "NUT 为空".to_string() }))?;
        let passport = personal.passport.as_ref().ok_or(anyhow!(Error{no:0x30013, msg: "护照号为空".to_string() }))?;
        let input = SearchNudRequest {
            nut: nut.clone(),
            passport: passport.clone(),
            nombres: personal.name.clone(),
            apellidos: personal.first_name.clone(),
            sexo: gender_to_string(personal.gender),
            fechaNacimiento: format_for_nut(personal.birth_date.as_str()) ,
        };
        let r = self.mox.search_nut(&input).await?;
        Ok(r.status && r.validate)
    }


    // 判断目标领事馆是否正常
    async fn check_office(&mut self) -> Result<AvailableProceduresResponse> {
        let input: ConfiguredOfficesRequest = ConfiguredOfficesRequest{
            jurisdictionId: 1,
            system_id: 1,
            api_key: self.config.mox_client.offices_console_keys.clone(),
        };
        let office_res = self.mox.get_configured_offices(&input).await?;
        if !office_res.success {
            return Err(anyhow!(Error{no:0x30005, msg: "获取领事馆数据失败".to_string() }))
        }
        // 判断目标领事馆是否可用
        let office = office_res.offices.iter().find(|&x| x.id == self.account.mox_endpoint.office_id);
        if office.is_none() {
            return Err(anyhow!(Error{no:0x30006, msg: "给定的领事馆不存在".to_string() }))
        }
        // 判断目标领事馆在指定范围是否包含目标业务
        let office = office.unwrap();
        let general_res = self.mox.get_general(office.id).await?;
        if !general_res.success {
            return Err(anyhow!(Error{no:0x30007, msg: "获取领事馆业务数据失败".to_string() }))
        }
        Ok(general_res)
    }

    // 获得领事办公室偏好
    async fn get_office_preferences(&mut self) -> Result<OfficePreferences> {
        let res = self.mox.get_office_preferences(self.account.mox_endpoint.office_id).await?;
        if !res.success {
            return Err(anyhow!(Error{no:0x30011, msg: "获取领事馆偏好数据失败".to_string() }))
        }
        Ok(res.office_preferences)
    }

    // 获得可用的服务
    async fn load_available_services(&mut self, personal: &Personal, available_procedures_response: &AvailableProceduresResponse) -> Result<Vec<Tramite>> {
        let input: ServiceNoCurpSuetRequest = ServiceNoCurpSuetRequest{
            p_nombre: personal.name.clone(),
            p_ap_paterno: personal.first_name.clone(),
            p_ap_materno: personal.last_name.to_string(),
            p_fec_nacimiento: personal.birth_date.clone(), // "YYYY-MM-DD"
            p_id_oficina: self.account.mox_endpoint.office_id,  // 223
            p_id_entidad_nacimiento: personal.state_id,    // 3372
            p_id_pais_nacimiento: personal.country_id,    // 44
        };
        let res = self.mox.service_no_curp_suet(&input).await?;
        if !res.success {
            return Err(anyhow!(Error{no:0x30009, msg: "获取领事馆服务数据失败".to_string() }))
        }
        if res.lResult.is_empty() {
            return Err(anyhow!(Error{no:0x30010, msg: "领事馆服务数据为空".to_string() }))
        }
        let mut tramites: Vec<Tramite> = Vec::new();

        for layer1 in &available_procedures_response.availableProcedures {
            for p in layer1 {
                for item in &res.lResult {
                    for v in &item.tramites.value {
                        if p.cat_procedure_id != v.procedure_id {
                            continue;
                        }
                        if p.cat_procedure_type_id.is_some() && v.procedure_type_id.is_some() && p.cat_procedure_type_id.unwrap() != v.procedure_type_id.unwrap() {
                            continue;
                        }
                        if p.cat_procedure_subtype_id.is_some() && v.procedure_subtype_id.is_some() && p.cat_procedure_subtype_id.unwrap() != v.procedure_subtype_id.unwrap() {
                            continue;
                        }
                        tramites.push(v.clone());
                    }
                }
            }
        }

        if tramites.is_empty() {
            return Err(anyhow!(Error{no:0x30011, msg: "没有可预约产品类型".to_string() }))
        }

        Ok(tramites)
    }

    fn build_person_data(&mut self, personal: &Personal, available_services: &Vec<Tramite>)-> Vec<SaveDataPeople> {
        let mut people: Vec<SaveDataPeople> = Vec::new();
        let p = SaveDataPeople {
            id: None,
            curp: "".to_string(),
            fullName: format!("{} {} {}", personal.name, personal.first_name, personal.last_name),
            name: personal.name.clone(),
            firstName: personal.first_name.clone(),
            lastName: personal.last_name.clone(),
            birthdate: personal.birth_date.clone(),
            age: age_from_birth_date(personal.birth_date.as_str()) as u32,
            statusCurp: None,
            docProbatorio: None,
            isValidateCurp: None,
            additional_person: false,
            naturalized: None,
            disability: None,
            civilState: personal.marital_status,
            firstNameMarried: "".to_string(),
            adoption: None,
            cat_gender_id: personal.gender,
            cat_nationality_id: personal.country_id,
            created_by_id: None,
            cat_apmt_type_person_id: None,
            apmt_persons_suet_formalities_status: false,
            showForm: true,
            apmt_persons_tmp_renapo_search_curp: false,
            country_id: personal.country_id,
            state_id: personal.state_id,
            municipality_id: None,
            locality_id: None,
            colony_id: None,
            location: None,
            postalCode: None,
            street: None,
            outdoorNumber: None,
            interiorNumber: None,
            passportNummber: None,
            email: self.account.email.clone(),
            phone: personal.phone.clone(),
            cell_phone: None,
            annotations: None,
            step: None,
            aceptNotificacionPhone: false,
            persons_formalities: vec![],
            apmt_persons_suet_tmp_formalities: available_services.clone(),
            apmt_persons_data_address_home: ApmtPersonsDataAddressHome::new_empty(),
            apmt_persons_data_address_emergency: ApmtPersonsDataAddressEmergency::new_empty(),
            apmt_persons_additional: ApmtPersonsAdditional::new_empty(),
            apmt_persons_second_additional: ApmtPersonsAdditional::new_empty(),
            apmt_persons_documents: ApmtPersonsDocuments::new_empty(),
        };
        people.push(p);
        people
    }

    fn append_person_formalities(&mut self, people: &mut Value, target_service: &Tramite, personal: &Personal) ->Result<()>{
        let service = target_service.clone();
        // 构建 apmt_persons_select_tmp_formalities
        let v1: ApmtPersonsSelectTmpFormalities = ApmtPersonsSelectTmpFormalities{
            t_id_tramite: service.procedure_id.to_string(),
            t_cad_tramite: service.procedure_name.clone(),
            data: vec![
                TipoTramite {
                    t_id_tipo_tramite: service.procedure_type_id.clone().ok_or(
                        anyhow!(Error{no:0x30012, msg: "service.procedure_type_id 字段不存在".to_string() })
                    )?.to_string(),
                    t_cad_tipo_tramite: service.procedure_type_name.clone().ok_or(
                        anyhow!(Error{no:0x30013, msg: "service.procedure_type_name 字段不存在".to_string() })
                    )?,
                    data: vec![
                        SubtipoTramite {
                            t_id_subtipo_tramite: service.procedure_subtype_id.clone().ok_or(
                                anyhow!(Error{no:0x30014, msg: "service.procedure_subtype_id 字段不存在".to_string() })
                            )?.to_string(),
                            t_cad_subtipo_tramite: service.procedure_subtype_name.clone().ok_or(
                                anyhow!(Error{no:0x30015, msg: "service.procedure_subtype_name 字段不存在".to_string() })
                            )?,
                            data: vec![service.clone()],
                        }
                    ],
                }
            ],
        };
        let mut formalities = Vec::new();
        formalities.push(v1.clone());
        people["apmt_persons_select_tmp_formalities"] = json!(formalities);
        // people.apmt_persons_select_tmp_formalities.push(v1.clone());
        // 构建 persons_formalities
        let v2: PersonsFormalities = PersonsFormalities{
            id: None,
            formalitites_id: service.procedure_id.to_string(),
            formalitites_name: service.procedure_name.clone(),
            formalitites_type_id: service.procedure_type_id.clone().ok_or(
                anyhow!(Error{no:0x30016, msg: "service.procedure_type_id 字段不存在".to_string() })
            )?.to_string(),
            formalitites_type_name: service.procedure_type_name.clone().ok_or(
                anyhow!(Error{no:0x30017, msg: "service.procedure_type_name 字段不存在".to_string() })
            )?,
            formalitites_subtype_id: service.procedure_subtype_id.clone().map(|x| x.to_string()),
            formalitites_subtype_name: service.procedure_subtype_name.clone(),
            passportNumber: personal.passport.clone(),
            nud: personal.nut.clone(),
            validity_id: None,
            validity_name: None,
            discount_id: None,
            discount_name: None,
            discount_formalitites_id: None,
            discount_formalitites_name: None,
            amount: None,
            coin_name: None,
            coin_id: None,
            document_formalitites_id: None,
            document_formalitites_name: None,
            temp_data: Option::from(v1),
            // id_tramite: service.procedure_type_id.clone()?.to_string(), // 新增
        };
        let mut persons_formalities = Vec::new();
        persons_formalities.push(v2);
        people["persons_formalities"] = json!(persons_formalities);
        // people.persons_formalities.push(v2);
        Ok(())
    }

    fn append_tramite_id(&mut self, people: &mut Value, id_tramite: String) {
        let arr = people["persons_formalities"].as_array_mut();
        if let Some(arr) = arr {
            for v in arr {
                v["id_tramite"] = json!(id_tramite);
            }
        }
    }

    fn fix_apmt_persons_second_additional(&mut self, people: &mut Value) {
        let obj = people["apmt_persons_second_additional"].as_object_mut();
        if let Some(p) = obj {
            p.remove("id");
            p.remove("curp");
            p.remove("parentesco_id");
            p.remove("parentesco_name");
            p.insert("name".to_string(), Value::Null);
            p.insert("firstName".to_string(), Value::Null);
            p.insert("lastName".to_string(), Value::Null);
        }
    }

    // 重复申请服务检测
    async fn check_repeat_apply(&mut self, people: &Value, target_server: &Tramite) -> Result<bool> {
        let mut et: Vec<EvidentiaryTramites> = Vec::new();

        let id_tramite = target_server.procedure_id.to_string();
        let id_tipo_tramite = target_server.procedure_type_id.unwrap().to_string();
        let id_subtipo_tramite = target_server.procedure_subtype_id.unwrap().to_string();
        et.push(EvidentiaryTramites{
            id_tramite,
            id_tipo_tramite,
            id_subtipo_tramite,
            id_rol: 1,
        });

        let input: ServiceGetDocumentEvidentiaryRequest = ServiceGetDocumentEvidentiaryRequest {
            p_tramites: serde_json::to_string(&et)?,
            p_edad: people["age"].as_u64().ok_or(
                anyhow!(Error{no:0x30017, msg: "people.age 字段不存在".to_string() })
            )? as u32,
            p_id_nacionalidad: people["cat_nationality_id"].as_u64().ok_or(
                anyhow!(Error{no:0x30017, msg: "people.cat_nationality_id 字段不存在".to_string() })
            )? as u32,
            // p_bol_naturalizado: people["naturalized"].as_bool(),
            // p_bol_discapacidad: people["disability"].as_bool(),
            p_bol_naturalizado: false,
            p_bol_discapacidad: false,
            p_bol_asistencia: false,
            p_id_edo_civil: people["civilState"].as_u64().ok_or(
                anyhow!(Error{no:0x30020, msg: "people.civilState 字段不存在".to_string() })
            )? as u32,
            p_bol_apellido_conyuge: false,
            p_id_oficina: self.account.mox_endpoint.office_id,
        };
        // if let Value::Object(ref mut map) = people {
        //     map.remove("id_tramite");
        // }
        let res = self.mox.service_get_document_evidentiary(&input).await?;
        if !res.success {
            return Err(anyhow!(Error{no:0x30017, msg: "获取重复申请服务数据失败".to_string() }))
        }
        Ok(res.lResult.is_empty())
    }

    async fn safe_captcha_valid(&mut self) -> Result<String>{
        let email = self.account.email.as_str();
        let mut captcha: String;
        loop{
            let captcha_res = self.mox.get_captcha(email).await?;
            captcha = self.identifying_captcha.identifying(captcha_res.img.as_str()).await?;
            // info!("最终申请->验证码识别成功: {} -> {}", self.account.email, captcha);
            let validate_captcha_res = self.mox.validate_captcha(captcha.as_str()).await?;
            if validate_captcha_res.status {
                // return Err(anyhow!(Error{no:30018, msg: "验证码验证失败".to_string() }))
                break;
            }
            info!("验证码验证失败[{}]: {}", email, captcha);
        }
        Ok(captcha)
    }

    async fn do_event(&mut self, event_block: &DayEvent, full_data: &mut Value, date_string: &String) -> Result<Option<DayEvent>>{
        if event_block.availables_by_block > 0 && event_block.total_by_block > 0 {
            // 填充 newSchedule
            let full_date = format!("{}T{}", date_string, event_block.initialTime);
            let new_schedule: NewSchedule = NewSchedule{
                id: event_block.hash_id,
                mainProcedureSelected: "".to_string(),
                newDateSelected: date_string.clone(),
                newTimeSelected: event_block.initialTime.clone(),
                newEndTimeSelected: event_block.endTime.clone(),
                fullDate: full_date,
            };
            full_data["newSchedule"] = json!(new_schedule);
            let final_save_res = self.mox.save_data(full_data).await?;
            if final_save_res.success {
                return Ok( Some(event_block.clone()) )
            }
            // 不成功就换个时间
            delay_min_max_secs(1, self.config.mox_client.time_period).await;
        }
        Ok(None)
    }

    async fn final_time_range(&mut self, office_events: &OfficeEventsRequest, date_string: &String, procces_id: &String, full_data: &mut Value) ->Result<Option<DayEvent>> {
        let captcha = self.safe_captcha_valid().await?;
        // 说的目标日期的所有时间
        let input: OfficeDayEventsRequest = OfficeDayEventsRequest {
            selectedDate: date_string.clone(),
            dateData: office_events.dateData.clone(),
            cat_system_id: 1,
            procces_id: procces_id.clone(),
            captcha_value: captcha,
            usr: office_events.usr.clone(),
            api_key: office_events.api_key.clone(),
        };
        let office_day_events = self.mox.get_office_day_events(&input).await?;
        if !office_day_events.success {
            return Err(anyhow!(Error{no:0x30019, msg: "获取目标日期的所有时间失败".to_string() }))
        }
        // 数组返回events
        if office_day_events.events.is_array() {
            if let Some(events) = office_day_events.events.as_array() {
                for event in events {
                    let event_block: DayEvent = serde_json::from_value(event.clone())?;
                    if let Ok(r) = self.do_event(&event_block, full_data, date_string).await {
                        if r.is_some() {
                            return Ok(r)
                        }
                    }
                }
            }
        }
        // map 返回
        if office_day_events.events.is_object() {
            if let Some(events) = office_day_events.events.as_object() {
                for (_, v) in events {
                    let event_block: DayEvent = serde_json::from_value(v.clone())?;
                    if let Ok(r) = self.do_event(&event_block, full_data, date_string).await {
                        if r.is_some() {
                            return Ok(r)
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    fn get_country(&mut self, country_id: u32) -> Result<CountryData> {
        let countries = self.mox.get_countries();
        let r = countries.iter().find(|&x| x.id_pais == country_id).ok_or(
            anyhow!(Error{no:0x30012, msg: "查找国家数据失败".to_string() })
        )?.clone();
        Ok(r)
    }

    async fn get_provinces(&mut self, country_id: u32, state_id: u32) ->Result<StateData> {
        let state_res = self.mox.get_provinces(country_id).await?;
        if !state_res.success {
            return Err(anyhow!(Error{no:0x30013, msg: "获取省份数据失败".to_string() }))
        }
        let r= state_res.data.states.iter().find(|&x| x.var_id_entidad == state_id);
        if r.is_none() {
            return Err(anyhow!(Error{no:0x31013, msg: "查找省份数据失败".to_string() }))
        }
        Ok(r.unwrap().clone())
    }

    async fn get_offices_by_provinces(&mut self, country_id: u32, state_id: u32, office_id: u32) ->Result<CatOfficeData> {
        let office_res = self.mox.get_offices_by_provinces(country_id, state_id).await?;
        if !office_res.success {
            return Err(anyhow!(Error{no:0x30014, msg: "获取办公室数据失败".to_string() }))
        }
        let r = office_res.data.offices.iter().find(|&x| x.office_id == office_id);
        if r.is_none() {
            return Err(anyhow!(Error{no:0x31014, msg: "查找办公室数据失败".to_string() }))
        }
        Ok(r.unwrap().clone())
    }

    async fn save_data_init(&mut self, data: &Value)->Result<String> {
        // 第一次提交预约数据
        let save_res = self.mox.save_data_init(data).await?;
        if !save_res.success {
            return Err(anyhow!(Error{no:0x30015, msg: "初始化预约请求失败".to_string() }))
        }
        // 获得预约请求的 id
        Ok(save_res.id.ok_or(
            anyhow!(Error{no:0x30016, msg: "获取预约请求 id 失败".to_string() })
        )?)
    }

    async fn get_process(&mut self, appointment_id: &String) ->Result<(ProcessToken, String)> {
        let process_res = self.mox.get_process(appointment_id).await?;
        if !process_res.success {
            return Err(anyhow!(Error{no:0x30016, msg: "获取预约请求前置数据失败".to_string() }))
        }
        Ok((ProcessToken {
            step_token: process_res.data.step_token,
            trakingId: process_res.data.trakingId,
            diff: process_res.segundos,
        }, process_res.data.folioProcedureInitial))
    }

    async fn save_data(&mut self, full_data: &mut Value, step: u8) ->Result<()> {
        full_data["step"] = json!(step);
        let save_res = self.mox.save_data(full_data).await?;
        if !save_res.success {
            return Err(anyhow!(Error{no:0x30015, msg: "预约请求失败".to_string() }))
        }
        Ok(())
    }

    async fn get_captcha(&mut self) ->Result<String> {
        let email = self.account.email.as_str();
        let res = self.mox.get_captcha(email).await?;
        let captcha = self.identifying_captcha.identifying(res.img.as_str()).await?;
        Ok(captcha)
    }

    async fn get_office_events(&mut self, input: &OfficeEventsRequest) ->Result<Vec<Event>> {
        let res = self.mox.get_office_events(input).await?;
        if !res.success {
            return Err(anyhow!(Error{no:0x30012, msg: "获取领事馆事件数据失败".to_string() }))
        }
        Ok(res.events)
    }

    async fn generate_document(&mut self, input: &GenerateDocumentsRequest) ->Result<String> {
        let file_id = input.folio.clone();
        let res = self.mox.generate_documents(input).await?;
        if !res.success {
            return Err(anyhow!(Error{no:0x30020, msg: "生成预约凭证失败".to_string() }))
        }
        let content = general_purpose::STANDARD.decode(res.file)?;
        // 生成文件名 预约日期+预约时间+预约凭证号
        let file_name = format!("{}.pdf", file_id);
        loop {
            let s3 = load_s3(&self.config).await?;
            let s3_res = s3.put_object(file_name.clone(), Bytes::from(content.clone()) ).await;
            if s3_res.is_ok() {
                info!("上传预约凭证成功: {}", file_name);
                break;
            }
            delay_min_max_secs(1,3).await;
            warn!("上传预约凭证失败: {}", file_name);
        }
        info!("预约成功 {}", file_id);
        Ok(file_id)
    }

    async fn save_appointment(&mut self, input: &SaveAppointmentRequest)->Result<String> {
        let save_appointment_res = self.mox.save_appointment(input).await?;
        if !save_appointment_res.success {
            return Err(anyhow!(Error{no:0x30022, msg: "提交预约申请失败".to_string() }))
        }
        // 下载预约凭证
        let input: GenerateDocumentsRequest = GenerateDocumentsRequest{
            folio: save_appointment_res.ticket.clone(),
            form: input.form.clone(),
        };
        let file_id = self.generate_document(&input).await?;
        Ok(file_id)
    }

    // 找到合适的可预约日期
    async fn find_available_date(&mut self, personal: &Personal, office_id: u32, traking_id: &String, target_service: &Tramite, full_data: &mut Value) ->Result<Option<DayEvent>> {
        // 预设值 full_data
        full_data["GrecaptchaResponse"] = Value::Null;
        full_data["program"] = json!(true);
        full_data["awaitStep"] = json!(false);

        // 构造需要的 procedures
        let mut office_procedures: Vec<OfficeProcedure> = Vec::new();
        office_procedures.push(OfficeProcedure {
            cat_procedure_id: target_service.procedure_id.to_string(),
            cat_procedure_type_id: target_service.procedure_type_id.unwrap().to_string(),
            cat_procedure_subtype_id: target_service.procedure_subtype_id.unwrap().to_string(),
            cat_procedure_name: target_service.procedure_name.clone(),
            cat_procedure_type_name: target_service.procedure_type_name.clone().unwrap(),
            cat_procedure_subtype_name: target_service.procedure_subtype_name.clone().unwrap(),
        });

        // 获得领事馆事件
        let mut office_event_request: OfficeEventsRequest = OfficeEventsRequest {
            currentView: "dayGridMonth".to_string(),
            interval: Interval {
                startDate: "".to_string(),
                endDate: "".to_string(),
            },
            dateData: DateData {
                cat_office_id: office_id,
                pcm_event_id: None,
                amountFormaliti: 1, // 一次只能申请一个
                procedures: office_procedures,
            },
            procces_id: traking_id.clone(),
            cat_system_id: 1,
            captcha_value: "".to_string(),
            usr: self.user.clone().ok_or(
                anyhow!(Error{no:0x30023, msg: "user 字段不存在".to_string() })
            )?.hash,
            api_key: self.config.mox_client.offices_console_keys.clone(),
        };

        // 根据用户的预约时间范围,找到合适的可预约日期
        // 计算月份范围
        let current_timestamp = Utc::now().timestamp();
        let start_timestamp = if current_timestamp > personal.appointment_start {
            current_timestamp + 86400
        } else {
            personal.appointment_start
        };
        let personal_start_date = timestamp_to_date(start_timestamp);
        let personal_end_date = timestamp_to_date(personal.appointment_end);
        let end_timestamp = personal.appointment_end + 86400;
        let mut date_helper = DateUntil::from(personal_start_date.year(), personal_start_date.month(), personal_start_date.day());
        loop {
            let (start_date, end_date) = get_gird_month_range(date_helper.year, date_helper.month);
            // 验证码
            let captcha = self.safe_captcha_valid().await?;
            office_event_request.interval.startDate = start_date.format("%Y-%m-%d").to_string();
            office_event_request.interval.endDate = end_date.format("%Y-%m-%d").to_string();
            office_event_request.captcha_value = captcha;
            let office_events: Vec<Event> = self.get_office_events(&office_event_request).await?;
            if !office_events.is_empty() {
                for event in &office_events {
                    if event.availables_by_day > 0 && event.total_by_day > 0 {
                        let event_timestamp = ymd_to_timestamp(event.date.as_str());
                        if event_timestamp >= start_timestamp && event_timestamp < end_timestamp {
                            let final_res = self.final_time_range(&office_event_request, &event.date, traking_id, full_data).await?;
                            if final_res.is_some() {
                                return Ok(final_res)
                            }
                        }
                    }
                    delay_min_max_secs(1, self.config.mox_client.date_period).await;
                }
            }
            // 月份漫步直到截止月份
            if date_helper.year >= personal_end_date.year()  && date_helper.month >= personal_end_date.month()  {
                break;
            }
            date_helper.next_month();
        }
        Err(anyhow!(Error{no:0x30024, msg: "找不到合适的可预约日期".to_string() }))
    }

    // 返回预约凭证的 minio id
    async fn appointment(&mut self, personal: &Personal, available_procedures_response: &AvailableProceduresResponse, office_preferences: &OfficePreferences, available_services: &Vec<Tramite>) ->Result<AppointmentInfo> {
        let account = self.account.clone();
        // 获得国家数据
        let country_id = account.mox_endpoint.country_id;
        let country_data = self.get_country(country_id)?;
        // 获得省份数据
        let state_id = account.mox_endpoint.state_id;
        let state_data = self.get_provinces(country_id, state_id).await?;
        // 获得办公室数据
        let office_id = account.mox_endpoint.office_id;
        let cat_office_data = self.get_offices_by_provinces(country_id, state_id, office_id).await?;
        // 获得可用的业务数据
        // let available_procedures = self.get_available_procedures(personal, available_procedures_response);
        // 获得申请人数据
        let persons = self.build_person_data(personal, available_services);
        // 构建预约数据
        let data: SaveDataRequest = SaveDataRequest{
            id: None,
            origin: 1,
            typeAppointment: 1,
            awaitStep: true,
            folioProcedureInitial: None,
            country_id,
            country_data,
            state_id,
            state_data,
            cat_office_id: office_id,
            cat_office_data,
            folioAppointment: None,
            dateAppointment: None,
            hourStarAppointment: None,
            hourEndAppointment: None,
            created_by_id: None,
            cat_apmt_type_appointment_id: None,
            office_selected: true,
            people: persons,
            officeConfigData: office_preferences.clone(),
            setTempFormalitiesConsole: available_procedures_response.clone(),
            step: 1,
        };

        let mut form = serde_json::to_value(data)?;

        // 第一次提交预约数据
        let appointment_id = self.save_data_init(&form).await?;
        // 保存 appointment_id
        self.appointment_id = Some(appointment_id.clone());
        // 获得第二次提交前置数据
        let (process_token, folio_procedure_initial) = self.get_process(&appointment_id).await?;
        update_save_date(&mut form, process_token);

        let target_service = get_target_service(personal, available_services)?;

        form["folioProcedureInitial"] = json!(folio_procedure_initial);
        match form["people"].as_array_mut() {
            Some(p) => {
                for people in p {
                    let _ = self.append_person_formalities(people, &target_service, personal);
                }
            },
            None => {
                return Err(anyhow!(Error{no:0x30018, msg: "人员数据异常".to_string() }))
            }
        }

        delay_min_max_secs(3,6).await;
        self.save_data(&mut form, 2).await?;
        // 获得第三次提交前置数据
        let (process_token, _) = self.get_process(&appointment_id).await?;

        // let available_procedures = available_procedures_response.availableProcedures.get(0).unwrap();

        // ---> 插入检测是否重复申请了服务
        match form["people"].as_array_mut() {
            Some(p) => {
                for people in p {
                    let flag = self.check_repeat_apply(people, &target_service).await?;
                    if !flag {
                        return Err(anyhow!(Error{no:0x30018, msg: "重复申请了服务".to_string() }))
                    }
                    self.append_tramite_id(people, target_service.procedure_type_id.ok_or(
                        anyhow!(Error{no:0x30019, msg: "target_service.procedure_type_id 字段不存在".to_string() })
                    )?.to_string());
                    self.fix_apmt_persons_second_additional(people);
                    people["adoption"] = json!(false);
                    people["disability"] = json!(false);
                    people["naturalized"] = json!(false);
                    people["apmt_persons_documents"]["modelo_rubros_dinamico_doc_acta_extemporanea"] = json!({});
                    people["apmt_persons_documents"]["modelo_rubros_dinamico_doc_complementario"] = json!({});
                    people["apmt_persons_documents"]["modelo_rubros_dinamico_doc_nacionalidad"] = json!({});
                    people["apmt_persons_documents"]["modelo_rubros_dinamico_doc_probatorio"] = json!({});
                    people["apmt_persons_second_additional"]["modelo_rubros_dinamico_doc_acta_extemporanea"] = json!({});
                    people["apmt_persons_second_additional"]["modelo_rubros_dinamico_doc_complementario"] = json!({});
                    people["apmt_persons_second_additional"]["modelo_rubros_dinamico_doc_nacionalidad"] = json!({});
                    people["apmt_persons_second_additional"]["modelo_rubros_dinamico_doc_probatorio"] = json!({});
                }
            },
            None => {
                return Err(anyhow!(Error{no:0x30018, msg: "人员数据异常".to_string() }))
            }
        }
        update_save_date(&mut form, process_token);
        delay_min_max_secs(0,2).await;
        self.save_data(&mut form, 3).await?;
        // 获得第四次提交前置数据
        let (process_token, _) = self.get_process(&appointment_id).await?;
        update_save_date(&mut form, process_token);
        match form["people"].as_array_mut() {
            Some(p) => {
                for people in p {
                    people["apmt_persons_data_address_emergency"]["name"] = json!(personal.emergency_contact.name);
                    people["apmt_persons_data_address_emergency"]["firstName"] = json!(personal.emergency_contact.first_name);
                    people["apmt_persons_data_address_emergency"]["lastName"] = json!(personal.emergency_contact.last_name);
                    people["apmt_persons_data_address_emergency"]["cellPhoneFormatInternational"] = json!(personal.emergency_contact.phone_number);
                    people["apmt_persons_data_address_emergency"]["sameDirection"] = json!(true);
                    let phone_info = get_phone_info(personal.emergency_contact.phone_number.as_str());
                    people["apmt_persons_data_address_emergency"]["phone"] = json!(phone_info.phone_number);

                    people["apmt_persons_data_address_home"]["country_id"] = json!(personal.country_id);
                    people["apmt_persons_data_address_home"]["direction"] = json!(personal.city_address);
                    people["apmt_persons_data_address_home"]["state_id"] = json!(personal.state_id);
                }
            }
            None => {
                return Err(anyhow!(Error{no:0x30018, msg: "人员数据异常".to_string() }))
            }
        }
        // form["apmt_persons_documents"]["modelo_rubros_dinamico_doc_acta_extemporanea"] = json!([]);
        // form["apmt_persons_documents"]["modelo_rubros_dinamico_doc_complementario"] = json!([]);
        // form["apmt_persons_documents"]["modelo_rubros_dinamico_doc_nacionalidad"] = json!([]);
        // form["apmt_persons_documents"]["modelo_rubros_dinamico_doc_probatorio"] = json!([]);
        // form["apmt_persons_second_additional"]["modelo_rubros_dinamico_doc_acta_extemporanea"] = json!([]);
        // form["apmt_persons_second_additional"]["modelo_rubros_dinamico_doc_complementario"] = json!([]);
        // form["apmt_persons_second_additional"]["modelo_rubros_dinamico_doc_nacionalidad"] = json!([]);
        // form["apmt_persons_second_additional"]["modelo_rubros_dinamico_doc_probatorio"] = json!([]);
        delay_min_max_secs(50,80).await;
        self.save_data(&mut form, 4).await?;

        // 获得第五次提交前置数据
        let (process_token, _) = self.get_process(&appointment_id).await?;
        form["step_token"] = json!(process_token.step_token);
        form["diffMin"] = json!(process_token.diff);

        // ---> 插入安全验证和实际详细数据
        let final_flag = self.find_available_date(personal, office_id, &process_token.trakingId, &target_service, &mut form).await?;
        // form = r;
        let day_event = final_flag.unwrap();
        // 提交预约申请
        let end_str = format!("{}T{}", day_event.date, day_event.endTime);

        println!("EndTime {}", end_str.as_str());

        if let Some(r) = personal.role {
            // 是测试查询的用户
            if r == 1 {
                return Err(anyhow!(Error{no:0x30018, msg: format!("[信息]测试角色({})->中断申请", personal.phone).to_string() }))
            }
        }

        let input: SaveAppointmentRequest = SaveAppointmentRequest{
            date: Date{
                id: day_event.hash_id,
                start: format!("{}T{}", day_event.date, day_event.initialTime),
                end: end_str.clone(),
            },
            newSchedule: serde_json::from_value(form["newSchedule"].clone())?,
            form: form.clone(),
        };
        let ticket_id = self.save_appointment(&input).await?;
        Ok(AppointmentInfo{
            ticket_id,
            appointment_id,
            appointment_time: ymd_hms_to_timestamp(end_str.as_str(), "+08:00"),
        })
    }

    async fn start_appointment(&mut self, personal: &mut Personal) -> Result<AppointmentInfo> {
        // 验证账号数据
        let verify =  self.mox.verify_user_data().await?;
        if !verify.success {
            return Err(anyhow!(Error{no:0x30007, msg: "验证账号数据失败".to_string() }))
        }
        // 验证目标领事馆与业务(获得目标时间范围内可用的业务数据)
        let available_procedures_response = self.check_office().await?;
        if available_procedures_response.availableProcedures.is_empty() {
            return Err(anyhow!(Error{no:0x30008, msg: "目标领事馆没有可用的业务".to_string() }))
        }
        // 申请的业务是nut时,尝试验证nut
        let nut_status = self.try_check_nut(&personal).await?;
        if !nut_status {
            return Err(anyhow!(Error{no:0x20004, msg: "客户nut数据不匹配".to_string() }));
        }
        // 获得领事办公室偏好
        let office_preferences = self.get_office_preferences().await?;
        // 验证服务内容
        let available_services = self.load_available_services(&personal, &available_procedures_response).await?;
        // 实际预约流程
        let appointment_info = self.appointment(&personal, &available_procedures_response, &office_preferences, &available_services).await?;
        info!("预约成功: {}", appointment_info.ticket_id);
        Ok(appointment_info)
    }
    pub async fn start(&mut self) -> Result<Vec<String>> {
        let persons = Vec::new();
        let mut personal_service = PersonalService::new().await?;

        // 登录账号
        let login_res = self.login().await;
        if login_res.is_err() {
            let error_msg = login_res.err().unwrap().to_string();
            self.account.log_message = error_msg.clone();
            error!("登录异常: {} -> 原因:{}", self.account.email.clone(), self.account.log_message.clone());
            // 账号移动到不可用队列
            if !self.account.log_message.contains("error sending request for url") {
                error!("禁用账号: {}", self.account.email.clone());
                ban_account(&self.account).await?;
            }
            return Err(anyhow!(error_msg));
        }

        // 检查账号预约量是否
        loop {
            // 抽取一个客户
            let personal_res = personal_service.get_valid_personal().await;
            if let Err(e) = personal_res {
                error!("客户信息异常:{}", e);
                break;
            }
            let mut personal = personal_res?;
            info!("提取客户信息: {}", personal.phone.clone());

            let mut maybe_ticket_info: Option<AppointmentInfo> = None;
            if !self.local.task.disable_assign && self.local.test.is_none() {
                assign_account(&mut self.account, &personal);
            }
            if !self.check_account().await? {
                info!("账号预约量已用完");
                break;
            }
            let appointment_info_res = self.start_appointment(&mut personal).await;
            match appointment_info_res {
                Err(e) => {
                    // 关闭当前申请
                    self.close_appointment().await?;
                    error!("task-error:{}", e);
                    // 尝试序列化成 GscError
                    match e.downcast_ref::<Error>() {
                        Some(err) => {
                            error!("错误[{}]:{}", err.no, err.msg.clone());
                            // 是用户信息类错误
                            if err.no & 0x00020000 == 0x20000 {
                                personal.log_message = err.msg.clone();
                                personal_service.record_exception_personal().await?;
                            } else {
                                if err.no & 0x00010000 == 0x10000 {
                                    return Err(anyhow!("网络系统错误"))
                                }
                                // 是三方服务类错误
                                if err.no & 0x00040000 == 0x40000 {
                                    if err.no & 0x00041000 == 0x41000 {
                                        self.config.captcha.log_message = err.msg.clone();
                                    }
                                    if err.no & 0x00042000 == 0x42000 {
                                        self.config.proxy.log_message = err.msg.clone();
                                    }
                                    set_share_config(&self.config)?;
                                    return Err(anyhow!("三方服务错误"))
                                }
                            }
                        },
                        None => {
                            // todo 调试暂时修改
                            // 同一个用户尝试重新申请流程
                            return Err(e);
                            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                            // continue;
                        }
                    }
                }
                Ok(info) => {
                    maybe_ticket_info = Some(info);
                }
            }
            // 随机延迟
            delay_max_secs(self.config.account.appointment_period).await;
            if maybe_ticket_info.is_none() {
                warn!("预约失败: {}", personal.phone.clone());
                continue;
            }
            personal.appointment_info = maybe_ticket_info;
            // 记录成功的客户
            personal_service.record_success_personal().await?;
        }
        // 返回预约凭证组
        Ok(persons)
    }

    fn reset_appointment(&mut self) {
        self.appointment_id = None;
    }

    pub async fn close_appointment(&mut self) -> Result<()>{
        if let Some(appointment_id) = &self.appointment_id {
            self.mox.close_appointment(appointment_id.clone()).await?;
            self.appointment_id = None;
        };
        Ok(())
    }
}

fn get_target_service(personal: &Personal, available_services: &Vec<Tramite>) -> Result<Tramite> {
    for x in available_services {
        if x.procedure_id != personal.visa_center_details.procedure_id {
            continue
        }
        if x.procedure_type_id.is_none() || x.procedure_subtype_id.is_none() {
            continue
        }
        if x.procedure_type_id.unwrap() != personal.visa_center_details.cat_id {
            continue
        }
        if x.procedure_subtype_id.unwrap() != personal.visa_center_details.sub_id {
            continue
        }
        return Ok(x.clone())
    }
    Err(anyhow!(Error{no:0x30023, msg: "找不到目标业务".to_string() }))
}

fn get_target_procedure(personal: &Personal, available_procedures: &Vec<Procedure>) -> Result<Procedure> {
    for x in available_procedures {
        if x.cat_procedure_id != personal.visa_center_details.procedure_id {
            continue
        }
        if x.cat_procedure_type_id.is_none() || x.cat_procedure_type_id.is_none() {
            continue
        }
        if x.cat_procedure_type_id.unwrap() != personal.visa_center_details.cat_id {
            continue
        }
        if x.cat_procedure_subtype_id.unwrap() != personal.visa_center_details.sub_id {
            continue
        }
        return Ok(x.clone())
    }
    Err(anyhow!(Error{no:0x30023, msg: "找不到目标业务".to_string() }))
}

fn update_save_date(data: &mut Value, process_token: ProcessToken) {
    data["step_token"] = json!(process_token.step_token);
    data["trakingId"] = json!(process_token.trakingId);
    data["diffMin"] = json!(process_token.diff);
}

fn assign_account(account: &mut MoxAccount, personal: &Personal) {
    if let Some(office_id) = offices_assign(personal.state_id) {
        account.mox_endpoint.office_id = office_id;
        account.mox_endpoint.state_id = office_id_2_state_id(office_id);
        account.mox_endpoint.country_id = personal.country_id;
    }
}

pub async fn start_task(account: MoxAccount, share_config: SystemConfig, local_config: FileConfig) ->Result<()> {
    let captcha = DDDCode::new(&share_config.captcha);
    let mut app = if local_config.task.disable_proxy {
        MoxAppointment::new::<RoxLabs>(&account, &share_config, &local_config, captcha, None).await
    } else {
        let ip_pool = RoxLabs::new(&share_config.proxy);
        MoxAppointment::new(&account, &share_config, &local_config, captcha, Some(ip_pool)).await
    };

    let r = app.start().await;
    if r.is_err() {
        error!("任务执行失败: {}", r.err().unwrap());
    }
    app.logout().await?;
    Ok(())
}

pub async fn start_appointment_task() ->Result<()> {
    let local_config = load_file_config()?;
    let semaphore = Arc::new(Semaphore::new(local_config.task.max as usize));
    loop {
        let local = local_config.clone();
        let mut share_config = get_share_config()?;
        match get_valid_account(&share_config.account).await {
            Ok(account) => {
                #[cfg(debug_assertions)]
                info!("成功提取账号: {}", account.email.clone());
                let sem_clone = semaphore.clone();
                let _permit = sem_clone.acquire().await.expect("并发队列已满!");
                let sem_clone_inside = sem_clone.clone();
                tokio::spawn(async move {
                    let _permit = sem_clone_inside.acquire().await.expect("并发队列已满!_协程");
                    #[cfg(debug_assertions)]
                    info!("[debug]开始执行任务: {}", account.email.clone());
                    if let Err(e) = start_task(account, share_config, local).await {
                        error!("任务执行失败: {}", e);
                    }
                });
                delay_ms(local_config.task.interval).await;
            },
            Err(e) => {
                info!("账号提取异常 原因: {}", e.to_string());
                delay_secs(local.task.retry_interval).await;
            }
        }
        delay_ms(local_config.task.interval).await;
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::gsc::config::file_config::load_file_config;
    use crate::gsc::config::system_config::{AccountConfig, CaptchaConfig, MoxClientConfig, ProxyConfig, S3Config, SystemConfig};
    use crate::gsc::data_source::config_source::{get_share_config, set_share_config};
    use crate::gsc::debug::helpers::start_logging;
    use crate::mox::account::{MoxAccount, MoxEndpoint};
    use crate::mox::appointment::{MoxAppointment, start_task};
    use crate::mox::personal::{add_valid_personal, EmergencyContact, Personal, VisaCenterDetails};
    use crate::third::cloud_code::CloudCode;
    use crate::third::rox_labs::RoxLabs;

    fn init_test_config() {
        // 初始系统配置并写入
        let cfg = SystemConfig{
            account: AccountConfig{
                max: 5,
                appointment_period: 3,
                cool_down: 3600,
            },
            mox_client: MoxClientConfig {
                api_key: "VcAmGCISOnRc6AA".to_string(),
                offices_console_keys: "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk".to_string(),
                date_period: 5,
                time_period: 5,
            },
            s3_config: S3Config{
                region: "us-east-1".to_string(),
                endpoint: "localhost:9000".to_string(),
                access_key: "2IiYe6HtY8RpF8R2j6bi".to_string(),
                secret_key: "hv94WgGAsqTO6YCujEXsN6ZGovKy7OW1edzz6xCV".to_string(),
            },
            captcha: CaptchaConfig {
                url: "http://150.138.84.183:9896/ocr/b64".to_string(),
                token: "".to_string(),
                log_message: "".to_string()
            },
            proxy: ProxyConfig{
                host: "api.proxy.ipidea.io".to_string(),
                name: "".to_string(),
                password: "".to_string(),
                port: 80,
                log_message: "".to_string(),
            },
            log_message: "".to_string(),
        };
        set_share_config(&cfg).unwrap();
    }

    async fn login_impl(account: &MoxAccount) -> Result<MoxAppointment<CloudCode>> {
        let local_config = load_file_config()?;
        let share_config = get_share_config()?;

        // 初始化测试配置
        init_test_config();
        // 测试到登录
        let captcha = CloudCode::new(&share_config.captcha);
        // 测试代理
        let mut app = MoxAppointment::new::<RoxLabs>(account, &share_config, &local_config, captcha, None).await;
        app.login().await?;
        Ok(app)
    }

    #[tokio::test]
    async fn test_login() {
        // 在澳大利亚申请的测试
        let test_account = MoxAccount {
            email: "zhuy34694@gmail.com".to_string(),
            password: "Qi12012hai**".to_string(),
            log_message: "".to_string(),
            mox_endpoint: MoxEndpoint {
                country_id: 14,
                state_id: 3380,
                city_code: None,
                office_id: 223,
            },
            expire_time: 0,
        };

        let app_res = login_impl(&test_account).await;

        let mut cmp_str = "".to_string();
        if app_res.is_err() {
            // 判断 app_res 是否是 Err,如果是判断错误信息
            let e = app_res.err().unwrap();
            cmp_str = e.to_string();
            println!("{}", cmp_str);
        }

        // 验证登录
        assert_eq!("给定的账号被屏蔽", cmp_str);
    }



    #[tokio::test]
    async fn test_appointment() {
        init_test_config();
        start_logging();

        let account= init_test_account();
        init_test_personal().await.unwrap();
        let share_config = get_share_config().unwrap();
        let local_config = load_file_config().unwrap();

        start_task(account, share_config, local_config).await.unwrap();

    }

    async fn init_test_personal() -> Result<()>{
        let personal = Personal {
            name: "JUN".to_string(),
            first_name: "LV".to_string(),
            last_name: "".to_string(),
            gender: 2,
            birth_date: "1993-08-28".to_string(),
            marital_status: 1,
            phone: "+86 177 6895 4321".to_string(),
            log_message: "".to_string(),
            passport: None,
            nut: None,
            priority: 1,
            role: Some(1),
            state_id: 3649,
            country_id: 44,
            city_address: "-".to_string(),
            appointment_start: 1713542400,  // 2024-04-20 00:00:00
            appointment_end: 1718816410,    // 2024-06-20 00:00:00 + 1d
            emergency_contact: EmergencyContact {
                name: "LI".to_string(),
                first_name: "ZHANG".to_string(),
                last_name: "".to_string(),
                phone_number: "+86 152 1590 3725".to_string(),
            },
            visa_center_details: VisaCenterDetails{
                procedure_id: 31,
                cat_id: 10,
                sub_id: 17,
            },
            appointment_info: None,
        };

        add_valid_personal(&personal).await?;

        Ok(())
    }

    fn init_test_account() ->MoxAccount {
        // 在澳大利亚申请的测试
        let test_account = MoxAccount {
            email: "JurkowskiRichmann832@gmail.com".to_string().to_lowercase(),
            password: "Qwer123456**".to_string(),
            log_message: "".to_string(),
            mox_endpoint: MoxEndpoint {
                country_id: 13,     // 国家 13 澳大利亚 14 中国
                state_id: 3358,     // 州 3380 对应奥地利
                city_code: None,
                office_id: 74,     // 领事馆 223
            },
            expire_time: 0,
        };
        test_account
    }
}