include!("resources/resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::simple")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_SIMPLE: [u8; 103] = *b"{\"version\":\"27.0.0\",\"pluginDownloadURL\":\"https://github.com/pulumi/pulumi-simple/releases/v${VERSION}\"}";
pub(crate) fn get_version() -> String {
    "27.0.0".to_string()
}
