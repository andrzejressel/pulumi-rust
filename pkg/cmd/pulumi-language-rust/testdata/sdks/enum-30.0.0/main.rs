pub mod mod_ {
    pub mod nested {
        include!("resources/mod_/nested/res.rs");
    }
    include!("resources/mod_/res.rs");
}
include!("resources/res.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {
    pub mod mod_ {
        pub mod nested {
            include!("types/mod_/nested/int_enum.rs");
            include!("types/mod_/nested/string_enum.rs");
        }
        include!("types/mod_/int_enum.rs");
        include!("types/mod_/string_enum.rs");
    }
    include!("types/int_enum.rs");
    include!("types/string_enum.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::enum")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_ENUM: [u8; 45] = *b"{\"version\":\"30.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "30.0.0".to_string()
}
