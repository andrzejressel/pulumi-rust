include!("resources/block.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::sync")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_SYNC: [u8; 77] = *b"{\"version\":\"3.0.0-alpha.1.internal+exp.sha.2143768\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "3.0.0-alpha.1.internal+exp.sha.2143768".to_string()
}
