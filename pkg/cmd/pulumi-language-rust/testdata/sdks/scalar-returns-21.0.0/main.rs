pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    include!("functions/invoke_array.rs");
    include!("functions/invoke_map.rs");
    include!("functions/invoke_secret.rs");
}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::scalar-returns")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_SCALAR_RETURNS: [u8; 45] = *b"{\"version\":\"21.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "21.0.0".to_string()
}
