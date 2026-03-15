#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InnerData {
    #[builder(into)]
    #[serde(rename = "boolArray")]
    pub r#bool_array: Vec<bool>,
    #[builder(into)]
    #[serde(rename = "boolean")]
    pub r#boolean: bool,
    #[builder(into)]
    #[serde(rename = "float")]
    pub r#float: f64,
    #[builder(into)]
    #[serde(rename = "integer")]
    pub r#integer: i32,
    #[builder(into)]
    #[serde(rename = "string")]
    pub r#string: String,
    #[builder(into)]
    #[serde(rename = "stringMap")]
    pub r#string_map: std::collections::HashMap<String, String>,
}
