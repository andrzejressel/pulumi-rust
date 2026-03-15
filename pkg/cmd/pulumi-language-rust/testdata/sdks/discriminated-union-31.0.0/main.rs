include!("resources/example.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {
    include!("types/variant_one.rs");
    include!("types/variant_two.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::discriminated-union")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_DISCRIMINATED_UNION: [u8; 45] = *b"{\"version\":\"31.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "31.0.0".to_string()
}
