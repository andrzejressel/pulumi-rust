#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod concat_world {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConcatWorldArgs {
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConcatWorldResult {
        pub result: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: ConcatWorldArgs,
    ) -> ConcatWorldResult {
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "module-format:mod/nested_concatWorld:concatWorld".into(),
            version: super::super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: &value_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        ConcatWorldResult {
            result: o.get_field("result"),
        }
    }
}
