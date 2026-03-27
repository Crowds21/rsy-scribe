//! SiYuan API 集成测试
//!
//! 根据官方 API 文档更新后的集成测试
//!
//! # 运行测试
//!
//! ```bash
//! # 运行所有集成测试
//! cargo test --test integration -- --ignored
//!
//! # 运行单个测试
//! cargo test --test integration test_full_workflow -- --ignored --nocapture
//! ```

use syservice::prelude::*;

const TEST_BASE_URL: &str = "http://127.0.0.1:6806";
const TEST_TOKEN: &str = "1g4rmbq473pv40jo";

fn create_client() -> HttpClient {
    let config = Config::builder()
        .base_url(TEST_BASE_URL)
        .token(TEST_TOKEN)
        .timeout_secs(30)
        .max_retries(3)
        .build();
    HttpClient::new(&config).expect("Failed to create HTTP client")
}

/// 完整工作流测试
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_full_workflow() {
    let client = create_client();

    // 1. 检查连通性
    assert!(client.ping().await, "SiYuan service should be reachable");
    println!("✓ SiYuan service is reachable");

    // 2. 获取版本
    let version = client.get_version().await.expect("Failed to get version");
    println!("✓ SiYuan version: {}", version);

    // 3. 列出笔记本
    let notebooks = client.list_notebooks().await.expect("Failed to list notebooks");
    assert!(!notebooks.is_empty(), "Should have at least one notebook");
    println!("✓ Found {} notebooks", notebooks.len());

    // 4. 查找测试文档
    let blocks = client
        .search_by_title("OpenClaw", 5)
        .await
        .expect("Failed to search");
    println!("✓ Found {} documents matching 'OpenClaw'", blocks.len());

    if !blocks.is_empty() {
        // 5. 获取块属性
        let block_id = &blocks[0].id;
        let attrs = client
            .get_block_attrs(block_id)
            .await
            .expect("Failed to get attributes");
        println!("✓ Block {} has {} attributes", block_id, attrs.len());

        // 6. 追加块（使用新的返回值类型）
        let content = "- Test item from integration test";
        let operations = client
            .append_block(block_id, "markdown", content)
            .await
            .expect("Failed to append block");
        assert!(!operations.is_empty());
        println!("✓ Appended {} blocks", operations.len());
        for op in &operations {
            println!("  - Block ID: {}, Action: {}", op.id, op.action);
        }

        // 7. SQL 查询
        let sql = format!("SELECT * FROM blocks WHERE id = '{}' LIMIT 1", block_id);
        let result = client.sql_query(&sql).await.expect("Failed to execute SQL");
        assert!(!result.is_empty());
        println!("✓ SQL query returned {} results", result.len());
    }

    println!("\n✅ All integration tests passed!");
}

/// 笔记本 CRUD 测试（使用新的返回值类型）
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_notebook_crud() {
    let client = create_client();

    // 创建测试笔记本
    let test_name = format!("test_integration_{}", chrono::Utc::now().timestamp());
    let notebook = client
        .create_notebook(&test_name)
        .await
        .expect("Failed to create notebook");
    
    println!("✓ Created notebook: {} ({})", notebook.name, notebook.id);
    assert_eq!(notebook.name, test_name);
    assert!(!notebook.id.is_empty());

    // 验证创建
    let notebooks = client.list_notebooks().await.expect("Failed to list notebooks");
    assert!(
        notebooks.iter().any(|n| n.id == notebook.id),
        "Created notebook should exist"
    );

    // 删除笔记本
    client
        .remove_notebook(&notebook.id)
        .await
        .expect("Failed to remove notebook");
    println!("✓ Removed notebook: {}", notebook.id);

    // 验证删除
    let notebooks = client.list_notebooks().await.expect("Failed to list notebooks");
    assert!(
        !notebooks.iter().any(|n| n.id == notebook.id),
        "Deleted notebook should not exist"
    );

    println!("✅ Notebook CRUD test passed!");
}

/// 文档 CRUD 测试（使用新的 API）
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_document_crud() {
    let client = create_client();

    // 获取一个笔记本
    let notebooks = client.list_notebooks().await.expect("Failed to list notebooks");
    assert!(!notebooks.is_empty(), "Should have at least one notebook");
    let notebook_id = &notebooks[0].id;

    // 创建文档
    let test_path = format!("/test/integration_{}", chrono::Utc::now().timestamp());
    let markdown = "# Integration Test\n\nThis is a test document.";

    let doc_id = client
        .create_doc_with_md(notebook_id, &test_path, markdown)
        .await
        .expect("Failed to create document");
    println!("✓ Created document: {}", doc_id);

    // 验证创建（通过 SQL 查询）
    let sql = format!("SELECT * FROM blocks WHERE id = '{}'", doc_id);
    let blocks = client.sql_query(&sql).await.expect("Failed to execute SQL");
    assert!(!blocks.is_empty(), "Created document should exist");
    assert_eq!(blocks[0].id, doc_id, "Document ID should match");

    // 更新文档（使用新的返回值类型）
    let updated_markdown = "# Updated Title\n\nUpdated content.";
    let operations = client
        .update_block(&doc_id, "markdown", updated_markdown)
        .await
        .expect("Failed to update document");
    assert!(!operations.is_empty());
    println!("✓ Updated document ({} operations)", operations.len());

    // 删除文档（通过 ID）
    client
        .remove_doc_by_id(&doc_id)
        .await
        .expect("Failed to remove document");
    println!("✓ Removed document by ID: {}", doc_id);

    println!("✅ Document CRUD test passed!");
}

/// 块属性测试
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_block_attributes() {
    let client = create_client();

    // 查找测试块
    let blocks = client
        .search_by_title("OpenClaw", 1)
        .await
        .expect("Failed to search");

    if blocks.is_empty() {
        println!("No test block found, skipping test");
        return;
    }

    let block_id = &blocks[0].id;

    // 获取现有属性
    let attrs = client
        .get_block_attrs(block_id)
        .await
        .expect("Failed to get attributes");
    println!("✓ Block has {} existing attributes", attrs.len());

    // 设置自定义属性
    let test_key = "custom-integration-test";
    let test_value = "test-value-123";
    client
        .set_block_attr(block_id, test_key, test_value)
        .await
        .expect("Failed to set attribute");
    println!("✓ Set attribute: {} = {}", test_key, test_value);

    // 验证属性
    let updated_attrs = client
        .get_block_attrs(block_id)
        .await
        .expect("Failed to get attributes");
    assert_eq!(
        updated_attrs.get(test_key),
        Some(&test_value.to_string()),
        "Attribute value should match"
    );

    // 清理：删除测试属性
    let mut delete_attrs = std::collections::HashMap::new();
    delete_attrs.insert(test_key.to_string(), "".to_string());
    client
        .set_block_attrs(block_id, delete_attrs)
        .await
        .expect("Failed to delete attribute");
    println!("✓ Cleaned up test attribute");

    println!("✅ Block attributes test passed!");
}

/// 错误处理测试
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_error_handling() {
    // 测试无效的 Token
    let config = Config::builder()
        .base_url(TEST_BASE_URL)
        .token("invalid_token")
        .build();

    let client = HttpClient::new(&config).expect("Failed to create client");

    // 应该返回认证错误
    let result = client.list_notebooks().await;
    assert!(
        result.is_err(),
        "Should fail with invalid token"
    );

    if let Err(e) = result {
        assert!(
            e.is_auth_error(),
            "Should be auth error, got: {:?}",
            e
        );
        println!("✓ Correctly detected auth error: {}", e.user_message());
    }

    println!("✅ Error handling test passed!");
}

/// 中间件测试
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_with_middleware() {
    use std::sync::{Arc, Mutex};

    // 简单的日志中间件
    struct TestLogger {
        logs: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait::async_trait]
    impl RequestMiddleware for TestLogger {
        async fn before_request(&self, ctx: &mut MiddlewareContext) -> Result<()> {
            let mut logs = self.logs.lock().unwrap();
            logs.push(format!("→ {} {}", ctx.method, ctx.endpoint));
            Ok(())
        }

        async fn after_request(&self, ctx: &MiddlewareContext) -> Result<()> {
            let mut logs = self.logs.lock().unwrap();
            logs.push(format!(
                "← {} {:?} in {:?}",
                ctx.endpoint, ctx.status_code, ctx.duration
            ));
            Ok(())
        }
    }

    let logs = Arc::new(Mutex::new(Vec::new()));
    let logger = TestLogger {
        logs: logs.clone(),
    };

    let client = create_client().with_middleware(Box::new(logger));

    // 执行一个简单请求
    let _ = client.get_version().await;

    // 验证中间件被调用
    let logs = logs.lock().unwrap();
    assert!(!logs.is_empty(), "Middleware should have logged requests");
    assert!(
        logs.iter().any(|l| l.contains("GET") || l.contains("POST")),
        "Should log HTTP method"
    );
    assert!(
        logs.iter().any(|l| l.contains("/api/system/getVersion")),
        "Should log endpoint"
    );

    println!("✓ Middleware logs: {:?}", *logs);
    println!("✅ Middleware test passed!");
}

/// 块操作测试（测试新的 BlockOperation 返回值）
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_block_operations() {
    let client = create_client();

    // 查找测试块
    let blocks = client
        .search_by_title("OpenClaw", 1)
        .await
        .expect("Failed to search");

    if blocks.is_empty() {
        println!("No test block found, skipping test");
        return;
    }

    let parent_id = &blocks[0].id;

    // 1. 测试 prepend_block
    let operations = client
        .prepend_block(parent_id, "markdown", "- Prepended item")
        .await
        .expect("Failed to prepend block");
    
    assert!(!operations.is_empty());
    println!("✓ Prepended {} blocks", operations.len());
    for op in &operations {
        assert!(!op.id.is_empty(), "Operation should have an ID");
        assert_eq!(op.action, "insert", "Action should be 'insert'");
        println!("  - Block ID: {}, Action: {}", op.id, op.action);
    }

    // 2. 测试 append_block
    let operations = client
        .append_block(parent_id, "markdown", "- Appended item")
        .await
        .expect("Failed to append block");
    
    assert!(!operations.is_empty());
    println!("✓ Appended {} blocks", operations.len());

    // 3. 测试 insert_block（通过 previousID）
    if let Some(last_op) = operations.last() {
        let operations = client
            .insert_block(
                "markdown",
                "- Inserted item",
                Some(&last_op.id), // previousID
                None,              // nextID
                None,              // parentID
            )
            .await
            .expect("Failed to insert block");
        
        assert!(!operations.is_empty());
        println!("✓ Inserted {} blocks", operations.len());
    }

    // 4. 测试 delete_block
    if let Some(last_op) = operations.last() {
        let operations = client
            .delete_block(&last_op.id)
            .await
            .expect("Failed to delete block");
        
        assert!(!operations.is_empty());
        println!("✓ Deleted {} blocks", operations.len());
        for op in &operations {
            assert_eq!(op.action, "delete", "Action should be 'delete'");
        }
    }

    println!("✅ Block operations test passed!");
}

/// 文档重命名测试（通过 ID）
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_rename_doc_by_id() {
    let client = create_client();

    // 获取一个笔记本
    let notebooks = client.list_notebooks().await.expect("Failed to list notebooks");
    let notebook_id = &notebooks[0].id;

    // 创建文档
    let test_path = format!("/test/rename_{}", chrono::Utc::now().timestamp());
    let markdown = "# Original Title";
    
    let doc_id = client
        .create_doc_with_md(notebook_id, &test_path, markdown)
        .await
        .expect("Failed to create document");

    // 通过 ID 重命名
    let new_title = "Renamed Title";
    client
        .rename_doc_by_id(&doc_id, new_title)
        .await
        .expect("Failed to rename document");
    
    println!("✓ Renamed document {} to '{}'", doc_id, new_title);

    // 清理
    client
        .remove_doc_by_id(&doc_id)
        .await
        .expect("Failed to remove document");

    println!("✅ Rename doc by ID test passed!");
}
