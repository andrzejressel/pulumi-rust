include!("resources/enum_output.rs");
include!("resources/example.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {
    include!("types/access_rights.rs");
    include!("types/blob_type.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::union")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_UNION: [u8; 45] = *b"{\"version\":\"18.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "18.0.0".to_string()
}
