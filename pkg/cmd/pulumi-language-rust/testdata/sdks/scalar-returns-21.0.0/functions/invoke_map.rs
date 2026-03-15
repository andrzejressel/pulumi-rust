#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod invoke_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InvokeMapArgs {
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context, args: InvokeMapArgs) {
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "scalar-returns:index:invokeMap".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: &value_binding.drop_type(),
                },
            ],
        };
        context.invoke_resource(request);
    }
}
