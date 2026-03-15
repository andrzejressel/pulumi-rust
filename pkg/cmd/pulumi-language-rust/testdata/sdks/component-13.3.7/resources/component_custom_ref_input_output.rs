/// A component resource that accepts a reference to a custom resource. The input resource's `value` is used to create a child custom resource inside the component, before a reference to this child is returned.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod component_custom_ref_input_output {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComponentCustomRefInputOutputArgs {
        #[builder(into)]
        pub input_ref: pulumi_gestalt_rust::InputOrOutput<super::types::Custom>,
    }
    #[allow(dead_code)]
    pub struct ComponentCustomRefInputOutputResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub input_ref: pulumi_gestalt_rust::Output<super::types::Custom>,
        pub output_ref: pulumi_gestalt_rust::Output<super::types::Custom>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentCustomRefInputOutputArgs,
    ) -> ComponentCustomRefInputOutputResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentCustomRefInputOutputArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ComponentCustomRefInputOutputResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComponentCustomRefInputOutputArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ComponentCustomRefInputOutputResult {
        let input_ref_binding = args.input_ref.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "component:index:ComponentCustomRefInputOutput".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputRef".into(),
                    value: &input_ref_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        ComponentCustomRefInputOutputResult {
            id: o.get_id(),
            urn: o.get_urn(),
            input_ref: o.get_field("inputRef"),
            output_ref: o.get_field("outputRef"),
        }
    }
}
