include!("resources/string.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::large")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_LARGE: [u8; 44] = *b"{\"version\":\"4.3.2\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "4.3.2".to_string()
}
