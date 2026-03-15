#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod component_refs {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComponentRefsArgs {
        #[builder(into)]
        pub self_: pulumi_gestalt_rust::InputOrOutput<super::super::types::Component>,
        #[builder(into)]
        pub resource: pulumi_gestalt_rust::InputOrOutput<super::super::types::Custom>,
        #[builder(into)]
        pub resource_list: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::Custom>,
        >,
        #[builder(into)]
        pub resource_map: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, super::super::types::Custom>,
        >,
    }
    #[allow(dead_code)]
    pub struct ComponentRefsResult {
        pub result: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, Vec<String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: ComponentRefsArgs,
    ) -> ComponentRefsResult {
        let self__binding = args.self_.get_output(context);
        let resource_binding = args.resource.get_output(context);
        let resource_list_binding = args.resource_list.get_output(context);
        let resource_map_binding = args.resource_map.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "component-property-deps:index:Component/refs".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "__self__".into(),
                    value: &self__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resource".into(),
                    value: &resource_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceList".into(),
                    value: &resource_list_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceMap".into(),
                    value: &resource_map_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        ComponentRefsResult {
            result: o.get_field("result"),
        }
    }
}
