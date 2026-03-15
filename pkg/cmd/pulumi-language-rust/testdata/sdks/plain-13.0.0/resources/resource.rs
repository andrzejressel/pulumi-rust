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
        pub data: pulumi_gestalt_rust::InputOrOutput<super::types::Data>,
        #[builder(into, default)]
        pub data_list: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::InnerData>>,
        >,
        /// A non plain input to compare against the plain inputs, as well as testing plain/non-plain nesting.
        #[builder(into, default)]
        pub non_plain_data: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::Data>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub data: pulumi_gestalt_rust::Output<super::types::Data>,
        pub data_list: pulumi_gestalt_rust::Output<Option<Vec<super::types::InnerData>>>,
        /// A non plain input to compare against the plain inputs, as well as testing plain/non-plain nesting.
        pub non_plain_data: pulumi_gestalt_rust::Output<Option<super::types::Data>>,
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
        let data_binding = args.data.get_output(context);
        let data_list_binding = args.data_list.get_output(context);
        let non_plain_data_binding = args.non_plain_data.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "plain:index:Resource".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "data".into(),
                    value: &data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataList".into(),
                    value: &data_list_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nonPlainData".into(),
                    value: &non_plain_data_binding.drop_type(),
                },
            ],
            options,
        };
        let o = context.register_resource(request);
        ResourceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            data: o.get_field("data"),
            data_list: o.get_field("dataList"),
            non_plain_data: o.get_field("nonPlainData"),
        }
    }
}
