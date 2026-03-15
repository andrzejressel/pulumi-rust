#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod sink {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SinkArgs {
        #[builder(into, default)]
        pub string_enum: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::StringEnum>,
        >,
    }
    #[allow(dead_code)]
    pub struct SinkResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub string_enum: pulumi_gestalt_rust::Output<Option<super::types::StringEnum>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SinkArgs,
    ) -> SinkResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SinkArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SinkResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SinkArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SinkResult {
        let string_enum_binding = args.string_enum.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "extenumref:index:Sink".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stringEnum".into(),
                    value: &string_enum_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        SinkResult {
            id: o.get_id(),
            urn: o.get_urn(),
            string_enum: o.get_field("stringEnum"),
        }
    }
}
