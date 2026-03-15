include!("resources/resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::primitive")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_PRIMITIVE: [u8; 44] = *b"{\"version\":\"7.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "7.0.0".to_string()
}
