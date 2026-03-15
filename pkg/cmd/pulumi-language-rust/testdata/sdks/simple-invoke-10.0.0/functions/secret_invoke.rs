#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod secret_invoke {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretInvokeArgs {
        #[builder(into)]
        pub secret_response: pulumi_gestalt_rust::InputOrOutput<bool>,
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecretInvokeResult {
        pub response: pulumi_gestalt_rust::Output<String>,
        pub secret: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: SecretInvokeArgs,
    ) -> SecretInvokeResult {
        let secret_response_binding = args.secret_response.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "simple-invoke:index:secretInvoke".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretResponse".into(),
                    value: &secret_response_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: &value_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        SecretInvokeResult {
            response: o.get_field("response"),
            secret: o.get_field("secret"),
        }
    }
}
