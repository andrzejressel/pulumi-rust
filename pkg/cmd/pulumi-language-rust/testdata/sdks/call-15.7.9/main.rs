include!("resources/custom.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    pub mod providers {
        include!("functions/providers/call_identity.rs");
        include!("functions/providers/call_prefixed.rs");
    }
    include!("functions/custom_provider_value.rs");
}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::call")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_CALL: [u8; 45] = *b"{\"version\":\"15.7.9\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "15.7.9".to_string()
}
