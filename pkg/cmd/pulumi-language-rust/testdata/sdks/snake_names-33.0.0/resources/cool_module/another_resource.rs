#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod another_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct another_resourceArgs {
        #[builder(into)]
        pub the_input: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct another_resourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub the_input: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: another_resourceArgs,
    ) -> another_resourceResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: another_resourceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> another_resourceResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: another_resourceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> another_resourceResult {
        let the_input_binding = args.the_input.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "snake_names:cool_module:another_resource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "the_input".into(),
                    value: &the_input_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        another_resourceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            the_input: o.get_field("the_input"),
        }
    }
}
