#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VariantTwo {
    #[builder(into)]
    #[serde(rename = "discriminantKind")]
    pub r#discriminant_kind: String,
    #[builder(into)]
    #[serde(rename = "field2")]
    pub r#field_2: Option<String>,
}
