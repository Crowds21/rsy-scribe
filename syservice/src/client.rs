//! HTTP 客户端实现

use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::api::{ApiResponse, BlockOperation, MiddlewareContext, Notebook, RequestMiddleware, SiYuanClient};
use crate::config::Config;
use crate::domain::SyBlock;
use crate::error::{Result, SiYuanError};

/// HTTP 客户端实现
pub struct HttpClient {
    client: Client,
    config: Config,
    middlewares: Vec<Box<dyn RequestMiddleware>>,
}

impl HttpClient {
    pub fn new(config: &Config) -> Result<Self> {
        config.validate().map_err(SiYuanError::Config)?;
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .no_proxy()
            .build()
            .map_err(SiYuanError::Http)?;
        Ok(Self {
            client,
            config: config.clone(),
            middlewares: Vec::new(),
        })
    }

    pub fn with_middleware(mut self, middleware: Box<dyn RequestMiddleware>) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn config(&self) -> &Config { &self.config }

    fn auth_headers(&self) -> reqwest::header::HeaderMap {
        use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Token {}", self.config.token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    async fn post<T: serde::de::DeserializeOwned + Default>(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
    ) -> Result<ApiResponse<T>> {
        let start = Instant::now();
        let mut ctx = MiddlewareContext::new(endpoint, "POST");
        ctx.body = Some(body.to_string());

        // before_request 钩子
        for middleware in &self.middlewares {
            if let Err(e) = middleware.before_request(&mut ctx).await {
                return Err(e);
            }
        }

        // 执行请求
        let result = self.execute_with_retry(endpoint, body).await;

        // 更新上下文
        ctx.duration = start.elapsed();
        match &result {
            Ok(_) => ctx.status_code = Some(200),
            Err(e) => ctx.error = Some(e.to_string()),
        }

        // after_request 钩子
        for middleware in &self.middlewares {
            let _ = middleware.after_request(&ctx).await;
        }

        result
    }

    async fn execute_with_retry<T: serde::de::DeserializeOwned + Default>(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
    ) -> Result<ApiResponse<T>> {
        let url = format!("{}{}", self.config.base_url, endpoint);
        let headers = self.auth_headers();
        let mut attempts = 0;
        let max_retries = 3;
        let retry_delay_ms = 100;

        loop {
            attempts += 1;
            match self.execute_post(&url, &headers, body).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if !e.is_retryable() || attempts >= max_retries {
                        return Err(e);
                    }
                    let delay = retry_delay_ms * (1 << (attempts - 1));
                    tokio::time::sleep(Duration::from_millis(delay as u64)).await;
                }
            }
        }
    }

    async fn execute_post<T: serde::de::DeserializeOwned + Default>(
        &self,
        url: &str,
        headers: &reqwest::header::HeaderMap,
        body: &serde_json::Value,
    ) -> Result<ApiResponse<T>> {
        let response = self.client.post(url).headers(headers.clone()).json(body).send().await.map_err(|e| {
            if e.is_timeout() {
                SiYuanError::Timeout { timeout_secs: self.config.timeout_secs }
            } else if e.is_connect() {
                SiYuanError::ConnectionFailed { url: url.to_string() }
            } else {
                SiYuanError::Http(e)
            }
        })?;

        match response.status() {
            StatusCode::OK => {}
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                return Err(SiYuanError::AuthFailed(format!("API returned {}: {}", response.status(), url)));
            }
            StatusCode::NOT_FOUND => {
                return Err(SiYuanError::NotFound {
                    resource_type: "endpoint".to_string(),
                    identifier: url.to_string(),
                });
            }
            StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => {
                return Err(SiYuanError::Timeout { timeout_secs: self.config.timeout_secs });
            }
            _ => return Err(SiYuanError::Internal(format!("Unexpected HTTP status: {}", response.status()))),
        }

        let api_response: ApiResponse<T> = response.json().await.map_err(|e: reqwest::Error| {
            SiYuanError::Internal(format!("JSON parse error: {}", e))
        })?;

        if api_response.code != 0 {
            return Err(SiYuanError::Api {
                code: api_response.code,
                message: api_response.msg.clone(),
            });
        }

        Ok(api_response)
    }
}

#[async_trait]
impl SiYuanClient for HttpClient {
    async fn list_notebooks(&self) -> Result<Vec<Notebook>> {
        #[derive(Debug, Default, serde::Deserialize)]
        struct NotebooksData { notebooks: Vec<Notebook> }
        let response: ApiResponse<NotebooksData> = self.post("/api/notebook/lsNotebooks", &json!({})).await?;
        Ok(response.data.notebooks)
    }

    async fn create_notebook(&self, name: &str) -> Result<Notebook> {
        #[derive(Debug, Default, serde::Deserialize)]
        struct CreateData { notebook: Notebook }
        let body = json!({ "name": name });
        let response: ApiResponse<CreateData> = self.post("/api/notebook/createNotebook", &body).await?;
        Ok(response.data.notebook)
    }

    async fn remove_notebook(&self, notebook_id: &str) -> Result<()> {
        let body = json!({ "notebook": notebook_id });
        let _: ApiResponse<()> = self.post("/api/notebook/removeNotebook", &body).await?;
        Ok(())
    }

    async fn create_doc_with_md(&self, notebook_id: &str, path: &str, markdown: &str) -> Result<String> {
        #[derive(Debug, Default, serde::Deserialize)]
        struct CreateData { id: String }
        let body = json!({ "notebook": notebook_id, "path": path, "markdown": markdown });
        let response: ApiResponse<CreateData> = self.post("/api/filetree/createDocWithMd", &body).await?;
        Ok(response.data.id)
    }

    async fn remove_doc(&self, doc_id: &str) -> Result<()> {
        // 通过 path 删除（需要 notebook）
        // 这里简化处理，假设 doc_id 就是 path
        let body = json!({ "path": format!("/{}.sy", doc_id) });
        let _: ApiResponse<()> = self.post("/api/filetree/removeDoc", &body).await?;
        Ok(())
    }

    async fn remove_doc_by_id(&self, id: &str) -> Result<()> {
        let body = json!({ "id": id });
        let _: ApiResponse<()> = self.post("/api/filetree/removeDocByID", &body).await?;
        Ok(())
    }

    async fn rename_doc_by_id(&self, id: &str, title: &str) -> Result<()> {
        let body = json!({ "id": id, "title": title });
        let _: ApiResponse<()> = self.post("/api/filetree/renameDocByID", &body).await?;
        Ok(())
    }

    async fn rename_doc(&self, notebook_id: &str, path: &str, title: &str) -> Result<()> {
        let body = json!({ "notebook": notebook_id, "path": path, "title": title });
        let _: ApiResponse<()> = self.post("/api/filetree/renameDoc", &body).await?;
        Ok(())
    }

    async fn insert_block(
        &self,
        data_type: &str,
        data: &str,
        previous_id: Option<&str>,
        next_id: Option<&str>,
        parent_id: Option<&str>,
    ) -> Result<Vec<BlockOperation>> {
        #[derive(Debug, Default, serde::Deserialize)]
        struct ResponseData {
            doOperations: Vec<BlockOperation>,
        }
        let mut body = serde_json::Map::new();
        body.insert("dataType".to_string(), json!(data_type));
        body.insert("data".to_string(), json!(data));
        if let Some(id) = previous_id {
            body.insert("previousID".to_string(), json!(id));
        }
        if let Some(id) = next_id {
            body.insert("nextID".to_string(), json!(id));
        }
        if let Some(id) = parent_id {
            body.insert("parentID".to_string(), json!(id));
        }
        let response: ApiResponse<Vec<ResponseData>> = self.post("/api/block/insertBlock", &json!(body)).await?;
        let mut operations = Vec::new();
        for item in response.data {
            operations.extend(item.doOperations);
        }
        Ok(operations)
    }

    async fn prepend_block(
        &self,
        parent_id: &str,
        data_type: &str,
        data: &str,
    ) -> Result<Vec<BlockOperation>> {
        #[derive(Debug, Default, serde::Deserialize)]
        struct ResponseData {
            doOperations: Vec<BlockOperation>,
        }
        let body = json!({ "parentID": parent_id, "dataType": data_type, "data": data });
        let response: ApiResponse<Vec<ResponseData>> = self.post("/api/block/prependBlock", &body).await?;
        let mut operations = Vec::new();
        for item in response.data {
            operations.extend(item.doOperations);
        }
        Ok(operations)
    }

    async fn append_block(
        &self,
        parent_id: &str,
        data_type: &str,
        data: &str,
    ) -> Result<Vec<BlockOperation>> {
        #[derive(Debug, Default, serde::Deserialize)]
        struct ResponseData {
            doOperations: Vec<BlockOperation>,
        }
        let body = json!({ "parentID": parent_id, "dataType": data_type, "data": data });
        let response: ApiResponse<Vec<ResponseData>> = self.post("/api/block/appendBlock", &body).await?;
        let mut operations = Vec::new();
        for item in response.data {
            operations.extend(item.doOperations);
        }
        Ok(operations)
    }

    async fn update_block(&self, block_id: &str, data_type: &str, data: &str) -> Result<Vec<BlockOperation>> {
        #[derive(Debug, Default, serde::Deserialize)]
        struct ResponseData {
            doOperations: Vec<BlockOperation>,
        }
        let body = json!({ "dataType": data_type, "data": data, "id": block_id });
        let response: ApiResponse<Vec<ResponseData>> = self.post("/api/block/updateBlock", &body).await?;
        let mut operations = Vec::new();
        for item in response.data {
            operations.extend(item.doOperations);
        }
        Ok(operations)
    }

    async fn delete_block(&self, block_id: &str) -> Result<Vec<BlockOperation>> {
        #[derive(Debug, Default, serde::Deserialize)]
        struct ResponseData {
            doOperations: Vec<BlockOperation>,
        }
        let body = json!({ "id": block_id });
        let response: ApiResponse<Vec<ResponseData>> = self.post("/api/block/deleteBlock", &body).await?;
        let mut operations = Vec::new();
        for item in response.data {
            operations.extend(item.doOperations);
        }
        Ok(operations)
    }

    async fn sql_query(&self, sql: &str) -> Result<Vec<SyBlock>> {
        // 官方 API: SQL 查询返回的是动态 JSON 数组 [{ "列": "值" }]
        // 需要手动转换为 SyBlock
        // https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#执行-sql-查询
        let body = json!({ "stmt": sql });
        let response: ApiResponse<Vec<serde_json::Value>> = self.post("/api/query/sql", &body).await?;
        
        // 将 JSON Value 转换为 SyBlock
        let blocks: Result<Vec<SyBlock>> = response.data
            .into_iter()
            .map(|value| {
                serde_json::from_value::<SyBlock>(value)
                    .map_err(|e| SiYuanError::Internal(format!("Failed to parse SyBlock: {}", e)))
            })
            .collect();
        
        blocks
    }

    async fn search_by_title(&self, title: &str, limit: usize) -> Result<Vec<SyBlock>> {
        let sql = format!("SELECT * FROM blocks WHERE content LIKE '%{}%' AND type='d' LIMIT {}", title, limit);
        self.sql_query(&sql).await
    }

    async fn get_block_attrs(&self, block_id: &str) -> Result<HashMap<String, String>> {
        // 官方 API: data 直接就是属性对象 { "custom-attr1": "value", "id": "..." }
        // https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#获取块属性
        let body = json!({ "id": block_id });
        let response: ApiResponse<HashMap<String, String>> = self.post("/api/attr/getBlockAttrs", &body).await?;
        Ok(response.data)
    }

    async fn set_block_attrs(&self, block_id: &str, attrs: HashMap<String, String>) -> Result<()> {
        let body = json!({ "id": block_id, "attrs": attrs });
        let _: ApiResponse<()> = self.post("/api/attr/setBlockAttrs", &body).await?;
        Ok(())
    }

    async fn set_block_attr(&self, block_id: &str, key: &str, value: &str) -> Result<()> {
        let mut attrs = HashMap::new();
        attrs.insert(key.to_string(), value.to_string());
        self.set_block_attrs(block_id, attrs).await
    }

    async fn get_version(&self) -> Result<String> {
        // 官方 API: /api/system/version 直接返回版本字符串
        // https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#获取系统版本
        let response: ApiResponse<String> = self.post("/api/system/version", &json!({})).await?;
        Ok(response.data)
    }

    async fn ping(&self) -> bool {
        self.get_version().await.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let config = Config::builder().base_url("http://localhost:6806").token("test_token").build();
        let client = HttpClient::new(&config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_invalid_config() {
        // 默认配置现在是有效的（token 不是必需的）
        let config = Config::default();
        let client = HttpClient::new(&config);
        assert!(client.is_ok());
        
        // 无效的 URL 才会导致错误
        let config = Config::builder()
            .base_url("invalid-url")
            .build();
        let client = HttpClient::new(&config);
        assert!(client.is_err());
    }
}
