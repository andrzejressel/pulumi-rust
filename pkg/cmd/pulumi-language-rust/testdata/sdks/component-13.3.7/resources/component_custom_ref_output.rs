/// A component resource that accepts an input that is used to create a child custom resource. A reference to this child custom resource is returned.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod component_custom_ref_output {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComponentCustomRefOutputArgs {
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ComponentCustomRefOutputResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub ref_: pulumi_gestalt_rust::Output<super::types::Custom>,
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentCustomRefOutputArgs,
    ) -> ComponentCustomRefOutputResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentCustomRefOutputArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ComponentCustomRefOutputResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentCustomRefOutputArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ComponentCustomRefOutputResult {
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "component:index:ComponentCustomRefOutput".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: &value_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        ComponentCustomRefOutputResult {
            id: o.get_id(),
            urn: o.get_urn(),
            ref_: o.get_field("ref"),
            value: o.get_field("value"),
        }
    }
}
