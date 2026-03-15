#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod lambda {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LambdaArgs {
        #[builder(into)]
        pub builtins: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub lambda: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub property: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LambdaResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub builtins: pulumi_gestalt_rust::Output<String>,
        pub lambda: pulumi_gestalt_rust::Output<String>,
        pub property: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LambdaArgs,
    ) -> LambdaResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LambdaArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> LambdaResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LambdaArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> LambdaResult {
        let builtins_binding = args.builtins.get_output(context);
        let lambda_binding = args.lambda.get_output(context);
        let property_binding = args.property.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "keywords:index:Lambda".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "builtins".into(),
                    value: &builtins_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lambda".into(),
                    value: &lambda_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "property".into(),
                    value: &property_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        LambdaResult {
            id: o.get_id(),
            urn: o.get_urn(),
            builtins: o.get_field("builtins"),
            lambda: o.get_field("lambda"),
            property: o.get_field("property"),
        }
    }
}
