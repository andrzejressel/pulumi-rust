include!("resources/component_callable.rs");
include!("resources/component_custom_ref_input_output.rs");
include!("resources/component_custom_ref_output.rs");
include!("resources/custom.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    include!("functions/component_callable_identity.rs");
    include!("functions/component_callable_prefixed.rs");
}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::component")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_COMPONENT: [u8; 45] = *b"{\"version\":\"13.3.7\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "13.3.7".to_string()
}
