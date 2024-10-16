use crate::prelude::*;

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

    async fn my_get_list(&self, url: &str) -> Result<Vec<Record>> {
        let response = reqwest::get(url).await?;
        self.check_status(&response)?;
        let body = response.text().await?;
        dbg!("Raw response body: {}", &body);

        let data: Vec<Record> = serde_json::from_str(&body)?;
        Ok(data)
    }
}

impl DataProvider for JsonPlaceholder {
    fn get_list(
        &self,
        resource: Resource,
        params: GetListParams,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GetListResult>> + '_ >> {
        Box::pin(async move {
            let res = resource.resource.clone();
            let url = self.url.join(&res)?.to_string();
            match self.my_get_list(&url).await {
                Ok(records) => {
                    let total = records.len();
                    let result = GetListResult {
                        data: records,
                        total: Some(total),
                        meta: None,
                        page_info: None,
                    };
                    Ok(result)
                }
                Err(e) => Err(e),
            }
        })
    }
}
