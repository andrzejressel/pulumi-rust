#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod resource_call {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceCallArgs {
        #[builder(into)]
        pub self_: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::super::types::mod_Resource::Resource,
        >,
        #[builder(into)]
        pub input: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceCallResult {
        pub output: pulumi_gestalt_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: ResourceCallArgs,
    ) -> ResourceCallResult {
        let self__binding = args.self_.get_output(context);
        let input_binding = args.input.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "module-format:mod/nested_Resource:Resource/call".into(),
            version: super::super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "__self__".into(),
                    value: &self__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "input".into(),
                    value: &input_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        ResourceCallResult {
            output: o.get_field("output"),
        }
    }
}
