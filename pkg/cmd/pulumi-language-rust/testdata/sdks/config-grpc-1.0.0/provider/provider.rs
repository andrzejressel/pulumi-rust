#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
#[derive(pulumi_gestalt_rust::__private::bon::Builder)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderArgs {
    #[builder(into, default)]
    pub bool1: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    #[builder(into, default)]
    pub bool2: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    #[builder(into, default)]
    pub bool3: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    #[builder(into, default)]
    pub int1: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    #[builder(into, default)]
    pub int2: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    #[builder(into, default)]
    pub int3: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    #[builder(into, default)]
    pub list_bool1: pulumi_gestalt_rust::InputOrOutput<Option<Vec<bool>>>,
    #[builder(into, default)]
    pub list_bool2: pulumi_gestalt_rust::InputOrOutput<Option<Vec<bool>>>,
    #[builder(into, default)]
    pub list_bool3: pulumi_gestalt_rust::InputOrOutput<Option<Vec<bool>>>,
    #[builder(into, default)]
    pub list_int1: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
    #[builder(into, default)]
    pub list_int2: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
    #[builder(into, default)]
    pub list_int3: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
    #[builder(into, default)]
    pub list_num1: pulumi_gestalt_rust::InputOrOutput<Option<Vec<f64>>>,
    #[builder(into, default)]
    pub list_num2: pulumi_gestalt_rust::InputOrOutput<Option<Vec<f64>>>,
    #[builder(into, default)]
    pub list_num3: pulumi_gestalt_rust::InputOrOutput<Option<Vec<f64>>>,
    #[builder(into, default)]
    pub list_secret_bool1: pulumi_gestalt_rust::InputOrOutput<Option<Vec<bool>>>,
    #[builder(into, default)]
    pub list_secret_bool2: pulumi_gestalt_rust::InputOrOutput<Option<Vec<bool>>>,
    #[builder(into, default)]
    pub list_secret_bool3: pulumi_gestalt_rust::InputOrOutput<Option<Vec<bool>>>,
    #[builder(into, default)]
    pub list_secret_int1: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
    #[builder(into, default)]
    pub list_secret_int2: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
    #[builder(into, default)]
    pub list_secret_int3: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
    #[builder(into, default)]
    pub list_secret_num1: pulumi_gestalt_rust::InputOrOutput<Option<Vec<f64>>>,
    #[builder(into, default)]
    pub list_secret_num2: pulumi_gestalt_rust::InputOrOutput<Option<Vec<f64>>>,
    #[builder(into, default)]
    pub list_secret_num3: pulumi_gestalt_rust::InputOrOutput<Option<Vec<f64>>>,
    #[builder(into, default)]
    pub list_secret_string1: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    #[builder(into, default)]
    pub list_secret_string2: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    #[builder(into, default)]
    pub list_secret_string3: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    #[builder(into, default)]
    pub list_string1: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    #[builder(into, default)]
    pub list_string2: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    #[builder(into, default)]
    pub list_string3: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    #[builder(into, default)]
    pub map_bool1: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, bool>>,
    >,
    #[builder(into, default)]
    pub map_bool2: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, bool>>,
    >,
    #[builder(into, default)]
    pub map_bool3: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, bool>>,
    >,
    #[builder(into, default)]
    pub map_int1: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, i32>>,
    >,
    #[builder(into, default)]
    pub map_int2: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, i32>>,
    >,
    #[builder(into, default)]
    pub map_int3: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, i32>>,
    >,
    #[builder(into, default)]
    pub map_num1: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, f64>>,
    >,
    #[builder(into, default)]
    pub map_num2: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, f64>>,
    >,
    #[builder(into, default)]
    pub map_num3: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, f64>>,
    >,
    #[builder(into, default)]
    pub map_secret_bool1: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, bool>>,
    >,
    #[builder(into, default)]
    pub map_secret_bool2: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, bool>>,
    >,
    #[builder(into, default)]
    pub map_secret_bool3: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, bool>>,
    >,
    #[builder(into, default)]
    pub map_secret_int1: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, i32>>,
    >,
    #[builder(into, default)]
    pub map_secret_int2: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, i32>>,
    >,
    #[builder(into, default)]
    pub map_secret_int3: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, i32>>,
    >,
    #[builder(into, default)]
    pub map_secret_num1: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, f64>>,
    >,
    #[builder(into, default)]
    pub map_secret_num2: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, f64>>,
    >,
    #[builder(into, default)]
    pub map_secret_num3: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, f64>>,
    >,
    #[builder(into, default)]
    pub map_secret_string1: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, String>>,
    >,
    #[builder(into, default)]
    pub map_secret_string2: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, String>>,
    >,
    #[builder(into, default)]
    pub map_secret_string3: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, String>>,
    >,
    #[builder(into, default)]
    pub map_string1: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, String>>,
    >,
    #[builder(into, default)]
    pub map_string2: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, String>>,
    >,
    #[builder(into, default)]
    pub map_string3: pulumi_gestalt_rust::InputOrOutput<
        Option<std::collections::HashMap<String, String>>,
    >,
    #[builder(into, default)]
    pub num1: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
    #[builder(into, default)]
    pub num2: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
    #[builder(into, default)]
    pub num3: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
    #[builder(into, default)]
    pub obj_bool1: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tbool1>>,
    #[builder(into, default)]
    pub obj_bool2: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tbool2>>,
    #[builder(into, default)]
    pub obj_bool3: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tbool3>>,
    #[builder(into, default)]
    pub obj_int1: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tint1>>,
    #[builder(into, default)]
    pub obj_int2: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tint2>>,
    #[builder(into, default)]
    pub obj_int3: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tint3>>,
    #[builder(into, default)]
    pub obj_num1: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tnum1>>,
    #[builder(into, default)]
    pub obj_num2: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tnum2>>,
    #[builder(into, default)]
    pub obj_num3: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tnum3>>,
    #[builder(into, default)]
    pub obj_secret_bool1: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretBool1>,
    >,
    #[builder(into, default)]
    pub obj_secret_bool2: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretBool2>,
    >,
    #[builder(into, default)]
    pub obj_secret_bool3: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretBool3>,
    >,
    #[builder(into, default)]
    pub obj_secret_int1: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretInt1>,
    >,
    #[builder(into, default)]
    pub obj_secret_int2: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretInt2>,
    >,
    #[builder(into, default)]
    pub obj_secret_int3: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretInt3>,
    >,
    #[builder(into, default)]
    pub obj_secret_num1: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretNum1>,
    >,
    #[builder(into, default)]
    pub obj_secret_num2: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretNum2>,
    >,
    #[builder(into, default)]
    pub obj_secret_num3: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretNum3>,
    >,
    #[builder(into, default)]
    pub obj_secret_string1: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretString1>,
    >,
    #[builder(into, default)]
    pub obj_secret_string2: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretString2>,
    >,
    #[builder(into, default)]
    pub obj_secret_string3: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::TsecretString3>,
    >,
    #[builder(into, default)]
    pub obj_string1: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tstring1>>,
    #[builder(into, default)]
    pub obj_string2: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tstring2>>,
    #[builder(into, default)]
    pub obj_string3: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Tstring3>>,
    #[builder(into, default)]
    pub secret_bool1: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    #[builder(into, default)]
    pub secret_bool2: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    #[builder(into, default)]
    pub secret_bool3: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    #[builder(into, default)]
    pub secret_int1: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    #[builder(into, default)]
    pub secret_int2: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    #[builder(into, default)]
    pub secret_int3: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    #[builder(into, default)]
    pub secret_num1: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
    #[builder(into, default)]
    pub secret_num2: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
    #[builder(into, default)]
    pub secret_num3: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
    #[builder(into, default)]
    pub secret_string1: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    #[builder(into, default)]
    pub secret_string2: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    #[builder(into, default)]
    pub secret_string3: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    #[builder(into, default)]
    pub string1: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    #[builder(into, default)]
    pub string2: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    #[builder(into, default)]
    pub string3: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
}
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
    /// Pulumi ID is the unique identifier assigned by the provider to this resource.
    pub id: pulumi_gestalt_rust::Output<String>,
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    pub provider_id: pulumi_gestalt_rust::Output<String>,
    pub secret_string1: pulumi_gestalt_rust::Output<Option<String>>,
    pub secret_string2: pulumi_gestalt_rust::Output<Option<String>>,
    pub secret_string3: pulumi_gestalt_rust::Output<Option<String>>,
    pub string1: pulumi_gestalt_rust::Output<Option<String>>,
    pub string2: pulumi_gestalt_rust::Output<Option<String>>,
    pub string3: pulumi_gestalt_rust::Output<Option<String>>,
}
impl pulumi_gestalt_rust::Provider for ProviderResult {
    fn get_provider_id(&self) -> pulumi_gestalt_rust::Output<String> {
        self.provider_id.clone()
    }
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
) -> ProviderResult {
    create_with_options(context, name, args, None)
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create_with_options(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
    options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
) -> ProviderResult {
    let bool1_binding = args.bool1.get_output(context);
    let bool2_binding = args.bool2.get_output(context);
    let bool3_binding = args.bool3.get_output(context);
    let int1_binding = args.int1.get_output(context);
    let int2_binding = args.int2.get_output(context);
    let int3_binding = args.int3.get_output(context);
    let list_bool1_binding = args.list_bool1.get_output(context);
    let list_bool2_binding = args.list_bool2.get_output(context);
    let list_bool3_binding = args.list_bool3.get_output(context);
    let list_int1_binding = args.list_int1.get_output(context);
    let list_int2_binding = args.list_int2.get_output(context);
    let list_int3_binding = args.list_int3.get_output(context);
    let list_num1_binding = args.list_num1.get_output(context);
    let list_num2_binding = args.list_num2.get_output(context);
    let list_num3_binding = args.list_num3.get_output(context);
    let list_secret_bool1_binding = args.list_secret_bool1.get_output(context);
    let list_secret_bool2_binding = args.list_secret_bool2.get_output(context);
    let list_secret_bool3_binding = args.list_secret_bool3.get_output(context);
    let list_secret_int1_binding = args.list_secret_int1.get_output(context);
    let list_secret_int2_binding = args.list_secret_int2.get_output(context);
    let list_secret_int3_binding = args.list_secret_int3.get_output(context);
    let list_secret_num1_binding = args.list_secret_num1.get_output(context);
    let list_secret_num2_binding = args.list_secret_num2.get_output(context);
    let list_secret_num3_binding = args.list_secret_num3.get_output(context);
    let list_secret_string1_binding = args.list_secret_string1.get_output(context);
    let list_secret_string2_binding = args.list_secret_string2.get_output(context);
    let list_secret_string3_binding = args.list_secret_string3.get_output(context);
    let list_string1_binding = args.list_string1.get_output(context);
    let list_string2_binding = args.list_string2.get_output(context);
    let list_string3_binding = args.list_string3.get_output(context);
    let map_bool1_binding = args.map_bool1.get_output(context);
    let map_bool2_binding = args.map_bool2.get_output(context);
    let map_bool3_binding = args.map_bool3.get_output(context);
    let map_int1_binding = args.map_int1.get_output(context);
    let map_int2_binding = args.map_int2.get_output(context);
    let map_int3_binding = args.map_int3.get_output(context);
    let map_num1_binding = args.map_num1.get_output(context);
    let map_num2_binding = args.map_num2.get_output(context);
    let map_num3_binding = args.map_num3.get_output(context);
    let map_secret_bool1_binding = args.map_secret_bool1.get_output(context);
    let map_secret_bool2_binding = args.map_secret_bool2.get_output(context);
    let map_secret_bool3_binding = args.map_secret_bool3.get_output(context);
    let map_secret_int1_binding = args.map_secret_int1.get_output(context);
    let map_secret_int2_binding = args.map_secret_int2.get_output(context);
    let map_secret_int3_binding = args.map_secret_int3.get_output(context);
    let map_secret_num1_binding = args.map_secret_num1.get_output(context);
    let map_secret_num2_binding = args.map_secret_num2.get_output(context);
    let map_secret_num3_binding = args.map_secret_num3.get_output(context);
    let map_secret_string1_binding = args.map_secret_string1.get_output(context);
    let map_secret_string2_binding = args.map_secret_string2.get_output(context);
    let map_secret_string3_binding = args.map_secret_string3.get_output(context);
    let map_string1_binding = args.map_string1.get_output(context);
    let map_string2_binding = args.map_string2.get_output(context);
    let map_string3_binding = args.map_string3.get_output(context);
    let num1_binding = args.num1.get_output(context);
    let num2_binding = args.num2.get_output(context);
    let num3_binding = args.num3.get_output(context);
    let obj_bool1_binding = args.obj_bool1.get_output(context);
    let obj_bool2_binding = args.obj_bool2.get_output(context);
    let obj_bool3_binding = args.obj_bool3.get_output(context);
    let obj_int1_binding = args.obj_int1.get_output(context);
    let obj_int2_binding = args.obj_int2.get_output(context);
    let obj_int3_binding = args.obj_int3.get_output(context);
    let obj_num1_binding = args.obj_num1.get_output(context);
    let obj_num2_binding = args.obj_num2.get_output(context);
    let obj_num3_binding = args.obj_num3.get_output(context);
    let obj_secret_bool1_binding = args.obj_secret_bool1.get_output(context);
    let obj_secret_bool2_binding = args.obj_secret_bool2.get_output(context);
    let obj_secret_bool3_binding = args.obj_secret_bool3.get_output(context);
    let obj_secret_int1_binding = args.obj_secret_int1.get_output(context);
    let obj_secret_int2_binding = args.obj_secret_int2.get_output(context);
    let obj_secret_int3_binding = args.obj_secret_int3.get_output(context);
    let obj_secret_num1_binding = args.obj_secret_num1.get_output(context);
    let obj_secret_num2_binding = args.obj_secret_num2.get_output(context);
    let obj_secret_num3_binding = args.obj_secret_num3.get_output(context);
    let obj_secret_string1_binding = args.obj_secret_string1.get_output(context);
    let obj_secret_string2_binding = args.obj_secret_string2.get_output(context);
    let obj_secret_string3_binding = args.obj_secret_string3.get_output(context);
    let obj_string1_binding = args.obj_string1.get_output(context);
    let obj_string2_binding = args.obj_string2.get_output(context);
    let obj_string3_binding = args.obj_string3.get_output(context);
    let secret_bool1_binding = args.secret_bool1.get_output(context);
    let secret_bool2_binding = args.secret_bool2.get_output(context);
    let secret_bool3_binding = args.secret_bool3.get_output(context);
    let secret_int1_binding = args.secret_int1.get_output(context);
    let secret_int2_binding = args.secret_int2.get_output(context);
    let secret_int3_binding = args.secret_int3.get_output(context);
    let secret_num1_binding = args.secret_num1.get_output(context);
    let secret_num2_binding = args.secret_num2.get_output(context);
    let secret_num3_binding = args.secret_num3.get_output(context);
    let secret_string1_binding = args.secret_string1.get_output(context);
    let secret_string2_binding = args.secret_string2.get_output(context);
    let secret_string3_binding = args.secret_string3.get_output(context);
    let string1_binding = args.string1.get_output(context);
    let string2_binding = args.string2.get_output(context);
    let string3_binding = args.string3.get_output(context);
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:config-grpc".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bool1".into(),
                value: &bool1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bool2".into(),
                value: &bool2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bool3".into(),
                value: &bool3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "int1".into(),
                value: &int1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "int2".into(),
                value: &int2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "int3".into(),
                value: &int3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listBool1".into(),
                value: &list_bool1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listBool2".into(),
                value: &list_bool2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listBool3".into(),
                value: &list_bool3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listInt1".into(),
                value: &list_int1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listInt2".into(),
                value: &list_int2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listInt3".into(),
                value: &list_int3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listNum1".into(),
                value: &list_num1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listNum2".into(),
                value: &list_num2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listNum3".into(),
                value: &list_num3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretBool1".into(),
                value: &list_secret_bool1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretBool2".into(),
                value: &list_secret_bool2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretBool3".into(),
                value: &list_secret_bool3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretInt1".into(),
                value: &list_secret_int1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretInt2".into(),
                value: &list_secret_int2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretInt3".into(),
                value: &list_secret_int3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretNum1".into(),
                value: &list_secret_num1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretNum2".into(),
                value: &list_secret_num2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretNum3".into(),
                value: &list_secret_num3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretString1".into(),
                value: &list_secret_string1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretString2".into(),
                value: &list_secret_string2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listSecretString3".into(),
                value: &list_secret_string3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listString1".into(),
                value: &list_string1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listString2".into(),
                value: &list_string2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "listString3".into(),
                value: &list_string3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapBool1".into(),
                value: &map_bool1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapBool2".into(),
                value: &map_bool2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapBool3".into(),
                value: &map_bool3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapInt1".into(),
                value: &map_int1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapInt2".into(),
                value: &map_int2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapInt3".into(),
                value: &map_int3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapNum1".into(),
                value: &map_num1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapNum2".into(),
                value: &map_num2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapNum3".into(),
                value: &map_num3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretBool1".into(),
                value: &map_secret_bool1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretBool2".into(),
                value: &map_secret_bool2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretBool3".into(),
                value: &map_secret_bool3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretInt1".into(),
                value: &map_secret_int1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretInt2".into(),
                value: &map_secret_int2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretInt3".into(),
                value: &map_secret_int3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretNum1".into(),
                value: &map_secret_num1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretNum2".into(),
                value: &map_secret_num2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretNum3".into(),
                value: &map_secret_num3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretString1".into(),
                value: &map_secret_string1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretString2".into(),
                value: &map_secret_string2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapSecretString3".into(),
                value: &map_secret_string3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapString1".into(),
                value: &map_string1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapString2".into(),
                value: &map_string2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mapString3".into(),
                value: &map_string3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "num1".into(),
                value: &num1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "num2".into(),
                value: &num2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "num3".into(),
                value: &num3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objBool1".into(),
                value: &obj_bool1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objBool2".into(),
                value: &obj_bool2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objBool3".into(),
                value: &obj_bool3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objInt1".into(),
                value: &obj_int1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objInt2".into(),
                value: &obj_int2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objInt3".into(),
                value: &obj_int3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objNum1".into(),
                value: &obj_num1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objNum2".into(),
                value: &obj_num2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objNum3".into(),
                value: &obj_num3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretBool1".into(),
                value: &obj_secret_bool1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretBool2".into(),
                value: &obj_secret_bool2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretBool3".into(),
                value: &obj_secret_bool3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretInt1".into(),
                value: &obj_secret_int1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretInt2".into(),
                value: &obj_secret_int2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretInt3".into(),
                value: &obj_secret_int3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretNum1".into(),
                value: &obj_secret_num1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretNum2".into(),
                value: &obj_secret_num2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretNum3".into(),
                value: &obj_secret_num3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretString1".into(),
                value: &obj_secret_string1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretString2".into(),
                value: &obj_secret_string2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objSecretString3".into(),
                value: &obj_secret_string3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objString1".into(),
                value: &obj_string1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objString2".into(),
                value: &obj_string2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "objString3".into(),
                value: &obj_string3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretBool1".into(),
                value: &secret_bool1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretBool2".into(),
                value: &secret_bool2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretBool3".into(),
                value: &secret_bool3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretInt1".into(),
                value: &secret_int1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretInt2".into(),
                value: &secret_int2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretInt3".into(),
                value: &secret_int3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretNum1".into(),
                value: &secret_num1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretNum2".into(),
                value: &secret_num2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretNum3".into(),
                value: &secret_num3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretString1".into(),
                value: &secret_string1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretString2".into(),
                value: &secret_string2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretString3".into(),
                value: &secret_string3_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "string1".into(),
                value: &string1_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "string2".into(),
                value: &string2_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "string3".into(),
                value: &string3_binding.drop_type(),
            },
        ],
        options,
    };
    let o = context.register_resource(request);
    ProviderResult {
        urn: o.get_urn(),
        id: o.get_id(),
        provider_id: o.get_provider_id(),
        secret_string1: o.get_field("secretString1"),
        secret_string2: o.get_field("secretString2"),
        secret_string3: o.get_field("secretString3"),
        string1: o.get_field("string1"),
        string2: o.get_field("string2"),
        string3: o.get_field("string3"),
    }
}
