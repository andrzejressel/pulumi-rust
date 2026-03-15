#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod container {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerArgs {
        #[builder(into)]
        pub inputs: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct ContainerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub details: pulumi_gestalt_rust::Output<Vec<super::types::Detail>>,
        pub inputs: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContainerArgs,
    ) -> ContainerResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContainerArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ContainerResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContainerArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ContainerResult {
        let inputs_binding = args.inputs.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "nestedobject:index:Container".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputs".into(),
                    value: &inputs_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        ContainerResult {
            id: o.get_id(),
            urn: o.get_urn(),
            details: o.get_field("details"),
            inputs: o.get_field("inputs"),
        }
    }
}
