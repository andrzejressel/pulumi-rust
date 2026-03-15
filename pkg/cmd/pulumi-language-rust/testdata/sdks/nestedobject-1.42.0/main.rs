include!("resources/container.rs");
include!("resources/map_container.rs");
include!("resources/receiver.rs");
include!("resources/target.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {
    include!("types/detail.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::nestedobject")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_NESTEDOBJECT: [u8; 45] = *b"{\"version\":\"1.42.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.42.0".to_string()
}
