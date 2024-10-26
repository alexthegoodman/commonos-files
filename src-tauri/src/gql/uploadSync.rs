use reqwest_graphql::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::format, str::FromStr};

#[derive(Deserialize)]
pub struct Data {
    pub uploadSync: String,
}

#[derive(Serialize)]
pub struct Vars {
    // id: u32,
    fileName: String,
    filePath: String,
    fileData: String,
}

pub async fn upload_sync_one_file(
    auth_token: String,
    file_name: String,
    file_path: String,
    file_data: String,
) -> Result<Data, Box<dyn std::error::Error>> {
    let endpoint = "http://localhost:4000/graphql";
    let query = r#"
        mutation UploadSync($fileName: String!, $filePath: String!, $fileData: String!) {
            uploadSync(fileName: $fileName, filePath: $filePath, fileData: $fileData)
        }
   "#;

    let auth_header = "Bearer ".to_owned() + &auth_token.clone();
    let auth_header_str: &str = &auth_header[..];

    let mut headers = HashMap::new();
    headers.insert("Authorization", auth_header_str);

    let client = Client::new_with_headers(endpoint, headers);

    let vars = Vars {
        fileName: file_name,
        filePath: file_path,
        fileData: file_data,
    };

    println!("Making call...");

    let data = client
        .query_with_vars::<Data, Vars>(query, vars)
        .await
        .unwrap();

    // println!("Id: {}, Name: {}", data.user.id, data.user.name);

    Ok(data)
}
