#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod config_fetcher {
    #[allow(dead_code)]
    pub struct ConfigFetcherResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub config: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
    ) -> ConfigFetcherResult {
        __create(context, name, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ConfigFetcherResult {
        __create(context, name, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ConfigFetcherResult {
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "config-grpc:index:ConfigFetcher".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[],
            options,
        };
        let o = context.register_resource(request);
        ConfigFetcherResult {
            id: o.get_id(),
            urn: o.get_urn(),
            config: o.get_field("config"),
        }
    }
}
