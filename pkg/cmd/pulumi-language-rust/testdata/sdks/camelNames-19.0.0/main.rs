pub mod CoolModule {
    include!("resources/CoolModule/some_resource.rs");
}
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::camelNames")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_CAMELNAMES: [u8; 45] = *b"{\"version\":\"19.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "19.0.0".to_string()
}
