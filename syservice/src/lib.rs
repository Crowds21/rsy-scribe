pub mod document;
pub mod domain;
pub mod file;
mod handler;
pub mod lute;

static REPO_PATH: &str= "/Users/crowds/Notes/SiYuanKnowledgeBase/data";
static SIYUAN_BASE: &str = "http://127.0.0.1:6806";
static API_SQL_QUERY: &str = "/api/query/sql";
static API_TOKEN: &str = "1g4rmbq473pv40jo";
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use crate::domain::SyResponse;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tokio::runtime::Handle;

// 定义异步搜索函数的 trait 别名（简化类型签名）
type SearchFn = dyn Fn(String) -> Pin<Box<dyn Future<Output = anyhow::Result<SyResponse>> + Send>>
    + Send
    + Sync;

pub fn perform_search<F, Fut>(search_func: F, input: String)
where
    F: Fn(String) -> Fut + 'static + Send + Sync + Copy,
    Fut: Future<Output = anyhow::Result<SyResponse>> + Send + 'static,
{
    let handle = Handle::current();
    tokio::spawn(async move {
        let result = search_func(input).await;

        handle.spawn_blocking(move || match result {
            Ok(res) => println!("Search results: {:?}", res.data),
            Err(e) => eprintln!("Search failed: {}", e),
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
