include!("resources/resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::config")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_CONFIG: [u8; 60] = *b"{\"version\":\"9.0.0\",\"pluginDownloadURL\":\"http://example.com\"}";
pub(crate) fn get_version() -> String {
    "9.0.0".to_string()
}
