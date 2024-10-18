use crate::prelude::*;

use std::collections::HashMap;

use crate::provider::*;
use url::{ParseError, Url};

pub struct JsonPlaceholder {
    url: Url,
}

impl JsonPlaceholder {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            url: Url::parse(url)?,
        })
    }

    fn check_status(&self, response: &reqwest::Response) -> Result<()> {
        if !response.status().is_success() {
            return Err(Error::RequestStatus(format!(
                "Received non-200 response: {}",
                response.status()
            )));
        }

        Ok(())
    }

    async fn my_get_list(&self, url: &str) -> Result<GetListResult> {
        let response = reqwest::get(url).await?;
        self.check_status(&response)?;
        let body = response.text().await?;
        println!("Raw response body: {}", body);

        let records: Vec<Record> = serde_json::from_str(&body).unwrap();
        let total: usize = records.len() as usize;
        Ok(GetListResult {
            data: records,
            total: Some(total),
            meta: None,
            page_info: None,
        })
    }

    async fn my_get_one(&self, url: &str) -> Result<GetOneResult> {
        let response = reqwest::get(url).await?;
        self.check_status(&response)?;
        let body = response.text().await?;
        //dbg!("Raw response body: {}", &body);

        let records: Record = serde_json::from_str(&body)?;
        Ok(GetOneResult { data: records })
    }

    async fn my_get_many(&self, url: &str) -> Result<GetManyResult> {
        let response = reqwest::get(url).await?;
        self.check_status(&response)?;
        let body = response.text().await?;
        //dbg!("Raw response body: {}", &body);

        let records: Vec<Record> = serde_json::from_str(&body)?;
        Ok(GetManyResult { data: records })
    }

    async fn my_get_many_reference(&self, url: &str) -> Result<GetManyReferenceResult> {
        let response = reqwest::get(url).await?;
        self.check_status(&response)?;
        let body = response.text().await?;
        //dbg!("Raw response body: {}", &body);

        let records: Vec<Record> = serde_json::from_str(&body)?;
        let total: usize = records.len() as usize;

        Ok(GetManyReferenceResult {
            data: records,
            total: if total > 0 { Some(total) } else { None },
            page_info: None,
            meta: None,
        })
    }

    async fn my_create(
        &self,
        url: &str,
        data: HashMap<String, serde_json::Value>,
    ) -> Result<CreateResult> {
        let client = reqwest::Client::new();

        let response = client.post(url).json(&data).send().await?;
        self.check_status(&response)?;
        let body = response.text().await?;
        //dbg!("Raw response body: {}", &body);
        let records: Record = serde_json::from_str(&body)?;
        Ok(CreateResult { data: records })
    }

    async fn my_update(
        &self,
        url: &str,
        data: HashMap<String, serde_json::Value>,
    ) -> Result<UpdateResult> {
        let client = reqwest::Client::new();

        let response = client.put(url).json(&data).send().await?;
        self.check_status(&response)?;
        let body = response.text().await?;
        //dbg!("Raw response body: {}", &body);
        let records: Record = serde_json::from_str(&body)?;
        Ok(UpdateResult { data: records })
    }

    async fn my_update_many(
        &self,
        url: &str,
        ids: Vec<Identifier>,
        data: HashMap<String, serde_json::Value>,
    ) -> Result<UpdateManyResult> {
        let client = reqwest::Client::new();

        let mut updates_ids = Vec::new();
        for id in ids {
            let url_with_id = format!("{}{}{}", url, "/", id);
            let response = client.put(url).json(&data).send().await?;
            if self.check_status(&response).is_ok() {
                updates_ids.push(id);
            }
        }

        Ok(UpdateManyResult { data: updates_ids })
    }

    async fn my_delete(&self, url: &str, data: Option<Record>) -> Result<DeleteResult> {
        let client = reqwest::Client::new();

        let response = client.delete(url).send().await?;
        self.check_status(&response)?;
        match data {
            Some(record) => Ok(DeleteResult { data: record }),
            None => Err(Error::Unknown("DetelteParams wrong".to_string())),
        }
    }

    async fn my_delete_many(&self, url: &str, ids: Vec<Identifier>) -> Result<DeleteManyResult> {
        let client = reqwest::Client::new();

        let mut deleted_ids = Vec::new();
        for id in ids {
            let url_with_id = format!("{}{}{}", url, "/", id);
            let response = client.delete(url_with_id).send().await?;
            if self.check_status(&response).is_ok() {
                deleted_ids.push(id);
            }
        }

        Ok(DeleteManyResult { data: deleted_ids })
    }
}

impl DataProvider for JsonPlaceholder {
    fn get_list(
        &self,
        resource: Resource,
        params: GetListParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GetListResult>> + '_>> {
        Box::pin(async move {
            let res = resource.resource;
            let url = self.url.join(&res)?.to_string();
            self.my_get_list(&url).await
        })
    }
    fn get_one(
        &self,
        resource: Resource,
        params: GetOneParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GetOneResult>> + '_>> {
        Box::pin(async move {
            let resource_path = format!("{}{}", resource.resource, "/");
            let url = self.url.join(&resource_path)?;
            let id: String = params.id.into();
            let url_with_id = url.join(&id)?.to_string();
            self.my_get_one(&url_with_id).await
        })
    }
    fn get_many(
        &self,
        resource: Resource,
        params: GetManyParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GetManyResult>> + '_>> {
        Box::pin(async move {
            let ids_query_param = params
                .ids
                .iter()
                .map(|id| format!("id={}", id))
                .collect::<Vec<_>>()
                .join("&");
            let resource_path = format!("{}{}{}", resource.resource, "?", ids_query_param);
            let url = self.url.join(&resource_path)?.to_string();
            self.my_get_many(&url).await
        })
    }

    fn get_many_reference(
        &self,
        resource: Resource,
        params: GetManyReferenceParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GetManyReferenceResult>> + '_>>
    {
        Box::pin(async move {
            let resource_path = format!("{}{}", resource.resource, "/");
            let id_path = format!("{}{}", &params.id.to_string(), "/");
            let url = self.url.join(&resource_path)?;
            let url_with_id = url.join(&id_path)?;
            let url_with_id_and_target = url_with_id.join(&params.target)?.to_string();
            self.my_get_many_reference(&url_with_id_and_target).await
        })
    }

    fn create(
        &self,
        resource: Resource,
        params: CreateParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<CreateResult>> + '_>> {
        Box::pin(async move {
            let url = self.url.join(&resource.resource)?.to_string();
            self.my_create(&url, params.data).await
        })
    }

    fn update(
        &self,
        resource: Resource,
        params: UpdateParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<UpdateResult>> + '_>> {
        Box::pin(async move {
            let resource_path = format!("{}{}", resource.resource, "/");
            let url = self.url.join(&resource_path)?;
            let url_with_id = url.join(&params.id.to_string())?.to_string();
            self.my_update(&url_with_id, params.data).await
        })
    }

    fn update_many(
        &self,
        resource: Resource,
        params: UpdateManyParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<UpdateManyResult>> + '_>> {
        Box::pin(async move {
            let resource_path = format!("{}{}", resource.resource, "/");
            let url = self.url.join(&resource_path)?.to_string();
            self.my_update_many(&url, params.ids, params.data).await
        })
    }

    fn delete(
        &self,
        resource: Resource,
        params: DeleteParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<DeleteResult>> + '_>> {
        Box::pin(async move {
            let resource_path = format!("{}{}", resource.resource, "/");
            let url = self.url.join(&resource_path)?;
            let url_with_id = url.join(&params.id.to_string())?.to_string();
            self.my_delete(&url_with_id, params.previous_data).await
        })
    }

    fn delete_many(
        &self,
        resource: Resource,
        params: DeleteManyParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<DeleteManyResult>> + '_>> {
        Box::pin(async move {
            let resource_path = format!("{}{}", resource.resource, "/");
            let url = self.url.join(&resource_path)?.to_string();
            self.my_delete_many(&url, params.ids).await
        })
    }
}
