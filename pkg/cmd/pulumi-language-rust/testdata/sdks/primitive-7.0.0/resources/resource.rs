#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        #[builder(into)]
        pub boolean: pulumi_gestalt_rust::InputOrOutput<bool>,
        #[builder(into)]
        pub boolean_map: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, bool>,
        >,
        #[builder(into)]
        pub float: pulumi_gestalt_rust::InputOrOutput<f64>,
        #[builder(into)]
        pub integer: pulumi_gestalt_rust::InputOrOutput<i32>,
        #[builder(into)]
        pub number_array: pulumi_gestalt_rust::InputOrOutput<Vec<f64>>,
        #[builder(into)]
        pub string: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub boolean: pulumi_gestalt_rust::Output<bool>,
        pub boolean_map: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        pub float: pulumi_gestalt_rust::Output<f64>,
        pub integer: pulumi_gestalt_rust::Output<i32>,
        pub number_array: pulumi_gestalt_rust::Output<Vec<f64>>,
        pub string: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        __create(context, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ResourceResult {
        __create(context, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ResourceResult {
        let boolean_binding = args.boolean.get_output(context);
        let boolean_map_binding = args.boolean_map.get_output(context);
        let float_binding = args.float.get_output(context);
        let integer_binding = args.integer.get_output(context);
        let number_array_binding = args.number_array.get_output(context);
        let string_binding = args.string.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "primitive:index:Resource".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "boolean".into(),
                    value: &boolean_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "booleanMap".into(),
                    value: &boolean_map_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "float".into(),
                    value: &float_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integer".into(),
                    value: &integer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numberArray".into(),
                    value: &number_array_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "string".into(),
                    value: &string_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        ResourceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            boolean: o.get_field("boolean"),
            boolean_map: o.get_field("booleanMap"),
            float: o.get_field("float"),
            integer: o.get_field("integer"),
            number_array: o.get_field("numberArray"),
            string: o.get_field("string"),
        }
    }
}
