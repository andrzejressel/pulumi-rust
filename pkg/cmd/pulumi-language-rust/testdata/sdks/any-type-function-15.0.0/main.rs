pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    include!("functions/dyn_list_to_dyn.rs");
}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::any-type-function")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_ANY_TYPE_FUNCTION: [u8; 45] = *b"{\"version\":\"15.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "15.0.0".to_string()
}
