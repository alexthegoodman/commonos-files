use reqwest_graphql::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::format};

use crate::helpers::auth::create_auth_header;

#[derive(Deserialize)]
pub struct Data {
    pub authenticate: String,
}

// #[derive(Deserialize)]
// pub struct User {
//     id: String,
//     name: String,
// }

#[derive(Serialize)]
pub struct Vars {
    // id: u32,
}

pub async fn authenticate(
    username: &str,
    password: &str,
) -> Result<Data, Box<dyn std::error::Error>> {
    let endpoint = "http://localhost:4000/graphql";
    let query = r#"
        query AuthenticateUser {
            authenticate
        }
   "#;

    let auth_string = format!("{}:{}", username, password);
    let auth_header = create_auth_header(&auth_string);
    let auth_header_str: &str = &auth_header[..];

    let mut headers = HashMap::new();
    headers.insert("Authorization", auth_header_str);

    let client = Client::new_with_headers(endpoint, headers);

    let vars = Vars {};
    let data = client
        .query_with_vars::<Data, Vars>(query, vars)
        .await
        .unwrap();

    // println!("Id: {}, Name: {}", data.user.id, data.user.name);

    Ok(data)
}
