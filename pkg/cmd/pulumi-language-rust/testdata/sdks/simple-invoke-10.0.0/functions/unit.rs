#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod unit {
    #[allow(dead_code)]
    pub struct UnitResult {
        pub result: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> UnitResult {
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "simple-invoke:index:unit".into(),
            version: super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        UnitResult {
            result: o.get_field("result"),
        }
    }
}
