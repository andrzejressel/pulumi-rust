#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod dyn_list_to_dyn {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DynListToDynArgs {
        #[builder(into, default)]
        pub inputs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct DynListToDynResult {
        pub result: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: DynListToDynArgs,
    ) -> DynListToDynResult {
        let inputs_binding = args.inputs.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "any-type-function:index:dynListToDyn".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputs".into(),
                    value: &inputs_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        DynListToDynResult {
            result: o.get_field("result"),
        }
    }
}
