#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod example {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExampleArgs {
        #[builder(into, default)]
        pub array_of_union_of: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    pulumi_gestalt_rust::OneOf2<
                        super::types::VariantOne,
                        super::types::VariantTwo,
                    >,
                >,
            >,
        >,
        #[builder(into, default)]
        pub union_of: pulumi_gestalt_rust::InputOrOutput<
            Option<
                pulumi_gestalt_rust::OneOf2<
                    super::types::VariantOne,
                    super::types::VariantTwo,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ExampleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub array_of_union_of: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    pulumi_gestalt_rust::OneOf2<
                        super::types::VariantOne,
                        super::types::VariantTwo,
                    >,
                >,
            >,
        >,
        pub union_of: pulumi_gestalt_rust::Output<
            Option<
                pulumi_gestalt_rust::OneOf2<
                    super::types::VariantOne,
                    super::types::VariantTwo,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExampleArgs,
    ) -> ExampleResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExampleArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ExampleResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExampleArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ExampleResult {
        let array_of_union_of_binding = args.array_of_union_of.get_output(context);
        let union_of_binding = args.union_of.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "discriminated-union:index:Example".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arrayOfUnionOf".into(),
                    value: &array_of_union_of_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "unionOf".into(),
                    value: &union_of_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        ExampleResult {
            id: o.get_id(),
            urn: o.get_urn(),
            array_of_union_of: o.get_field("arrayOfUnionOf"),
            union_of: o.get_field("unionOf"),
        }
    }
}
