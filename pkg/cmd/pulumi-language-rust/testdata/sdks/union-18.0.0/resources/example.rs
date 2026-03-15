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
        pub map_map_union_property: pulumi_gestalt_rust::InputOrOutput<
            Option<
                std::collections::HashMap<
                    String,
                    std::collections::HashMap<
                        String,
                        pulumi_gestalt_rust::OneOf2<String, Vec<String>>,
                    >,
                >,
            >,
        >,
        #[builder(into, default)]
        pub string_enum_union_list_property: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<pulumi_gestalt_rust::OneOf2<String, super::types::AccessRights>>>,
        >,
        #[builder(into, default)]
        pub string_or_integer_property: pulumi_gestalt_rust::InputOrOutput<
            Option<pulumi_gestalt_rust::OneOf2<String, i32>>,
        >,
        #[builder(into, default)]
        pub typed_enum_property: pulumi_gestalt_rust::InputOrOutput<
            Option<pulumi_gestalt_rust::OneOf2<String, super::types::BlobType>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExampleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub map_map_union_property: pulumi_gestalt_rust::Output<
            Option<
                std::collections::HashMap<
                    String,
                    std::collections::HashMap<
                        String,
                        pulumi_gestalt_rust::OneOf2<String, Vec<String>>,
                    >,
                >,
            >,
        >,
        pub string_enum_union_list_property: pulumi_gestalt_rust::Output<
            Option<Vec<pulumi_gestalt_rust::OneOf2<String, super::types::AccessRights>>>,
        >,
        pub string_or_integer_property: pulumi_gestalt_rust::Output<
            Option<pulumi_gestalt_rust::OneOf2<String, i32>>,
        >,
        pub typed_enum_property: pulumi_gestalt_rust::Output<
            Option<pulumi_gestalt_rust::OneOf2<String, super::types::BlobType>>,
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
        let map_map_union_property_binding = args
            .map_map_union_property
            .get_output(context);
        let string_enum_union_list_property_binding = args
            .string_enum_union_list_property
            .get_output(context);
        let string_or_integer_property_binding = args
            .string_or_integer_property
            .get_output(context);
        let typed_enum_property_binding = args.typed_enum_property.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "union:index:Example".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mapMapUnionProperty".into(),
                    value: &map_map_union_property_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stringEnumUnionListProperty".into(),
                    value: &string_enum_union_list_property_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stringOrIntegerProperty".into(),
                    value: &string_or_integer_property_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typedEnumProperty".into(),
                    value: &typed_enum_property_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        ExampleResult {
            id: o.get_id(),
            urn: o.get_urn(),
            map_map_union_property: o.get_field("mapMapUnionProperty"),
            string_enum_union_list_property: o.get_field("stringEnumUnionListProperty"),
            string_or_integer_property: o.get_field("stringOrIntegerProperty"),
            typed_enum_property: o.get_field("typedEnumProperty"),
        }
    }
}
