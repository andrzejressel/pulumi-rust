include!("resources/string_resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    include!("functions/my_invoke_scalar.rs");
}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::simple-invoke-with-scalar-return")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_SIMPLE_INVOKE_WITH_SCALAR_RETURN: [u8; 45] = *b"{\"version\":\"17.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "17.0.0".to_string()
}
