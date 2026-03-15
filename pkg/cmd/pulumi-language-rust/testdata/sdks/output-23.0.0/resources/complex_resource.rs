#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod complex_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComplexResourceArgs {
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<f64>,
    }
    #[allow(dead_code)]
    pub struct ComplexResourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub output_array: pulumi_gestalt_rust::Output<Vec<String>>,
        pub output_map: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub output_object: pulumi_gestalt_rust::Output<super::types::Data>,
        pub value: pulumi_gestalt_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComplexResourceArgs,
    ) -> ComplexResourceResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComplexResourceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ComplexResourceResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ComplexResourceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ComplexResourceResult {
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "output:index:ComplexResource".into(),
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
        ComplexResourceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            output_array: o.get_field("outputArray"),
            output_map: o.get_field("outputMap"),
            output_object: o.get_field("outputObject"),
            value: o.get_field("value"),
        }
    }
}
