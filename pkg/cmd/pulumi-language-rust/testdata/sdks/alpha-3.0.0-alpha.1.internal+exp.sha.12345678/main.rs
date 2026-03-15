include!("resources/resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::alpha")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_ALPHA: [u8; 78] = *b"{\"version\":\"3.0.0-alpha.1.internal+exp.sha.12345678\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "3.0.0-alpha.1.internal+exp.sha.12345678".to_string()
}
