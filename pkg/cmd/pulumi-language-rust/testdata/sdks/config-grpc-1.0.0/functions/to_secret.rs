#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod to_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ToSecretArgs {
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
        pub obj_bool1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tbool1>,
        >,
        #[builder(into, default)]
        pub obj_bool2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tbool2>,
        >,
        #[builder(into, default)]
        pub obj_bool3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tbool3>,
        >,
        #[builder(into, default)]
        pub obj_int1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tint1>,
        >,
        #[builder(into, default)]
        pub obj_int2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tint2>,
        >,
        #[builder(into, default)]
        pub obj_int3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tint3>,
        >,
        #[builder(into, default)]
        pub obj_num1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tnum1>,
        >,
        #[builder(into, default)]
        pub obj_num2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tnum2>,
        >,
        #[builder(into, default)]
        pub obj_num3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tnum3>,
        >,
        #[builder(into, default)]
        pub obj_secret_bool1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretBool1>,
        >,
        #[builder(into, default)]
        pub obj_secret_bool2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretBool2>,
        >,
        #[builder(into, default)]
        pub obj_secret_bool3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretBool3>,
        >,
        #[builder(into, default)]
        pub obj_secret_int1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretInt1>,
        >,
        #[builder(into, default)]
        pub obj_secret_int2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretInt2>,
        >,
        #[builder(into, default)]
        pub obj_secret_int3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretInt3>,
        >,
        #[builder(into, default)]
        pub obj_secret_num1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretNum1>,
        >,
        #[builder(into, default)]
        pub obj_secret_num2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretNum2>,
        >,
        #[builder(into, default)]
        pub obj_secret_num3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretNum3>,
        >,
        #[builder(into, default)]
        pub obj_secret_string1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretString1>,
        >,
        #[builder(into, default)]
        pub obj_secret_string2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretString2>,
        >,
        #[builder(into, default)]
        pub obj_secret_string3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::TsecretString3>,
        >,
        #[builder(into, default)]
        pub obj_string1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tstring1>,
        >,
        #[builder(into, default)]
        pub obj_string2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tstring2>,
        >,
        #[builder(into, default)]
        pub obj_string3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::Tstring3>,
        >,
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
    pub struct ToSecretResult {
        pub bool1: pulumi_gestalt_rust::Output<bool>,
        pub bool2: pulumi_gestalt_rust::Output<bool>,
        pub bool3: pulumi_gestalt_rust::Output<bool>,
        pub int1: pulumi_gestalt_rust::Output<i32>,
        pub int2: pulumi_gestalt_rust::Output<i32>,
        pub int3: pulumi_gestalt_rust::Output<i32>,
        pub list_bool1: pulumi_gestalt_rust::Output<Vec<bool>>,
        pub list_bool2: pulumi_gestalt_rust::Output<Vec<bool>>,
        pub list_bool3: pulumi_gestalt_rust::Output<Vec<bool>>,
        pub list_int1: pulumi_gestalt_rust::Output<Vec<i32>>,
        pub list_int2: pulumi_gestalt_rust::Output<Vec<i32>>,
        pub list_int3: pulumi_gestalt_rust::Output<Vec<i32>>,
        pub list_num1: pulumi_gestalt_rust::Output<Vec<f64>>,
        pub list_num2: pulumi_gestalt_rust::Output<Vec<f64>>,
        pub list_num3: pulumi_gestalt_rust::Output<Vec<f64>>,
        pub list_secret_bool1: pulumi_gestalt_rust::Output<Vec<bool>>,
        pub list_secret_bool2: pulumi_gestalt_rust::Output<Vec<bool>>,
        pub list_secret_bool3: pulumi_gestalt_rust::Output<Vec<bool>>,
        pub list_secret_int1: pulumi_gestalt_rust::Output<Vec<i32>>,
        pub list_secret_int2: pulumi_gestalt_rust::Output<Vec<i32>>,
        pub list_secret_int3: pulumi_gestalt_rust::Output<Vec<i32>>,
        pub list_secret_num1: pulumi_gestalt_rust::Output<Vec<f64>>,
        pub list_secret_num2: pulumi_gestalt_rust::Output<Vec<f64>>,
        pub list_secret_num3: pulumi_gestalt_rust::Output<Vec<f64>>,
        pub list_secret_string1: pulumi_gestalt_rust::Output<Vec<String>>,
        pub list_secret_string2: pulumi_gestalt_rust::Output<Vec<String>>,
        pub list_secret_string3: pulumi_gestalt_rust::Output<Vec<String>>,
        pub list_string1: pulumi_gestalt_rust::Output<Vec<String>>,
        pub list_string2: pulumi_gestalt_rust::Output<Vec<String>>,
        pub list_string3: pulumi_gestalt_rust::Output<Vec<String>>,
        pub map_bool1: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        pub map_bool2: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        pub map_bool3: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        pub map_int1: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, i32>,
        >,
        pub map_int2: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, i32>,
        >,
        pub map_int3: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, i32>,
        >,
        pub map_num1: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, f64>,
        >,
        pub map_num2: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, f64>,
        >,
        pub map_num3: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, f64>,
        >,
        pub map_secret_bool1: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        pub map_secret_bool2: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        pub map_secret_bool3: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        pub map_secret_int1: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, i32>,
        >,
        pub map_secret_int2: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, i32>,
        >,
        pub map_secret_int3: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, i32>,
        >,
        pub map_secret_num1: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, f64>,
        >,
        pub map_secret_num2: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, f64>,
        >,
        pub map_secret_num3: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, f64>,
        >,
        pub map_secret_string1: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub map_secret_string2: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub map_secret_string3: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub map_string1: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub map_string2: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub map_string3: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub num1: pulumi_gestalt_rust::Output<f64>,
        pub num2: pulumi_gestalt_rust::Output<f64>,
        pub num3: pulumi_gestalt_rust::Output<f64>,
        pub obj_bool1: pulumi_gestalt_rust::Output<super::super::types::Tbool1>,
        pub obj_bool2: pulumi_gestalt_rust::Output<super::super::types::Tbool2>,
        pub obj_bool3: pulumi_gestalt_rust::Output<super::super::types::Tbool3>,
        pub obj_int1: pulumi_gestalt_rust::Output<super::super::types::Tint1>,
        pub obj_int2: pulumi_gestalt_rust::Output<super::super::types::Tint2>,
        pub obj_int3: pulumi_gestalt_rust::Output<super::super::types::Tint3>,
        pub obj_num1: pulumi_gestalt_rust::Output<super::super::types::Tnum1>,
        pub obj_num2: pulumi_gestalt_rust::Output<super::super::types::Tnum2>,
        pub obj_num3: pulumi_gestalt_rust::Output<super::super::types::Tnum3>,
        pub obj_secret_bool1: pulumi_gestalt_rust::Output<
            super::super::types::TsecretBool1,
        >,
        pub obj_secret_bool2: pulumi_gestalt_rust::Output<
            super::super::types::TsecretBool2,
        >,
        pub obj_secret_bool3: pulumi_gestalt_rust::Output<
            super::super::types::TsecretBool3,
        >,
        pub obj_secret_int1: pulumi_gestalt_rust::Output<
            super::super::types::TsecretInt1,
        >,
        pub obj_secret_int2: pulumi_gestalt_rust::Output<
            super::super::types::TsecretInt2,
        >,
        pub obj_secret_int3: pulumi_gestalt_rust::Output<
            super::super::types::TsecretInt3,
        >,
        pub obj_secret_num1: pulumi_gestalt_rust::Output<
            super::super::types::TsecretNum1,
        >,
        pub obj_secret_num2: pulumi_gestalt_rust::Output<
            super::super::types::TsecretNum2,
        >,
        pub obj_secret_num3: pulumi_gestalt_rust::Output<
            super::super::types::TsecretNum3,
        >,
        pub obj_secret_string1: pulumi_gestalt_rust::Output<
            super::super::types::TsecretString1,
        >,
        pub obj_secret_string2: pulumi_gestalt_rust::Output<
            super::super::types::TsecretString2,
        >,
        pub obj_secret_string3: pulumi_gestalt_rust::Output<
            super::super::types::TsecretString3,
        >,
        pub obj_string1: pulumi_gestalt_rust::Output<super::super::types::Tstring1>,
        pub obj_string2: pulumi_gestalt_rust::Output<super::super::types::Tstring2>,
        pub obj_string3: pulumi_gestalt_rust::Output<super::super::types::Tstring3>,
        pub secret_bool1: pulumi_gestalt_rust::Output<bool>,
        pub secret_bool2: pulumi_gestalt_rust::Output<bool>,
        pub secret_bool3: pulumi_gestalt_rust::Output<bool>,
        pub secret_int1: pulumi_gestalt_rust::Output<i32>,
        pub secret_int2: pulumi_gestalt_rust::Output<i32>,
        pub secret_int3: pulumi_gestalt_rust::Output<i32>,
        pub secret_num1: pulumi_gestalt_rust::Output<f64>,
        pub secret_num2: pulumi_gestalt_rust::Output<f64>,
        pub secret_num3: pulumi_gestalt_rust::Output<f64>,
        pub secret_string1: pulumi_gestalt_rust::Output<String>,
        pub secret_string2: pulumi_gestalt_rust::Output<String>,
        pub secret_string3: pulumi_gestalt_rust::Output<String>,
        pub string1: pulumi_gestalt_rust::Output<String>,
        pub string2: pulumi_gestalt_rust::Output<String>,
        pub string3: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: ToSecretArgs,
    ) -> ToSecretResult {
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
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "config-grpc:index:toSecret".into(),
            version: super::super::get_version(),
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
        };
        let o = context.invoke_resource(request);
        ToSecretResult {
            bool1: o.get_field("bool1"),
            bool2: o.get_field("bool2"),
            bool3: o.get_field("bool3"),
            int1: o.get_field("int1"),
            int2: o.get_field("int2"),
            int3: o.get_field("int3"),
            list_bool1: o.get_field("listBool1"),
            list_bool2: o.get_field("listBool2"),
            list_bool3: o.get_field("listBool3"),
            list_int1: o.get_field("listInt1"),
            list_int2: o.get_field("listInt2"),
            list_int3: o.get_field("listInt3"),
            list_num1: o.get_field("listNum1"),
            list_num2: o.get_field("listNum2"),
            list_num3: o.get_field("listNum3"),
            list_secret_bool1: o.get_field("listSecretBool1"),
            list_secret_bool2: o.get_field("listSecretBool2"),
            list_secret_bool3: o.get_field("listSecretBool3"),
            list_secret_int1: o.get_field("listSecretInt1"),
            list_secret_int2: o.get_field("listSecretInt2"),
            list_secret_int3: o.get_field("listSecretInt3"),
            list_secret_num1: o.get_field("listSecretNum1"),
            list_secret_num2: o.get_field("listSecretNum2"),
            list_secret_num3: o.get_field("listSecretNum3"),
            list_secret_string1: o.get_field("listSecretString1"),
            list_secret_string2: o.get_field("listSecretString2"),
            list_secret_string3: o.get_field("listSecretString3"),
            list_string1: o.get_field("listString1"),
            list_string2: o.get_field("listString2"),
            list_string3: o.get_field("listString3"),
            map_bool1: o.get_field("mapBool1"),
            map_bool2: o.get_field("mapBool2"),
            map_bool3: o.get_field("mapBool3"),
            map_int1: o.get_field("mapInt1"),
            map_int2: o.get_field("mapInt2"),
            map_int3: o.get_field("mapInt3"),
            map_num1: o.get_field("mapNum1"),
            map_num2: o.get_field("mapNum2"),
            map_num3: o.get_field("mapNum3"),
            map_secret_bool1: o.get_field("mapSecretBool1"),
            map_secret_bool2: o.get_field("mapSecretBool2"),
            map_secret_bool3: o.get_field("mapSecretBool3"),
            map_secret_int1: o.get_field("mapSecretInt1"),
            map_secret_int2: o.get_field("mapSecretInt2"),
            map_secret_int3: o.get_field("mapSecretInt3"),
            map_secret_num1: o.get_field("mapSecretNum1"),
            map_secret_num2: o.get_field("mapSecretNum2"),
            map_secret_num3: o.get_field("mapSecretNum3"),
            map_secret_string1: o.get_field("mapSecretString1"),
            map_secret_string2: o.get_field("mapSecretString2"),
            map_secret_string3: o.get_field("mapSecretString3"),
            map_string1: o.get_field("mapString1"),
            map_string2: o.get_field("mapString2"),
            map_string3: o.get_field("mapString3"),
            num1: o.get_field("num1"),
            num2: o.get_field("num2"),
            num3: o.get_field("num3"),
            obj_bool1: o.get_field("objBool1"),
            obj_bool2: o.get_field("objBool2"),
            obj_bool3: o.get_field("objBool3"),
            obj_int1: o.get_field("objInt1"),
            obj_int2: o.get_field("objInt2"),
            obj_int3: o.get_field("objInt3"),
            obj_num1: o.get_field("objNum1"),
            obj_num2: o.get_field("objNum2"),
            obj_num3: o.get_field("objNum3"),
            obj_secret_bool1: o.get_field("objSecretBool1"),
            obj_secret_bool2: o.get_field("objSecretBool2"),
            obj_secret_bool3: o.get_field("objSecretBool3"),
            obj_secret_int1: o.get_field("objSecretInt1"),
            obj_secret_int2: o.get_field("objSecretInt2"),
            obj_secret_int3: o.get_field("objSecretInt3"),
            obj_secret_num1: o.get_field("objSecretNum1"),
            obj_secret_num2: o.get_field("objSecretNum2"),
            obj_secret_num3: o.get_field("objSecretNum3"),
            obj_secret_string1: o.get_field("objSecretString1"),
            obj_secret_string2: o.get_field("objSecretString2"),
            obj_secret_string3: o.get_field("objSecretString3"),
            obj_string1: o.get_field("objString1"),
            obj_string2: o.get_field("objString2"),
            obj_string3: o.get_field("objString3"),
            secret_bool1: o.get_field("secretBool1"),
            secret_bool2: o.get_field("secretBool2"),
            secret_bool3: o.get_field("secretBool3"),
            secret_int1: o.get_field("secretInt1"),
            secret_int2: o.get_field("secretInt2"),
            secret_int3: o.get_field("secretInt3"),
            secret_num1: o.get_field("secretNum1"),
            secret_num2: o.get_field("secretNum2"),
            secret_num3: o.get_field("secretNum3"),
            secret_string1: o.get_field("secretString1"),
            secret_string2: o.get_field("secretString2"),
            secret_string3: o.get_field("secretString3"),
            string1: o.get_field("string1"),
            string2: o.get_field("string2"),
            string3: o.get_field("string3"),
        }
    }
}
