pub mod lambda {
    include!("resources/lambda/some_resource.rs");
}
include!("resources/lambda.rs");
include!("resources/some_resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::keywords")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_KEYWORDS: [u8; 45] = *b"{\"version\":\"20.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "20.0.0".to_string()
}
