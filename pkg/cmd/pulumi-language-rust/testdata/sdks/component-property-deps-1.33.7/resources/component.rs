/// A component resource that accepts a list of resources. The construct request's property dependencies are returned as an output.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod component {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComponentArgs {
        #[builder(into)]
        pub resource: pulumi_gestalt_rust::InputOrOutput<super::types::Custom>,
        #[builder(into)]
        pub resource_list: pulumi_gestalt_rust::InputOrOutput<Vec<super::types::Custom>>,
        #[builder(into)]
        pub resource_map: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, super::types::Custom>,
        >,
    }
    #[allow(dead_code)]
    pub struct ComponentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub property_deps: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, Vec<String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentArgs,
    ) -> ComponentResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ComponentResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ComponentResult {
        let resource_binding = args.resource.get_output(context);
        let resource_list_binding = args.resource_list.get_output(context);
        let resource_map_binding = args.resource_map.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "component-property-deps:index:Component".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
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
            options,
        };
        let o = context.register_resource(request);
        ComponentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            property_deps: o.get_field("propertyDeps"),
        }
    }
}
