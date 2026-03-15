pub mod mod_ {
    pub mod nested_Resource {
        include!("resources/mod_/nested_Resource/resource.rs");
    }
}
pub mod mod_Resource {
    include!("resources/mod_Resource/resource.rs");
}
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    pub mod mod_ {
        pub mod nested_Resource {
            include!("functions/mod_/nested_Resource/resource_call.rs");
        }
        pub mod nested_concatWorld {
            include!("functions/mod_/nested_concatWorld/concat_world.rs");
        }
    }
    pub mod mod_Resource {
        include!("functions/mod_Resource/resource_call.rs");
    }
    pub mod mod_concatWorld {
        include!("functions/mod_concatWorld/concat_world.rs");
    }
}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::module-format")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_MODULE_FORMAT: [u8; 45] = *b"{\"version\":\"29.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "29.0.0".to_string()
}
