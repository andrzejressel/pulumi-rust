include!("resources/archive_resource.rs");
include!("resources/asset_resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::asset-archive")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_ASSET_ARCHIVE: [u8; 44] = *b"{\"version\":\"5.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "5.0.0".to_string()
}
