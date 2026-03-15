include!("resources/string_resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    include!("functions/my_invoke.rs");
    include!("functions/secret_invoke.rs");
    include!("functions/unit.rs");
}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::output-only-invoke")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_OUTPUT_ONLY_INVOKE: [u8; 45] = *b"{\"version\":\"24.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "24.0.0".to_string()
}
