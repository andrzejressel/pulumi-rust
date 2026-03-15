pub mod cool_module {
    include!("resources/cool_module/another_resource.rs");
    include!("resources/cool_module/some_resource.rs");
}
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    pub mod cool_module {
        include!("functions/cool_module/some_data.rs");
    }
}
pub mod types {
    pub mod cool_module {
        include!("types/cool_module/entry.rs");
        include!("types/cool_module/nested_input.rs");
        include!("types/cool_module/output_item.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::snake_names")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_SNAKE_NAMES: [u8; 45] = *b"{\"version\":\"33.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "33.0.0".to_string()
}
