include!("resources/complex_resource.rs");
include!("resources/resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {
    include!("types/data.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::output")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_OUTPUT: [u8; 45] = *b"{\"version\":\"23.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "23.0.0".to_string()
}
