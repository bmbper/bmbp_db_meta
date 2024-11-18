use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcUserMeta {
    pub name: String,
    pub comment: String,
    pub user_password: String,
    pub schema: Option<Vec<RdbcSchemaMeta>>,
    pub role: Option<Vec<RdbcRoleMeta>>,
    pub grant: Option<Vec<RdbcGrant>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcRoleMeta {
    pub name: String,
    pub comment: String,
    pub grants: Option<Vec<RdbcGrant>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcGrant {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcSchemaMeta {
    pub schema: String,
    pub owner: String,
    pub comment: String,
    pub catalog: String,
    pub charset: String,
    pub table: Option<Vec<RdbcTableMeta>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcTableMeta {
    pub schema: String,
    pub owner: String,
    pub name: String,
    pub comment: String,
    pub charset: String,
    pub engine: String,
    pub column: Option<Vec<RdbcColumnMeta>>,
    pub index: Option<Vec<RdbcIndexMeta>>,
    pub constraint: Option<Vec<RdbcConstraintMeta>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcColumnMeta {
    pub owner: String,
    pub schema: String,
    pub table: String,
    pub name: String,
    pub comment: String,
    pub data_type: String,
    pub length: Option<u32>,
    pub scale: Option<u32>,
    pub default: Option<String>,
    pub primary: bool,
    pub unique: bool,
    pub nullable: bool,
    pub auto_increment: bool,
    pub zerofill: bool,
    pub charset: String,
    pub order_rule: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcIndexMeta {
    pub name: String,
    pub comment: String,
    pub column: Vec<RdbcColumnMeta>,
    pub unique: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcConstraintMeta {
    pub name: String,
    pub comment: String,
    pub kind: ConstraintKind,
    pub column: Vec<RdbcColumnMeta>,
    pub script: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ConstraintKind {
    PrimaryKey,
    Unique,
    Check,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcForeignKeyMeta {
    pub name: String,
    pub comment: String,
    pub column: RdbcColumnMeta,
    pub target_table: String,
    pub target_unique: RdbcConstraintMeta,
    pub on_delete: Vec<ForeignKeyCascadeKind>,
    pub on_update: Vec<ForeignKeyCascadeKind>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ForeignKeyCascadeKind {
    Restrict,
    Cascade,
    SetNull,
    NoAction,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcProducerMeta {
    pub name: String,
    pub comment: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcFuncMeta {
    pub name: String,
    pub comment: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcTriggerMeta {
    pub name: String,
    pub comment: String,
    pub trigger_kind: TriggerKind,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TriggerKind {
    Before,
    After,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcViewMeta {
    pub name: String,
    pub comment: String,
    pub sql: String,
}

pub type RdbcMetaResp<T> = Result<T, RdbcMetaError>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcMetaError {
    kind: MetaErrorKind,
    msg: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MetaErrorKind {
    ConnectError,
    QueryError,
    ParseError,
    Other,
}
