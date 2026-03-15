#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod call_prefixed {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CallPrefixedArgs {
        #[builder(into)]
        pub self_: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::providers::Call,
        >,
        #[builder(into)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CallPrefixedResult {
        pub result: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: CallPrefixedArgs,
    ) -> CallPrefixedResult {
        let self__binding = args.self_.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "pulumi:providers:call/prefixed".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "__self__".into(),
                    value: &self__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        CallPrefixedResult {
            result: o.get_field("result"),
        }
    }
}
