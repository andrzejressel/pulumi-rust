include!("resources/resource_a.rs");
include!("resources/resource_b.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::replaceonchanges")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_REPLACEONCHANGES: [u8; 45] = *b"{\"version\":\"25.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "25.0.0".to_string()
}
