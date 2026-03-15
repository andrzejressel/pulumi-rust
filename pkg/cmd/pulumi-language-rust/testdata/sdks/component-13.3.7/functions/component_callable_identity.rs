#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod component_callable_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComponentCallableIdentityArgs {
        #[builder(into)]
        pub self_: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ComponentCallable,
        >,
    }
    #[allow(dead_code)]
    pub struct ComponentCallableIdentityResult {
        pub result: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: ComponentCallableIdentityArgs,
    ) -> ComponentCallableIdentityResult {
        let self__binding = args.self_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "component:index:ComponentCallable/identity".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "__self__".into(),
                    value: &self__binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        ComponentCallableIdentityResult {
            result: o.get_field("result"),
        }
    }
}
