#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfiguration {
    #[doc = "All rules are defined here."]
    pub rules: Vec<OrganizeRuleConfigurationRulesItem>,
}
impl From<&Self> for OrganizeRuleConfiguration {
    fn from(value: &Self) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfiguration {
    #[must_use]
    pub fn builder() -> builder::OrganizeRuleConfiguration {
        builder::OrganizeRuleConfiguration::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItem {
    pub actions: Vec<OrganizeRuleConfigurationRulesItemActionsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_mode: Option<OrganizeRuleConfigurationRulesItemFilterMode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<OrganizeRuleConfigurationRulesItemFiltersItem>,
    pub locations: OrganizeRuleConfigurationRulesItemLocations,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subfolders: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<OrganizeRuleConfigurationRulesItemTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<OrganizeRuleConfigurationRulesItemTargets>,
}
impl From<&OrganizeRuleConfigurationRulesItem> for OrganizeRuleConfigurationRulesItem {
    fn from(value: &OrganizeRuleConfigurationRulesItem) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItem {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItem {
        builder::OrganizeRuleConfigurationRulesItem::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrganizeRuleConfigurationRulesItemActionsItem {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype1>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype3>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_4: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype4>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_5: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype5>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_6: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype6>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_7: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype7>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_8: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype8>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_9: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype9>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_10: Option<serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_11: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype11>,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItem>
    for OrganizeRuleConfigurationRulesItemActionsItem
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItem) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItem {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItem {
        builder::OrganizeRuleConfigurationRulesItemActionsItem::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype0 {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1>,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype0>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype0) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype0 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype0 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype0::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1 {
    pub confirm: OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1Confirm,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1Confirm {
    Variant0(String),
    Variant1 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        msg: Option<String>,
    },
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1Confirm>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1Confirm
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1Confirm) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype1 {
    pub copy: OrganizeRuleConfigurationRulesItemActionsItemSubtype1Copy,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype1>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype1
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype1) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype1 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype1 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype1::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype11 {
    pub write: OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype11>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype11
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype11) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype11 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype11 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype11::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clear_before_first_write: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub newline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub text: String,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode {
    #[serde(rename = "prepend")]
    Prepend,
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "overwrite")]
    Overwrite,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode {
    fn to_string(&self) -> String {
        match *self {
            Self::Prepend => "prepend".to_string(),
            Self::Append => "append".to_string(),
            Self::Overwrite => "overwrite".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "prepend" => Ok(Self::Prepend),
            "append" => Ok(Self::Append),
            "overwrite" => Ok(Self::Overwrite),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode
{
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype1Copy {
    Variant0(String),
    Variant1 {
        dest: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filesystem: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        on_conflict:
            Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rename_template: Option<String>,
    },
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype1Copy>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype1Copy
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype1Copy) -> Self {
        value.clone()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "trash")]
    Trash,
    #[serde(rename = "rename_new")]
    RenameNew,
    #[serde(rename = "rename_existing")]
    RenameExisting,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict,
    ) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict {
    fn to_string(&self) -> String {
        match *self {
            Self::Skip => "skip".to_string(),
            Self::Overwrite => "overwrite".to_string(),
            Self::Trash => "trash".to_string(),
            Self::RenameNew => "rename_new".to_string(),
            Self::RenameExisting => "rename_existing".to_string(),
        }
    }
}
impl std::str::FromStr
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict
{
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "skip" => Ok(Self::Skip),
            "overwrite" => Ok(Self::Overwrite),
            "trash" => Ok(Self::Trash),
            "rename_new" => Ok(Self::RenameNew),
            "rename_existing" => Ok(Self::RenameExisting),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype1CopyVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype3 {
    pub echo: String,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype3>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype3
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype3) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype3 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype3 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype3::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype4 {
    pub macos_tags: OrganizeRuleConfigurationRulesItemActionsItemSubtype4MacosTags,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype4>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype4
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype4) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype4 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype4 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype4::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype4MacosTags {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype4MacosTags>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype4MacosTags
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype4MacosTags) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for OrganizeRuleConfigurationRulesItemActionsItemSubtype4MacosTags {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype5 {
    #[serde(rename = "move")]
    pub move_: OrganizeRuleConfigurationRulesItemActionsItemSubtype5Move,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype5>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype5
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype5) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype5 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype5 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype5::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype5Move {
    Variant0(String),
    Variant1 {
        dest: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filesystem: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        on_conflict:
            Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rename_template: Option<String>,
    },
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype5Move>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype5Move
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype5Move) -> Self {
        value.clone()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "trash")]
    Trash,
    #[serde(rename = "rename_new")]
    RenameNew,
    #[serde(rename = "rename_existing")]
    RenameExisting,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict,
    ) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict {
    fn to_string(&self) -> String {
        match *self {
            Self::Skip => "skip".to_string(),
            Self::Overwrite => "overwrite".to_string(),
            Self::Trash => "trash".to_string(),
            Self::RenameNew => "rename_new".to_string(),
            Self::RenameExisting => "rename_existing".to_string(),
        }
    }
}
impl std::str::FromStr
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict
{
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "skip" => Ok(Self::Skip),
            "overwrite" => Ok(Self::Overwrite),
            "trash" => Ok(Self::Trash),
            "rename_new" => Ok(Self::RenameNew),
            "rename_existing" => Ok(Self::RenameExisting),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype5MoveVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype6 {
    pub python: OrganizeRuleConfigurationRulesItemActionsItemSubtype6Python,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype6>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype6
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype6) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype6 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype6 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype6::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype6Python {
    Variant0(String),
    Variant1 {
        code: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        run_in_simulation: Option<bool>,
    },
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype6Python>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype6Python
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype6Python) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype7 {
    pub rename: OrganizeRuleConfigurationRulesItemActionsItemSubtype7Rename,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype7>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype7
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype7) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype7 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype7 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype7::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype7Rename {
    Variant0(String),
    Variant1 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        on_conflict:
            Option<OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rename_template: Option<String>,
    },
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype7Rename>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype7Rename
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype7Rename) -> Self {
        value.clone()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "trash")]
    Trash,
    #[serde(rename = "rename_new")]
    RenameNew,
    #[serde(rename = "rename_existing")]
    RenameExisting,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict,
    ) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict {
    fn to_string(&self) -> String {
        match *self {
            Self::Skip => "skip".to_string(),
            Self::Overwrite => "overwrite".to_string(),
            Self::Trash => "trash".to_string(),
            Self::RenameNew => "rename_new".to_string(),
            Self::RenameExisting => "rename_existing".to_string(),
        }
    }
}
impl std::str::FromStr
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict
{
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "skip" => Ok(Self::Skip),
            "overwrite" => Ok(Self::Overwrite),
            "trash" => Ok(Self::Trash),
            "rename_new" => Ok(Self::RenameNew),
            "rename_existing" => Ok(Self::RenameExisting),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype7RenameVariant1OnConflict
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype8 {
    pub shell: OrganizeRuleConfigurationRulesItemActionsItemSubtype8Shell,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype8>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype8
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype8) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype8 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype8 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype8::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype8Shell {
    Variant0(String),
    Variant1 {
        cmd: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ignore_errors: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        run_in_simulation: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        simulation_output: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        simulation_returncode: Option<i64>,
    },
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype8Shell>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype8Shell
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype8Shell) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype9 {
    pub symlink: OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink,
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype9>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype9
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype9) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemActionsItemSubtype9 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype9 {
        builder::OrganizeRuleConfigurationRulesItemActionsItemSubtype9::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink {
    Variant0(String),
    Variant1(Vec<String>),
    Variant2(serde_json::Map<String, serde_json::Value>),
}
impl From<&OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink
{
    fn from(value: &OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self::Variant2(value)
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFilterMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "none")]
    None,
}
impl From<&OrganizeRuleConfigurationRulesItemFilterMode>
    for OrganizeRuleConfigurationRulesItemFilterMode
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFilterMode) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFilterMode {
    fn to_string(&self) -> String {
        match *self {
            Self::All => "all".to_string(),
            Self::Any => "any".to_string(),
            Self::None => "none".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFilterMode {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "all" => Ok(Self::All),
            "any" => Ok(Self::Any),
            "none" => Ok(Self::None),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFilterMode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OrganizeRuleConfigurationRulesItemFilterMode {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OrganizeRuleConfigurationRulesItemFilterMode {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrganizeRuleConfigurationRulesItemFiltersItem {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype1>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype2>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype3>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_4: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype4>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_5: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype5>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_6: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype6>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_7: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype7>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_8: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype8>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_9: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype9>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_10: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype10>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_11: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype11>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_12: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype12>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_13: Option<OrganizeRuleConfigurationRulesItemFiltersItemSubtype13>,
}
impl From<&Self> for OrganizeRuleConfigurationRulesItemFiltersItem {
    fn from(value: &Self) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemFiltersItem {
    #[must_use]
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemFiltersItem {
        builder::OrganizeRuleConfigurationRulesItemFiltersItem::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype0 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype0) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype0
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Matches files / folders by created date"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0 {
    #[serde(rename = "not created")]
    NotCreated,
    #[serde(rename = "created")]
    Created,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotCreated => "not created".to_string(),
            Self::Created => "created".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not created" => Ok(Self::NotCreated),
            "created" => Ok(Self::Created),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype0Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype1 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype1>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype1
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype1) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype1
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype10 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype10>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype10
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype10) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype10
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Matches files by last modified date"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0 {
    #[serde(rename = "not lastmodified")]
    NotLastmodified,
    #[serde(rename = "lastmodified")]
    Lastmodified,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotLastmodified => "not lastmodified".to_string(),
            Self::Lastmodified => "lastmodified".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not lastmodified" => Ok(Self::NotLastmodified),
            "lastmodified" => Ok(Self::Lastmodified),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0
{
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype10Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype11 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype11>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype11
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype11) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype11
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Filter by macOS tags"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0 {
    #[serde(rename = "not macos_tags")]
    NotMacosTags,
    #[serde(rename = "macos_tags")]
    MacosTags,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotMacosTags => "not macos_tags".to_string(),
            Self::MacosTags => "macos_tags".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not macos_tags" => Ok(Self::NotMacosTags),
            "macos_tags" => Ok(Self::MacosTags),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0
{
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype11Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype12 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype12>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype12
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype12) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype12
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Filter by MIME type associated with the file extension."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0 {
    #[serde(rename = "not mimetype")]
    NotMimetype,
    #[serde(rename = "mimetype")]
    Mimetype,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotMimetype => "not mimetype".to_string(),
            Self::Mimetype => "mimetype".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not mimetype" => Ok(Self::NotMimetype),
            "mimetype" => Ok(Self::Mimetype),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0
{
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype12Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizeRuleConfigurationRulesItemFiltersItemSubtype13 {}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype13>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype13
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype13) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemFiltersItemSubtype13 {
    pub fn builder() -> builder::OrganizeRuleConfigurationRulesItemFiltersItemSubtype13 {
        builder::OrganizeRuleConfigurationRulesItemFiltersItemSubtype13::default()
    }
}
#[doc = "Matches files by the time the file was added to a folder."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0 {
    #[serde(rename = "not date_added")]
    NotDateAdded,
    #[serde(rename = "date_added")]
    DateAdded,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotDateAdded => "not date_added".to_string(),
            Self::DateAdded => "date_added".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not date_added" => Ok(Self::NotDateAdded),
            "date_added" => Ok(Self::DateAdded),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype1Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype2 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype2>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype2
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype2) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype2
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Matches files by the time the file was last used."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0 {
    #[serde(rename = "not date_lastused")]
    NotDateLastused,
    #[serde(rename = "date_lastused")]
    DateLastused,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotDateLastused => "not date_lastused".to_string(),
            Self::DateLastused => "date_lastused".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not date_lastused" => Ok(Self::NotDateLastused),
            "date_lastused" => Ok(Self::DateLastused),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype2Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype3 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype3>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype3
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype3) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype3
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "A fast duplicate file finder."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0 {
    #[serde(rename = "not duplicate")]
    NotDuplicate,
    #[serde(rename = "duplicate")]
    Duplicate,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotDuplicate => "not duplicate".to_string(),
            Self::Duplicate => "duplicate".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not duplicate" => Ok(Self::NotDuplicate),
            "duplicate" => Ok(Self::Duplicate),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype3Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype4 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype4>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype4
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype4) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype4
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Filter by image EXIF data"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0 {
    #[serde(rename = "not exif")]
    NotExif,
    #[serde(rename = "exif")]
    Exif,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotExif => "not exif".to_string(),
            Self::Exif => "exif".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not exif" => Ok(Self::NotExif),
            "exif" => Ok(Self::Exif),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype4Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype5 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype5>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype5
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype5) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype5
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Filter by file extension"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0 {
    #[serde(rename = "not extension")]
    NotExtension,
    #[serde(rename = "extension")]
    Extension,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotExtension => "not extension".to_string(),
            Self::Extension => "extension".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not extension" => Ok(Self::NotExtension),
            "extension" => Ok(Self::Extension),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype5Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype6 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype6>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype6
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype6) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype6
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Matches file content with the given regular expression"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0 {
    #[serde(rename = "not filecontent")]
    NotFilecontent,
    #[serde(rename = "filecontent")]
    Filecontent,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotFilecontent => "not filecontent".to_string(),
            Self::Filecontent => "filecontent".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not filecontent" => Ok(Self::NotFilecontent),
            "filecontent" => Ok(Self::Filecontent),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype6Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype7 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype7>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype7
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype7) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype7
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Calculates the hash of a file."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0 {
    #[serde(rename = "not hash")]
    NotHash,
    #[serde(rename = "hash")]
    Hash,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotHash => "not hash".to_string(),
            Self::Hash => "hash".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not hash" => Ok(Self::NotHash),
            "hash" => Ok(Self::Hash),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype7Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype8 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype8>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype8
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype8) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype8
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Match files and folders by name"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0 {
    #[serde(rename = "not name")]
    NotName,
    #[serde(rename = "name")]
    Name,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotName => "not name".to_string(),
            Self::Name => "name".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not name" => Ok(Self::NotName),
            "name" => Ok(Self::Name),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype8Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype9 {
    Variant0(OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0),
    Variant1 {},
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype9>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype9
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype9) -> Self {
        value.clone()
    }
}
impl From<OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype9
{
    fn from(value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "Matches files and folders by size"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0 {
    #[serde(rename = "not size")]
    NotSize,
    #[serde(rename = "size")]
    Size,
}
impl From<&OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0
{
    fn from(value: &OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0 {
    fn to_string(&self) -> String {
        match *self {
            Self::NotSize => "not size".to_string(),
            Self::Size => "size".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "not size" => Ok(Self::NotSize),
            "size" => Ok(Self::Size),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemFiltersItemSubtype9Variant0
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemLocations {
    Variant0(String),
    Variant1(Vec<OrganizeRuleConfigurationRulesItemLocationsVariant1Item>),
}
impl From<&OrganizeRuleConfigurationRulesItemLocations>
    for OrganizeRuleConfigurationRulesItemLocations
{
    fn from(value: &OrganizeRuleConfigurationRulesItemLocations) -> Self {
        value.clone()
    }
}
impl From<Vec<OrganizeRuleConfigurationRulesItemLocationsVariant1Item>>
    for OrganizeRuleConfigurationRulesItemLocations
{
    fn from(value: Vec<OrganizeRuleConfigurationRulesItemLocationsVariant1Item>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OrganizeRuleConfigurationRulesItemLocationsVariant1Item {
    Variant0(String),
    Variant1 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        exclude_dirs:
            Option<OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeDirs>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        exclude_files:
            Option<OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeFiles>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filesystem: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filter: Option<OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Filter>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filter_dirs:
            Option<OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1FilterDirs>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ignore_errors: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_depth: Option<OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth>,
        path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        search: Option<OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        system_exclude_dirs: Option<
            OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeDirs,
        >,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        system_exclude_files: Option<
            OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeFiles,
        >,
    },
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1Item>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1Item
{
    fn from(value: &OrganizeRuleConfigurationRulesItemLocationsVariant1Item) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeDirs {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeDirs>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeDirs
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeDirs,
    ) -> Self {
        value.clone()
    }
}
impl From<Vec<String>>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeDirs
{
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeFiles {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeFiles>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeFiles
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeFiles,
    ) -> Self {
        value.clone()
    }
}
impl From<Vec<String>>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1ExcludeFiles
{
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Filter {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Filter>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Filter
{
    fn from(value: &OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Filter) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Filter {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1FilterDirs {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1FilterDirs>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1FilterDirs
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1FilterDirs,
    ) -> Self {
        value.clone()
    }
}
impl From<Vec<String>>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1FilterDirs
{
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<i64>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<serde_json::Value>,
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth,
    ) -> Self {
        value.clone()
    }
}
impl OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth {
    pub fn builder(
    ) -> builder::OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth {
        builder::OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search {
    #[serde(rename = "depth")]
    Depth,
    #[serde(rename = "breadth")]
    Breadth,
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search
{
    fn from(value: &OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search {
    fn to_string(&self) -> String {
        match *self {
            Self::Depth => "depth".to_string(),
            Self::Breadth => "breadth".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "depth" => Ok(Self::Depth),
            "breadth" => Ok(Self::Breadth),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search
{
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search
{
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1Search
{
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeDirs {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeDirs>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeDirs
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeDirs,
    ) -> Self {
        value.clone()
    }
}
impl From<Vec<String>>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeDirs
{
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeFiles {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeFiles>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeFiles
{
    fn from(
        value: &OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeFiles,
    ) -> Self {
        value.clone()
    }
}
impl From<Vec<String>>
    for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1SystemExcludeFiles
{
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrganizeRuleConfigurationRulesItemTags {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&OrganizeRuleConfigurationRulesItemTags> for OrganizeRuleConfigurationRulesItemTags {
    fn from(value: &OrganizeRuleConfigurationRulesItemTags) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for OrganizeRuleConfigurationRulesItemTags {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganizeRuleConfigurationRulesItemTargets {
    #[serde(rename = "dirs")]
    Dirs,
    #[serde(rename = "files")]
    Files,
}
impl From<&OrganizeRuleConfigurationRulesItemTargets>
    for OrganizeRuleConfigurationRulesItemTargets
{
    fn from(value: &OrganizeRuleConfigurationRulesItemTargets) -> Self {
        value.clone()
    }
}
impl ToString for OrganizeRuleConfigurationRulesItemTargets {
    fn to_string(&self) -> String {
        match *self {
            Self::Dirs => "dirs".to_string(),
            Self::Files => "files".to_string(),
        }
    }
}
impl std::str::FromStr for OrganizeRuleConfigurationRulesItemTargets {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "dirs" => Ok(Self::Dirs),
            "files" => Ok(Self::Files),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganizeRuleConfigurationRulesItemTargets {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OrganizeRuleConfigurationRulesItemTargets {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OrganizeRuleConfigurationRulesItemTargets {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfiguration {
        rules: Result<Vec<super::OrganizeRuleConfigurationRulesItem>, String>,
    }
    impl Default for OrganizeRuleConfiguration {
        fn default() -> Self {
            Self {
                rules: Err("no value supplied for rules".to_string()),
            }
        }
    }
    impl OrganizeRuleConfiguration {
        pub fn rules<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::OrganizeRuleConfigurationRulesItem>>,
            T::Error: std::fmt::Display,
        {
            self.rules = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rules: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfiguration> for super::OrganizeRuleConfiguration {
        type Error = String;
        fn try_from(value: OrganizeRuleConfiguration) -> Result<Self, String> {
            Ok(Self {
                rules: value.rules?,
            })
        }
    }
    impl From<super::OrganizeRuleConfiguration> for OrganizeRuleConfiguration {
        fn from(value: super::OrganizeRuleConfiguration) -> Self {
            Self {
                rules: Ok(value.rules),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItem {
        actions: Result<Vec<super::OrganizeRuleConfigurationRulesItemActionsItem>, String>,
        enabled: Result<Option<bool>, String>,
        filter_mode: Result<Option<super::OrganizeRuleConfigurationRulesItemFilterMode>, String>,
        filters: Result<Vec<super::OrganizeRuleConfigurationRulesItemFiltersItem>, String>,
        locations: Result<super::OrganizeRuleConfigurationRulesItemLocations, String>,
        name: Result<Option<String>, String>,
        subfolders: Result<Option<bool>, String>,
        tags: Result<Option<super::OrganizeRuleConfigurationRulesItemTags>, String>,
        targets: Result<Option<super::OrganizeRuleConfigurationRulesItemTargets>, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItem {
        fn default() -> Self {
            Self {
                actions: Err("no value supplied for actions".to_string()),
                enabled: Ok(Default::default()),
                filter_mode: Ok(Default::default()),
                filters: Ok(Default::default()),
                locations: Err("no value supplied for locations".to_string()),
                name: Ok(Default::default()),
                subfolders: Ok(Default::default()),
                tags: Ok(Default::default()),
                targets: Ok(Default::default()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItem {
        pub fn actions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::OrganizeRuleConfigurationRulesItemActionsItem>>,
            T::Error: std::fmt::Display,
        {
            self.actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actions: {}", e));
            self
        }
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {}", e));
            self
        }
        pub fn filter_mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::OrganizeRuleConfigurationRulesItemFilterMode>>,
            T::Error: std::fmt::Display,
        {
            self.filter_mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filter_mode: {}", e));
            self
        }
        pub fn filters<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::OrganizeRuleConfigurationRulesItemFiltersItem>>,
            T::Error: std::fmt::Display,
        {
            self.filters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filters: {}", e));
            self
        }
        pub fn locations<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OrganizeRuleConfigurationRulesItemLocations>,
            T::Error: std::fmt::Display,
        {
            self.locations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for locations: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn subfolders<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.subfolders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subfolders: {}", e));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::OrganizeRuleConfigurationRulesItemTags>>,
            T::Error: std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
        pub fn targets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::OrganizeRuleConfigurationRulesItemTargets>>,
            T::Error: std::fmt::Display,
        {
            self.targets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for targets: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItem>
        for super::OrganizeRuleConfigurationRulesItem
    {
        type Error = String;
        fn try_from(value: OrganizeRuleConfigurationRulesItem) -> Result<Self, String> {
            Ok(Self {
                actions: value.actions?,
                enabled: value.enabled?,
                filter_mode: value.filter_mode?,
                filters: value.filters?,
                locations: value.locations?,
                name: value.name?,
                subfolders: value.subfolders?,
                tags: value.tags?,
                targets: value.targets?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItem> for OrganizeRuleConfigurationRulesItem {
        fn from(value: super::OrganizeRuleConfigurationRulesItem) -> Self {
            Self {
                actions: Ok(value.actions),
                enabled: Ok(value.enabled),
                filter_mode: Ok(value.filter_mode),
                filters: Ok(value.filters),
                locations: Ok(value.locations),
                name: Ok(value.name),
                subfolders: Ok(value.subfolders),
                tags: Ok(value.tags),
                targets: Ok(value.targets),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItem {
        subtype_0:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0>, String>,
        subtype_1:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype1>, String>,
        subtype_2: Result<Option<serde_json::Value>, String>,
        subtype_3:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype3>, String>,
        subtype_4:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype4>, String>,
        subtype_5:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype5>, String>,
        subtype_6:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype6>, String>,
        subtype_7:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype7>, String>,
        subtype_8:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype8>, String>,
        subtype_9:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype9>, String>,
        subtype_10: Result<Option<serde_json::Value>, String>,
        subtype_11:
            Result<Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11>, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItem {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
                subtype_5: Ok(Default::default()),
                subtype_6: Ok(Default::default()),
                subtype_7: Ok(Default::default()),
                subtype_8: Ok(Default::default()),
                subtype_9: Ok(Default::default()),
                subtype_10: Ok(Default::default()),
                subtype_11: Ok(Default::default()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItem {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype1>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype3>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype4>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {}", e));
            self
        }
        pub fn subtype_5<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype5>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_5: {}", e));
            self
        }
        pub fn subtype_6<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype6>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_6: {}", e));
            self
        }
        pub fn subtype_7<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype7>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_7: {}", e));
            self
        }
        pub fn subtype_8<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype8>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_8: {}", e));
            self
        }
        pub fn subtype_9<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype9>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_9 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_9: {}", e));
            self
        }
        pub fn subtype_10<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_10 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_10: {}", e));
            self
        }
        pub fn subtype_11<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_11 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_11: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItem>
        for super::OrganizeRuleConfigurationRulesItemActionsItem
    {
        type Error = String;
        fn try_from(value: OrganizeRuleConfigurationRulesItemActionsItem) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
                subtype_5: value.subtype_5?,
                subtype_6: value.subtype_6?,
                subtype_7: value.subtype_7?,
                subtype_8: value.subtype_8?,
                subtype_9: value.subtype_9?,
                subtype_10: value.subtype_10?,
                subtype_11: value.subtype_11?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItem>
        for OrganizeRuleConfigurationRulesItemActionsItem
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItem) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
                subtype_5: Ok(value.subtype_5),
                subtype_6: Ok(value.subtype_6),
                subtype_7: Ok(value.subtype_7),
                subtype_8: Ok(value.subtype_8),
                subtype_9: Ok(value.subtype_9),
                subtype_10: Ok(value.subtype_10),
                subtype_11: Ok(value.subtype_11),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype0 {
        subtype_0: Result<Option<serde_json::Value>, String>,
        subtype_1: Result<
            Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1>,
            String,
        >,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype0 {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype0 {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype0>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype0,
        ) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype0
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1 {
        confirm: Result<
            super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1Confirm,
            String,
        >,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1 {
        fn default() -> Self {
            Self {
                confirm: Err("no value supplied for confirm".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1 {
        pub fn confirm<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1Confirm,
            >,
            T::Error: std::fmt::Display,
        {
            self.confirm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for confirm: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1,
        ) -> Result<Self, String> {
            Ok(Self {
                confirm: value.confirm?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1
    {
        fn from(
            value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype0Subtype1,
        ) -> Self {
            Self {
                confirm: Ok(value.confirm),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype1 {
        copy: Result<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype1Copy, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype1 {
        fn default() -> Self {
            Self {
                copy: Err("no value supplied for copy".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype1 {
        pub fn copy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype1Copy,
            >,
            T::Error: std::fmt::Display,
        {
            self.copy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for copy: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype1>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype1
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype1,
        ) -> Result<Self, String> {
            Ok(Self { copy: value.copy? })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype1>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype1
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype1) -> Self {
            Self {
                copy: Ok(value.copy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype11 {
        write: Result<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype11 {
        fn default() -> Self {
            Self {
                write: Err("no value supplied for write".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype11 {
        pub fn write<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write,
            >,
            T::Error: std::fmt::Display,
        {
            self.write = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for write: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype11>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype11,
        ) -> Result<Self, String> {
            Ok(Self {
                write: value.write?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype11
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11) -> Self {
            Self {
                write: Ok(value.write),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write {
        clear_before_first_write: Result<Option<bool>, String>,
        filesystem: Result<Option<String>, String>,
        mode: Result<
            Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode>,
            String,
        >,
        newline: Result<Option<bool>, String>,
        path: Result<Option<String>, String>,
        text: Result<String, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write {
        fn default() -> Self {
            Self {
                clear_before_first_write: Ok(Default::default()),
                filesystem: Ok(Default::default()),
                mode: Ok(Default::default()),
                newline: Ok(Default::default()),
                path: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write {
        pub fn clear_before_first_write<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.clear_before_first_write = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for clear_before_first_write: {}",
                    e
                )
            });
            self
        }
        pub fn filesystem<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.filesystem = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filesystem: {}", e));
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11WriteMode>,
            >,
            T::Error: std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {}", e));
            self
        }
        pub fn newline<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.newline = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for newline: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write,
        ) -> Result<Self, String> {
            Ok(Self {
                clear_before_first_write: value.clear_before_first_write?,
                filesystem: value.filesystem?,
                mode: value.mode?,
                newline: value.newline?,
                path: value.path?,
                text: value.text?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype11Write) -> Self {
            Self {
                clear_before_first_write: Ok(value.clear_before_first_write),
                filesystem: Ok(value.filesystem),
                mode: Ok(value.mode),
                newline: Ok(value.newline),
                path: Ok(value.path),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype3 {
        echo: Result<String, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype3 {
        fn default() -> Self {
            Self {
                echo: Err("no value supplied for echo".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype3 {
        pub fn echo<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.echo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for echo: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype3>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype3
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype3,
        ) -> Result<Self, String> {
            Ok(Self { echo: value.echo? })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype3>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype3
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype3) -> Self {
            Self {
                echo: Ok(value.echo),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype4 {
        macos_tags:
            Result<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype4MacosTags, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype4 {
        fn default() -> Self {
            Self {
                macos_tags: Err("no value supplied for macos_tags".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype4 {
        pub fn macos_tags<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype4MacosTags,
            >,
            T::Error: std::fmt::Display,
        {
            self.macos_tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for macos_tags: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype4>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype4
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype4,
        ) -> Result<Self, String> {
            Ok(Self {
                macos_tags: value.macos_tags?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype4>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype4
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype4) -> Self {
            Self {
                macos_tags: Ok(value.macos_tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype5 {
        move_: Result<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype5Move, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype5 {
        fn default() -> Self {
            Self {
                move_: Err("no value supplied for move_".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype5 {
        pub fn move_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype5Move,
            >,
            T::Error: std::fmt::Display,
        {
            self.move_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for move_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype5>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype5
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype5,
        ) -> Result<Self, String> {
            Ok(Self {
                move_: value.move_?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype5>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype5
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype5) -> Self {
            Self {
                move_: Ok(value.move_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype6 {
        python: Result<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype6Python, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype6 {
        fn default() -> Self {
            Self {
                python: Err("no value supplied for python".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype6 {
        pub fn python<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype6Python,
            >,
            T::Error: std::fmt::Display,
        {
            self.python = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for python: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype6>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype6
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype6,
        ) -> Result<Self, String> {
            Ok(Self {
                python: value.python?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype6>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype6
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype6) -> Self {
            Self {
                python: Ok(value.python),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype7 {
        rename: Result<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype7Rename, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype7 {
        fn default() -> Self {
            Self {
                rename: Err("no value supplied for rename".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype7 {
        pub fn rename<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype7Rename,
            >,
            T::Error: std::fmt::Display,
        {
            self.rename = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rename: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype7>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype7
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype7,
        ) -> Result<Self, String> {
            Ok(Self {
                rename: value.rename?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype7>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype7
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype7) -> Self {
            Self {
                rename: Ok(value.rename),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype8 {
        shell: Result<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype8Shell, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype8 {
        fn default() -> Self {
            Self {
                shell: Err("no value supplied for shell".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype8 {
        pub fn shell<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype8Shell,
            >,
            T::Error: std::fmt::Display,
        {
            self.shell = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shell: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype8>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype8
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype8,
        ) -> Result<Self, String> {
            Ok(Self {
                shell: value.shell?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype8>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype8
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype8) -> Self {
            Self {
                shell: Ok(value.shell),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemActionsItemSubtype9 {
        symlink:
            Result<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemActionsItemSubtype9 {
        fn default() -> Self {
            Self {
                symlink: Err("no value supplied for symlink".to_string()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemActionsItemSubtype9 {
        pub fn symlink<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                super::OrganizeRuleConfigurationRulesItemActionsItemSubtype9Symlink,
            >,
            T::Error: std::fmt::Display,
        {
            self.symlink = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symlink: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemActionsItemSubtype9>
        for super::OrganizeRuleConfigurationRulesItemActionsItemSubtype9
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemActionsItemSubtype9,
        ) -> Result<Self, String> {
            Ok(Self {
                symlink: value.symlink?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemActionsItemSubtype9>
        for OrganizeRuleConfigurationRulesItemActionsItemSubtype9
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemActionsItemSubtype9) -> Self {
            Self {
                symlink: Ok(value.symlink),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemFiltersItem {
        subtype_0:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype0>, String>,
        subtype_1:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype1>, String>,
        subtype_2:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype2>, String>,
        subtype_3:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype3>, String>,
        subtype_4:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype4>, String>,
        subtype_5:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype5>, String>,
        subtype_6:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype6>, String>,
        subtype_7:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype7>, String>,
        subtype_8:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype8>, String>,
        subtype_9:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype9>, String>,
        subtype_10:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype10>, String>,
        subtype_11:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype11>, String>,
        subtype_12:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype12>, String>,
        subtype_13:
            Result<Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype13>, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemFiltersItem {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
                subtype_5: Ok(Default::default()),
                subtype_6: Ok(Default::default()),
                subtype_7: Ok(Default::default()),
                subtype_8: Ok(Default::default()),
                subtype_9: Ok(Default::default()),
                subtype_10: Ok(Default::default()),
                subtype_11: Ok(Default::default()),
                subtype_12: Ok(Default::default()),
                subtype_13: Ok(Default::default()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemFiltersItem {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype0>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype1>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype2>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype3>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype4>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {}", e));
            self
        }
        pub fn subtype_5<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype5>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_5: {}", e));
            self
        }
        pub fn subtype_6<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype6>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_6: {}", e));
            self
        }
        pub fn subtype_7<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype7>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_7: {}", e));
            self
        }
        pub fn subtype_8<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype8>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_8: {}", e));
            self
        }
        pub fn subtype_9<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype9>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_9 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_9: {}", e));
            self
        }
        pub fn subtype_10<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype10>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_10 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_10: {}", e));
            self
        }
        pub fn subtype_11<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype11>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_11 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_11: {}", e));
            self
        }
        pub fn subtype_12<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype12>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_12 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_12: {}", e));
            self
        }
        pub fn subtype_13<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype13>,
            >,
            T::Error: std::fmt::Display,
        {
            self.subtype_13 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_13: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemFiltersItem>
        for super::OrganizeRuleConfigurationRulesItemFiltersItem
    {
        type Error = String;
        fn try_from(value: OrganizeRuleConfigurationRulesItemFiltersItem) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
                subtype_5: value.subtype_5?,
                subtype_6: value.subtype_6?,
                subtype_7: value.subtype_7?,
                subtype_8: value.subtype_8?,
                subtype_9: value.subtype_9?,
                subtype_10: value.subtype_10?,
                subtype_11: value.subtype_11?,
                subtype_12: value.subtype_12?,
                subtype_13: value.subtype_13?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemFiltersItem>
        for OrganizeRuleConfigurationRulesItemFiltersItem
    {
        fn from(value: super::OrganizeRuleConfigurationRulesItemFiltersItem) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
                subtype_5: Ok(value.subtype_5),
                subtype_6: Ok(value.subtype_6),
                subtype_7: Ok(value.subtype_7),
                subtype_8: Ok(value.subtype_8),
                subtype_9: Ok(value.subtype_9),
                subtype_10: Ok(value.subtype_10),
                subtype_11: Ok(value.subtype_11),
                subtype_12: Ok(value.subtype_12),
                subtype_13: Ok(value.subtype_13),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemFiltersItemSubtype13 {}
    impl Default for OrganizeRuleConfigurationRulesItemFiltersItemSubtype13 {
        fn default() -> Self {
            Self {}
        }
    }
    impl OrganizeRuleConfigurationRulesItemFiltersItemSubtype13 {}
    impl std::convert::TryFrom<OrganizeRuleConfigurationRulesItemFiltersItemSubtype13>
        for super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype13
    {
        type Error = String;
        fn try_from(
            _value: OrganizeRuleConfigurationRulesItemFiltersItemSubtype13,
        ) -> Result<Self, String> {
            Ok(Self {})
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype13>
        for OrganizeRuleConfigurationRulesItemFiltersItemSubtype13
    {
        fn from(_value: super::OrganizeRuleConfigurationRulesItemFiltersItemSubtype13) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth {
        subtype_0: Result<Option<i64>, String>,
        subtype_1: Result<Option<serde_json::Value>, String>,
    }
    impl Default for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl
        std::convert::TryFrom<
            OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth,
        > for super::OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth
    {
        type Error = String;
        fn try_from(
            value: OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth,
        ) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth>
        for OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth
    {
        fn from(
            value: super::OrganizeRuleConfigurationRulesItemLocationsVariant1ItemVariant1MaxDepth,
        ) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
}
