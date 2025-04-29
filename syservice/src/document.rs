use anyhow::{anyhow, Context};

use super::domain::*;
use super::*;
use std::collections::HashMap;
use serde_json::json;

async fn create_doc_with_md(
    notebook: String,
    path: String,
    markdown: String,
) -> anyhow::Result<()> {
    let url = "/api/filetree/createDocWithMd";

    let mut map = HashMap::new();
    map.insert("notebook", notebook);
    map.insert("path", path);
    map.insert("markdown", markdown);

    let client = reqwest::Client::new();
    let response = client.post(url).json(&map).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response body: {}", body);
    }
    Ok(())
}

pub async fn search_doc_with_title(title: String) -> anyhow::Result<SyResponse, anyhow::Error> {
    let sql = format!(
        "SELECT * FROM blocks WHERE content LIKE '%{}%' and type='d' LIMIT 20",
        title
    );

    let mut map = HashMap::new();
    map.insert("stmt", sql);
    let body = json!(map);

    let client = reqwest::Client::builder()
        .no_proxy()
        .build()?;
    
    let response = client
        .post(format!("{}{}", SIYUAN_BASE, API_SQL_QUERY))
        .header("Content-Type", "application/json")
        .header("Authorization", "Token ".to_owned() + API_TOKEN)
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send request to {}: {}", API_SQL_QUERY , e))?;

    if !response.status().is_success() {
        println!("API returned {}: {}",
                 response.status(), response.url());
        return Err(anyhow!("API returned"));
    }

    response
        .json::<SyResponse>()
        .await
        .map_err(|e| anyhow!("Failed to parse response: {}", e))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_document_list_by_title() {
        let result = search_doc_with_title(String::from("rust")).await;
        match result {
            Ok(resp) => {
                assert_eq!(resp.code, 0);
                assert!(resp.data.len() > 0);
            }
            Err(ref resp) => {
                panic!("Search failed: {:?}", result.err());
            }
        }
    }
}
