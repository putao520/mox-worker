use reqwest::{Request, Response};
use ahash::AHashMap;
use anyhow::{Result, anyhow};
use log::{debug, info};
use crate::gsc::config::system_config::{MoxClientConfig};
use crate::gsc::error::Error;
use crate::gsc::time_until::delay_min_max_secs;
use crate::website::http_until::{HTTP_ORIGIN, HTTP_REFERER, HTTP_USER_AGENT};
use crate::website::param_crypto::AdvancedCrypt;

static MAX_RETRY: u32 = 3;

pub struct MoxClient {
    force_no_auth: bool,
    host: String,
    pub common_headers: AHashMap<String, String>,
    post_headers: AHashMap<String, String>,
    get_headers: AHashMap<String, String>,
    client: reqwest::Client,
    coder: AdvancedCrypt,
}

impl MoxClient {
    pub fn new(host: &str, cfg: MoxClientConfig, proxy: Option<reqwest::Proxy>) -> Self {
        let mut common_headers = AHashMap::new();
        common_headers.insert("User-Agent".to_string(), HTTP_USER_AGENT.to_string());
        common_headers.insert("Origin".to_string(), HTTP_ORIGIN.to_string());
        common_headers.insert("Referer".to_string(), HTTP_REFERER.to_string());
        common_headers.insert("Accept".to_string(), "application/json".to_string());
        common_headers.insert("Accept-Encoding".to_string(), "gzip, deflate, br, zstd".to_string());
        common_headers.insert("Accept-Language".to_string(), "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".to_string());
        common_headers.insert("X-Requested-With".to_string(), "XMLHttpRequest".to_string());
        common_headers.insert("Accept-C".to_string(), "true".to_string());
        let client = match proxy {
            Some(p) => reqwest::Client::builder().proxy(p).build().unwrap(),
            None => reqwest::Client::new(),
        };
        MoxClient {
            force_no_auth: false,
            host: host.to_string(),
            common_headers,
            post_headers: AHashMap::new(),
            get_headers: AHashMap::new(),
            client,
            coder: AdvancedCrypt::new( cfg.api_key.as_str() ),
        }
    }

    fn hash_map2header_map(&mut self, headers: &AHashMap<String, String>) -> reqwest::header::HeaderMap {
        let mut reqwest_headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
        for (k, v) in headers.iter() {
            if self.force_no_auth && k == "Authorization" {
                continue;
            } else {
                let key = reqwest::header::HeaderName::from_bytes(k.as_bytes()).unwrap();
                let value = reqwest::header::HeaderValue::from_str(v).unwrap();
                reqwest_headers.insert(key, value);
            }
        }
        // 重置
        self.force_no_auth = false;
        reqwest_headers
    }

    pub fn set_force_no_auth(&mut self, force_no_auth: bool) -> &mut Self {
        self.force_no_auth = force_no_auth;
        self
    }
    pub async fn post(&mut self, path: &str, data: &Option<serde_json::Value>) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.host, path);
        let mut headers = self.common_headers.clone();
        headers.extend(self.post_headers.clone());
        headers.insert("Content-Type".to_string(), "application/json;charset=UTF-8".to_string());
        // let sec_data = match (data, headers.get("Accept-C")) {
        let sec_data = match (data, headers.get("Accept-C")) {
            (Some(d), Some(accept_c)) if accept_c == "true" => {
                let en = self.coder.encrypt(d.clone());
                Some(serde_json::json!({
                    "encrypt": en,
                }))
            },
            _ => None,
        };
        let (len, d) = match sec_data {
            Some(d) => (d.to_string().len(), d),
            None => (0, serde_json::Value::Null),
        };
        headers.insert("Content-Length".to_string(), len.to_string());
        let headers: reqwest::header::HeaderMap = self.hash_map2header_map(&headers);
        let mut r: Response;
        let mut err_no = 0;
        loop{
            let mut c = self.client.post(&url).headers(headers.clone());
            if len > 0 {
                c = c.json(&d);
            }
            let request = c.build()?;

            reqwest_debug(&request, &data);

            r = self.client.execute(request).await?;

            if r.status().is_success() {
                break;
            }
            err_no += 1;
            if err_no > MAX_RETRY {
                let sc = r.status().as_u16();
                let msg = format!("POST 请求失败, Code: {}", sc);
                return Err(anyhow!(Error{no:0x10001, msg }));
            }
            delay_min_max_secs(1,5).await;
        }
        let body = r.text().await?;
        let v = self.coder.decrypt(body.as_str())?;
        debug_print_body(&v);
        Ok(v)
    }

    pub async fn get(&mut self, path: &str, data: &Option<serde_json::Value>) -> Result<serde_json::Value> {
        let mut headers = self.common_headers.clone();
        headers.extend(self.get_headers.clone());
        let headers: reqwest::header::HeaderMap = self.hash_map2header_map(&headers);
        let sec_data = match (data, headers.get("Accept-C")) {
            (Some(d), Some(accept_c)) if accept_c == "true" => {
                format!("?encryptParams={}", urlencoding::encode(self.coder.encrypt(d.clone()).as_str()))
            },
            _ => "".to_string(),
        };
        let url = format!("{}{}{}", self.host, path, sec_data);
        // let url = Url::parse(url.as_str())?.to_string();

        let mut err_no = 0;

        let r = loop{
            let request = self.client.get(&url).headers(headers.clone()).build()?;

            reqwest_debug(&request, data);

            let resp = self.client.execute(request).await?;
            // 判断 r 的 status code 是否是 200
            if resp.status().is_success() {
                break resp;
            }
            err_no += 1;
            if err_no > MAX_RETRY {
                let sc = resp.status().as_u16();
                let msg = format!("GET 请求失败, Code: {}", sc);
                return Err(anyhow!(Error{no:0x10002, msg, }));
            }
            delay_min_max_secs(1,5).await;
        };

        let body = r.text().await?;

        let v = self.coder.decrypt(body.as_str())?;
        debug_print_body(&v);
        Ok(v)
    }
}


#[cfg(debug_assertions)]
fn reqwest_debug(request: &Request, data: &Option<serde_json::Value>) {
    let method = request.method();
    let url = request.url();
    let headers = request.headers();
    // 打印当前时间
    info!("========================================================================");
    info!("request:");
    info!("time: {:?}", chrono::Local::now());
    info!("method: {:?}", method);
    info!("url: {:?}", url.path());
    info!("headers: {:?}", headers);
    if let Some(d) = data {
        info!("data: {:?}", serde_json::to_string_pretty(d).unwrap());
    }
}

#[cfg(debug_assertions)]
fn reqwest_debug_post(request: &Request, data: &serde_json::Value) {
    let method = request.method();
    let url = request.url();
    let headers = request.headers();
    // 打印当前时间
    info!("========================================================================");
    info!("request:");
    info!("time: {:?}", chrono::Local::now());
    info!("method: {:?}", method);
    info!("url: {:?}", url.path());
    info!("headers: {:?}", headers);
    info!("data: {:?}", serde_json::to_string_pretty(data).unwrap());
}

#[cfg(debug_assertions)]
fn debug_print_body(body: &serde_json::Value) {
    let v = serde_json::to_string_pretty(body).unwrap();
    info!("response:");
    info!("time: {:?}", chrono::Local::now());
    info!("data: {:?}", serde_json::to_string_pretty(&v).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::gsc::data_source::config_source::get_share_config;
    use crate::website::mox_client::MoxClient;
    use crate::website::proto::OfficeDataRequest;

    #[tokio::test]
    async fn test_get(){
        let cfg = get_share_config().unwrap();
        let mut cli = MoxClient::new("http://127.0.0.1:9900", cfg.mox_client, None);

        let input = OfficeDataRequest {
            cat_office_id: 223,
            procces_id: "eyJpdiI6Im5WM3JYMXQrcmtjMzdvcGJEUk41M2c9PSIsInZhbHVlIjoiUjFEZVlFK0g0MDk5cDRoY1NHUnYzZz09IiwibWFjIjoiNGQwYWEyYzYzZDc0MGIxNzBlODEwYmNkMTM0YThlMGRjY2QyNzI4NzE0ODVkYThkNmRmNWY5OGVjMGRlZDA4NyJ9".to_string(),
            jurisdictionId: 1,
            captcha_value: "0ujuxutqp".to_string(),
            usr: "eyJpdiI6Ilh3d01TZkVXdEt6S1FXSUQ5Szh4c1E9PSIsInZhbHVlIjoieXRkeE5PbS85TEl2b0dkR1ordm5QZz09IiwibWFjIjoiMTRmZGNlNGU3MjdmYzQzZDkyNTM4MmY5NGRjZGUzZDljODY0NmM1Mjk0MDk3MDAwNTA2OGQ1Zjk2NzhhMjQ4NyJ9".to_string(),
            api_key: "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk".to_string(),
        };
        let json = serde_json::to_value(input).unwrap();
        let json_value = cli.get("/api/console/office-data", &Option::from(json)).await.unwrap();


    }
}