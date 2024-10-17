#![allow(unused)]

use crate::prelude::*;
use serde_json::json;
use std::collections::HashMap;

mod error;
mod prelude;
mod provider;
mod providers;
mod utils;

use crate::provider::*;
use crate::providers::JsonPlaceholder;
#[tokio::main]
async fn main() {
    let url = "http://jsonplaceholder.typicode.com/";
    let resource = Resource::new("posts");
    let api = JsonPlaceholder::new(url).unwrap();

    let params = GetListParams {
        pagination: None,
        sort: None,
        filter: None,
        meta: None,
    };

    println!("###### LIST ######");
    match api.get_list(resource.clone(), params).await {
        Ok(result) => {
            for record in result.data {
                println!("Record ID: {}", record.id());
                for (key, value) in &record.fields {
                    println!("Field: {}, Value: {}", key, value);
                }
            }
        }
        Err(e) => println!("Error fetching list: {:?}", e),
    }

    let params = GetOneParams {
        id: Identifier::Str("1".to_string()),
        meta: None,
    };

    println!("###### ONE ######");
    match api.get_one(resource.clone(), params).await {
        Ok(result) => {
            println!("Record ID: {}", result.data.id());
            for (key, value) in &result.data.fields {
                println!("Field: {}, Value: {}", key, value);
            }
        }
        Err(e) => println!("Error fetching list: {:?}", e),
    }

    let params = GetManyParams {
        ids: vec![
            Identifier::Str("1".to_string()),
            Identifier::Str("2".to_string()),
            Identifier::Str("99".to_string()),
        ],
        meta: None,
    };

    println!("###### MANY ######");
    match api.get_many(resource.clone(), params).await {
        Ok(result) => {
            for record in result.data {
                println!("Record ID: {}", record.id());
                for (key, value) in &record.fields {
                    println!("Field: {}, Value: {}", key, value);
                }
            }
        }
        Err(e) => println!("Error fetching list: {:?}", e),
    }

    let params = GetManyReferenceParams {
        id: Identifier::Str("1".to_string()),
        target: "comments".to_string(),
        pagination: PaginationPayload {
            page: 1,
            per_page: 10,
        },
        sort: SortPayload {
            field: String::from("name"),
            order: SortOrder::Asc,
        },
        filter: HashMap::new(),
        meta: None,
    };

    println!("###### MANY REFERENCE ######");
    match api.get_many_reference(resource.clone(), params).await {
        Ok(result) => {
            for record in result.data {
                println!("Record ID: {}", record.id());
                for (key, value) in &record.fields {
                    println!("Field: {}, Value: {}", key, value);
                }
            }
        }
        Err(e) => println!("Error fetching list: {:?}", e),
    }

    println!("###### CREATE ######");
    let mut fields = HashMap::new();
    fields.insert("userId".to_string(), json!("1".to_string()));
    fields.insert("body".to_string(), json!("sarasa".to_string()));
    fields.insert("title".to_string(), json!("pepe".to_string()));
    let params = CreateParams {
        data: fields.clone(),
        meta: None,
    };

    match api.create(resource.clone(), params).await {
        Ok(result) => {
            println!("Record ID: {}", result.data.id());
            for (key, value) in &result.data.fields {
                println!("Field: {}, Value: {}", key, value);
            }
        }
        Err(e) => println!("Error fetching list: {:?}", e),
    }

    println!("###### Update ######");
    let mut fields = HashMap::new();
    fields.insert("userId".to_string(), json!("1".to_string()));
    fields.insert("body".to_string(), json!("sarasa".to_string()));
    fields.insert("title".to_string(), json!("pepe".to_string()));
    let params = UpdateParams {
        id: Identifier::Str("1".to_string()),
        data: fields.clone(),
        previous_data: Record {
            id: Identifier::Num(1),
            fields,
        },
        meta: None,
    };

    match api.update(resource.clone(), params).await {
        Ok(result) => {
            println!("Record ID: {}", result.data.id());
            for (key, value) in &result.data.fields {
                println!("Field: {}, Value: {}", key, value);
            }
        }
        Err(e) => println!("Error fetching list: {:?}", e),
    }

    println!("###### Update Many ######");
    let mut fields = HashMap::new();
    fields.insert("userId".to_string(), json!("1".to_string()));
    fields.insert("body".to_string(), json!("sarasa".to_string()));
    fields.insert("title".to_string(), json!("pepe".to_string()));
    let params = UpdateManyParams {
        ids: vec![
            Identifier::Str("1".to_string()),
            Identifier::Str("2".to_string()),
        ],
        data: fields.clone(),
        meta: None,
    };

    match api.update_many(resource.clone(), params).await {
        Ok(result) => {
            for record in result.data {
                println!("Record ID: {}", record);
            }
        }
        Err(e) => println!("Error fetching list: {:?}", e),
    }
}
