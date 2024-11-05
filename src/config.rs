use crate::prelude::*;

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use serde::Deserialize;
use std::env;

//name_of_the_client:
//  url: "http://test.com/api"
//  resources:
//    - name: endpoint 1
//      resource: /ep1
//      fields_list:
//        - name: field_1_name
//        - name: field_2_name
//    - name: endpoint 2
//      resource: /ep2
//      fields_list:
//        - name: field_1_name
//        - name: field_2_name
//
#[derive(Debug, Default, Deserialize)]
pub struct Field {
    pub name: String,
    // TODO: type
}

#[derive(Debug, Default, Deserialize)]
pub struct Resource {
    pub name: String,
    pub resource: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub name: String,
    pub url: String,
    pub resources: Vec<Resource>,
}

impl Config {
    pub fn from_cli() -> Result<Self> {
        let args: Vec<String> = env::args().collect();

        let file_path = &args[1];
        Config::from_file(file_path)
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        Ok(Figment::new().merge(Yaml::file(file_path)).extract()?)
    }
}
