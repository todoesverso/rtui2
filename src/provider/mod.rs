use crate::prelude::*;

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::future::Future;
use std::pin::Pin;

// Identifier type can be either a String or a u64 (number in Rust).
#[derive(Debug, Deserialize)]
#[serde(untagged)] // Allow deserialization of both strings and numbers
pub enum Identifier {
    Str(String),
    Num(usize),
}

// Implement `Display` for `Identifier`
impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier::Str(s) => write!(f, "{}", s),
            Identifier::Num(n) => write!(f, "{}", n),
        }
    }
}

// Implement `From<String>` for `Identifier`
impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier::Str(value)
    }
}

// Implement `From<u64>` for `Identifier`
impl From<usize> for Identifier {
    fn from(value: usize) -> Self {
        Identifier::Num(value)
    }
}

// Implement `From<Identifier>` for `String`
impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        match identifier {
            Identifier::Str(s) => s,
            Identifier::Num(n) => n.to_string(),
        }
    }
}

// RaRecord struct with generic IdentifierType
pub trait RaRecord {
    type IdentifierType;
    fn id(&self) -> &Self::IdentifierType;
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Identifier,
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

impl RaRecord for Record {
    type IdentifierType = Identifier;
    fn id(&self) -> &Self::IdentifierType {
        &self.id
    }
}

// SortPayload struct
pub struct SortPayload {
    pub field: String,
    pub order: SortOrder,
}

pub enum SortOrder {
    Asc,
    Desc,
}

// FilterPayload as a HashMap
pub type FilterPayload = HashMap<String, String>;

// Meta as a HashMap
pub type Meta = HashMap<String, serde_json::Value>;

#[derive(Clone)]
pub struct Resource {
    pub resource: String,
}

impl Resource {
    pub fn new(res: &str) -> Self {
        Self {
            resource: res
                .trim()
                .trim_start_matches("/")
                .trim_end_matches("/")
                .to_string(),
        }
    }
}

// PaginationPayload struct
pub struct PaginationPayload {
    pub page: usize,
    pub per_page: usize,
}

// DataProvider trait
pub trait DataProvider {
    fn get_list(
        &self,
        resource: Resource,
        params: GetListParams,
    ) -> Pin<Box<dyn Future<Output = Result<GetListResult>> + '_>>;

    fn get_one(
        &self,
        resource: Resource,
        params: GetOneParams,
    ) -> Pin<Box<dyn Future<Output = Result<GetOneResult>> + '_>>;

    fn get_many(
        &self,
        resource: Resource,
        params: GetManyParams,
    ) -> Pin<Box<dyn Future<Output = Result<GetManyResult>> + '_>>;

    fn get_many_reference(
        &self,
        resource: Resource,
        params: GetManyReferenceParams,
    ) -> Pin<Box<dyn Future<Output = Result<GetManyReferenceResult>> + '_>>;

    fn update(
        &self,
        resource: Resource,
        params: UpdateParams,
    ) -> Pin<Box<dyn Future<Output = Result<UpdateResult>> + '_>>;

    fn update_many(
        &self,
        resource: Resource,
        params: UpdateManyParams,
    ) -> Pin<Box<dyn Future<Output = Result<UpdateManyResult>> + '_>>;

    fn create(
        &self,
        resource: Resource,
        params: CreateParams,
    ) -> Pin<Box<dyn Future<Output = Result<CreateResult>> + '_>>;

    fn delete(
        &self,
        resource: Resource,
        params: DeleteParams,
    ) -> Pin<Box<dyn Future<Output = Result<DeleteResult>> + '_>>;

    fn delete_many(
        &self,
        resource: Resource,
        params: DeleteManyParams,
    ) -> Pin<Box<dyn Future<Output = Result<DeleteManyResult>> + '_>>;
}

// GetListParams struct
pub struct GetListParams {
    pub pagination: Option<PaginationPayload>,
    pub sort: Option<SortPayload>,
    pub filter: Option<FilterPayload>,
    pub meta: Option<Meta>,
}

// GetListResult struct
pub struct GetListResult {
    pub data: Vec<Record>,
    pub total: Option<usize>,
    pub page_info: Option<PageInfo>,
    pub meta: Option<Meta>,
}

// GetOneParams struct
pub struct GetOneParams {
    pub id: Identifier,
    pub meta: Option<Meta>,
}

// GetOneResult struct
pub struct GetOneResult {
    pub data: Record,
}

// GetManyParams struct
pub struct GetManyParams {
    pub ids: Vec<Identifier>,
    pub meta: Option<Meta>,
}

// GetManyResult struct
pub struct GetManyResult {
    pub data: Vec<Record>,
}

// GetManyReferenceParams struct
pub struct GetManyReferenceParams {
    pub target: String,
    pub id: Identifier,
    pub pagination: PaginationPayload,
    pub sort: SortPayload,
    pub filter: FilterPayload,
    pub meta: Option<Meta>,
}

// GetManyReferenceResult struct
pub struct GetManyReferenceResult {
    pub data: Vec<Record>,
    pub total: Option<usize>,
    pub page_info: Option<PageInfo>,
    pub meta: Option<Meta>,
}

// UpdateParams struct
pub struct UpdateParams {
    pub id: Identifier,
    pub data: HashMap<String, serde_json::Value>,
    pub previous_data: Record,
    pub meta: Option<Meta>,
}

// UpdateResult struct
pub struct UpdateResult {
    pub data: Record,
}

// UpdateManyParams struct
pub struct UpdateManyParams {
    pub ids: Vec<Identifier>,
    pub data: HashMap<String, serde_json::Value>,
    pub meta: Option<Meta>,
}

// UpdateManyResult struct
pub struct UpdateManyResult {
    pub data: Vec<Identifier>,
}

// CreateParams struct
pub struct CreateParams {
    pub data: HashMap<String, serde_json::Value>,
    pub meta: Option<Meta>,
}

// CreateResult struct
pub struct CreateResult {
    pub data: Record,
}

// DeleteParams struct
pub struct DeleteParams {
    pub id: Identifier,
    pub previous_data: Option<Record>,
    pub meta: Option<Meta>,
}

// DeleteResult struct
pub struct DeleteResult {
    pub data: Record,
}

// DeleteManyParams struct
pub struct DeleteManyParams {
    pub ids: Vec<Identifier>,
    pub meta: Option<Meta>,
}

// DeleteManyResult struct
pub struct DeleteManyResult {
    pub data: Vec<Identifier>,
}

// PageInfo struct
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
}
