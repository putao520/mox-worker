use anyhow::Result;
use minio_rsc::client::Bucket;
use minio_rsc::Minio;
use minio_rsc::provider::StaticProvider;
use crate::gsc::config::system_config::SystemConfig;


static BUCKET_NAME: &str = "appointment-documents";

pub async fn load_s3(config: &SystemConfig) -> Result<Bucket> {
    let s3_config = &config.s3_config;
    let provider = StaticProvider::new(s3_config.access_key.as_str(), s3_config.secret_key.as_str(), None);
    let minio = Minio::builder()
        .region(s3_config.region.clone())
        .endpoint(s3_config.endpoint.clone())
        .provider(provider)
        .secure(false)
        .build()?;
    if !minio.bucket_exists(BUCKET_NAME).await? {
        minio.make_bucket(BUCKET_NAME, false).await?;
    }
    let bucket = minio.bucket(BUCKET_NAME);
    Ok(bucket)
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose;
    use bytes::Bytes;
    use super::*;
    use crate::gsc::config::system_config::S3Config;

    #[tokio::test]
    async fn test_load_s3() {
        let s3_config = S3Config {
            region: "us-east-1".to_string(),
            endpoint: "localhost:9000".to_string(),
            access_key: "2IiYe6HtY8RpF8R2j6bi".to_string(),
            secret_key: "hv94WgGAsqTO6YCujEXsN6ZGovKy7OW1edzz6xCV".to_string(),
        };
        let mut system_config = SystemConfig::new();
        system_config.s3_config = s3_config;


        // 测试加载 S3
        let bucket = load_s3(&system_config).await.unwrap();
        assert!(bucket.exists().await.unwrap());

        // 测试写入base64文本到 test.txt 文件
        let file_name = "test.txt";
        let string_content = "hello putao520";
        let base64_content = general_purpose::STANDARD.encode(string_content);
        let content = general_purpose::STANDARD.decode(base64_content.clone()).unwrap();
        bucket.put_object(file_name, Bytes::from(content)).await.unwrap();

        // 测试读取文件
        let object = bucket.get_object(file_name).await.unwrap();
        let content = object.text().await.unwrap();
        assert_eq!(content, string_content);
        let content = general_purpose::STANDARD.encode(content);
        assert_eq!(content, base64_content);

        // 测试删除文件
        bucket.remove_object(file_name).await.unwrap();
        let stat = bucket.stat_object(file_name).await.unwrap();
        assert!(stat.is_none());
    }

    #[tokio::test]
    async fn test_load_2_s3() {
        let s3_config = S3Config {
            region: "us-east-1".to_string(),
            endpoint: "localhost:9000".to_string(),
            access_key: "putao520".to_string(),
            secret_key: "YuYao1022@".to_string(),
        };
        let mut system_config = SystemConfig::new();
        system_config.s3_config = s3_config;


        // 测试加载 S3
        let bucket = load_s3(&system_config).await.unwrap();
        assert!(bucket.exists().await.unwrap());

        // 测试写入base64文本到 test.txt 文件
        let file_name = "test.txt";
        let string_content = "hello putao520";
        let base64_content = general_purpose::STANDARD.encode(string_content);
        let content = general_purpose::STANDARD.decode(base64_content.clone()).unwrap();
        bucket.put_object(file_name, Bytes::from(content)).await.unwrap();

        // 测试读取文件
        let object = bucket.get_object(file_name).await.unwrap();
        let content = object.text().await.unwrap();
        assert_eq!(content, string_content);
        let content = general_purpose::STANDARD.encode(content);
        assert_eq!(content, base64_content);

        // 测试删除文件
        bucket.remove_object(file_name).await.unwrap();
        let stat = bucket.stat_object(file_name).await.unwrap();
        assert!(stat.is_none());
    }
}