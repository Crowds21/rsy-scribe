use anyhow::Context;

use super::*;
use std::collections::HashMap;
use super::domain::*;

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

async fn search_doc_with_title(title: String) -> anyhow::Result<SyResponse, anyhow::Error> {
    let sql = format!(
        "SELECT * FROM blocks WHERE content LIKE'%{}%' LIMIT 20",
        title
    );

    println!("Query sql: {}", sql);
    println!("Url: {}",format!("{}{}", SIYUAN_BASE, API_SQL_QUERY) );
    let mut map = HashMap::new();
    map.insert("stmt", sql);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}{}", SIYUAN_BASE, API_SQL_QUERY))
        .header("Content-Type", "application/json")
        .header("Authorization","token 1g4rmbq473pv40jo")
        .json(&map)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let parsed_response: SyResponse = serde_json::from_str(&body)?;
        Ok(parsed_response)
    } else {
        Err(anyhow::anyhow!(
            "Request failed with status: {}",
            response.status()
        ))
    }
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
            Err(resp) => {
                
            }
        }
    }
}
