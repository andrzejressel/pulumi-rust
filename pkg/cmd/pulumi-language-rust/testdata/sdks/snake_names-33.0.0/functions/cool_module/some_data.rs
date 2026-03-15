#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod some_data {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SomeDataArgs {
        #[builder(into)]
        pub nested: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::cool_module::Entry>,
        >,
        #[builder(into)]
        pub the_input: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SomeDataResult {
        pub nested_output: pulumi_gestalt_rust::Output<
            Vec<
                std::collections::HashMap<
                    String,
                    super::super::super::types::cool_module::Entry,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: SomeDataArgs,
    ) -> SomeDataResult {
        let nested_binding = args.nested.get_output(context);
        let the_input_binding = args.the_input.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "snake_names:cool_module:some_data".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nested".into(),
                    value: &nested_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "the_input".into(),
                    value: &the_input_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        SomeDataResult {
            nested_output: o.get_field("nested_output"),
        }
    }
}
