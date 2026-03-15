#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod do_hello_world {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DoHelloWorldArgs {
        #[builder(into)]
        pub input: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DoHelloWorldResult {
        pub output: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: DoHelloWorldArgs,
    ) -> DoHelloWorldResult {
        let input_binding = args.input.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "hipackage:index:doHelloWorld".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "input".into(),
                    value: &input_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        DoHelloWorldResult {
            output: o.get_field("output"),
        }
    }
}
