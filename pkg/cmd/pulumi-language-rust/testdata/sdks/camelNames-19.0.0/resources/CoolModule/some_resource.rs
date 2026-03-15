#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod some_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SomeResourceArgs {
        #[builder(into, default)]
        pub resource_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub the_input: pulumi_gestalt_rust::InputOrOutput<bool>,
    }
    #[allow(dead_code)]
    pub struct SomeResourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub resource_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub the_output: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SomeResourceArgs,
    ) -> SomeResourceResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SomeResourceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SomeResourceResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SomeResourceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SomeResourceResult {
        let resource_name_binding = args.resource_name.get_output(context);
        let the_input_binding = args.the_input.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "camelNames:CoolModule:SomeResource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceName".into(),
                    value: &resource_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "theInput".into(),
                    value: &the_input_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        SomeResourceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            resource_name: o.get_field("resourceName"),
            the_output: o.get_field("theOutput"),
        }
    }
}
