include!("resources/component.rs");
include!("resources/custom.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    include!("functions/component_refs.rs");
}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::component-property-deps")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_COMPONENT_PROPERTY_DEPS: [u8; 45] = *b"{\"version\":\"1.33.7\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.33.7".to_string()
}
