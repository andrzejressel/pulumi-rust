pub mod mod_ {
    pub mod nested {
        include!("resources/mod_/nested/res.rs");
    }
    include!("resources/mod_/res.rs");
}
include!("resources/res_array.rs");
include!("resources/res_list.rs");
include!("resources/res_map.rs");
include!("resources/res_resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::names")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_NAMES: [u8; 44] = *b"{\"version\":\"6.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.0.0".to_string()
}
