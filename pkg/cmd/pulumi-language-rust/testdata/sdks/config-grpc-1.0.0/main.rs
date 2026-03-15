include!("resources/config_fetcher.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    include!("functions/to_secret.rs");
}
pub mod types {
    include!("types/tbool_1.rs");
    include!("types/tbool_2.rs");
    include!("types/tbool_3.rs");
    include!("types/tint_1.rs");
    include!("types/tint_2.rs");
    include!("types/tint_3.rs");
    include!("types/tnum_1.rs");
    include!("types/tnum_2.rs");
    include!("types/tnum_3.rs");
    include!("types/tsecret_bool_1.rs");
    include!("types/tsecret_bool_2.rs");
    include!("types/tsecret_bool_3.rs");
    include!("types/tsecret_int_1.rs");
    include!("types/tsecret_int_2.rs");
    include!("types/tsecret_int_3.rs");
    include!("types/tsecret_num_1.rs");
    include!("types/tsecret_num_2.rs");
    include!("types/tsecret_num_3.rs");
    include!("types/tsecret_string_1.rs");
    include!("types/tsecret_string_2.rs");
    include!("types/tsecret_string_3.rs");
    include!("types/tstring_1.rs");
    include!("types/tstring_2.rs");
    include!("types/tstring_3.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::config-grpc")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_CONFIG_GRPC: [u8; 44] = *b"{\"version\":\"1.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.0.0".to_string()
}
