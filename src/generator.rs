use crate::pcl_model::node::Value;
use crate::pcl_model::{
    ConfigVariable, Expression, LocalVariable, Node, OutputVariable, PclProtobufProgram,
    expression, literal_value_expression, traverse_index, traverser,
};
use rootcause::prelude::ResultExt;
use rootcause::{Result, bail};

pub fn generate_main(model_program: &PclProtobufProgram) -> Result<String> {
    let nodes = model_program
        .nodes
        .iter()
        .map(convert_node)
        .collect::<Result<Vec<_>>>()
        .context("Failed to convert model nodes")?
        .join("\n");

    // model_program.nodes
    //     .f

    let file = include_str!("main.rs.template").replace("{{CONTENT}}", &nodes);

    let syntax_tree = syn::parse_file(file.as_str())
        .context_with(|| format!("Failed to parse file [{}]", file))?;

    Ok(prettyplease::unparse(&syntax_tree))
}

fn convert_node(node: &Node) -> Result<String> {
    match &node.value {
        Value::Resource(_) => {
            bail!("Resource not yet supported")
        }
        Value::LocalVariable(local_variable) => {
            Ok(convert_local_variable(local_variable)
                .context("Failed to convert local variable")?)
        }
        Value::ConfigVariable(config_variable) => {
            Ok(convert_config_variable(config_variable)
                .context("Failed to convert config variable")?)
        }
        Value::OutputVariable(output) => {
            Ok(convert_output_variable(output).context("Failed to convert output variable")?)
        }
    }
}

fn convert_config_variable(config_variable: &ConfigVariable) -> Result<String> {
    Ok(format!(
        "let {} = context.require_config(None, \"{}\").expect(\"Expected config [{}] to exist\");",
        config_variable.name, config_variable.name, config_variable.name
    ))
}

fn convert_local_variable(local_variable: &LocalVariable) -> Result<String> {
    let value = convert_expression(&local_variable.value).context("Failed to convert value")?;
    Ok(format!("let {} = {};", local_variable.name, value))
}

fn convert_output_variable(output_variable: &OutputVariable) -> Result<String> {
    let value = convert_expression(&output_variable.value).context("Failed to convert value")?;
    Ok(format!(
        "pulumi_gestalt_rust::add_export(\"{}\", &context.new_output(&{}));",
        output_variable.name, value
    ))
}

fn convert_expression(expression: &Expression) -> Result<String> {
    match &expression.value {
        expression::Value::LiteralValueExpression(literal_value) => match &literal_value.value {
            literal_value_expression::Value::UnknownValue(_) => {
                bail!("UnknownValue not yet supported")
            }
            literal_value_expression::Value::StringValue(s) => Ok(format!("\"{}\"", s)),
            literal_value_expression::Value::NumberValue(n) => Ok(n.to_string()),
            literal_value_expression::Value::BoolValue(b) => Ok(b.to_string()),
        },
        expression::Value::TemplateExpression(_) => {
            bail!("TemplateExpression not yet supported")
        }
        expression::Value::IndexExpression(_) => {
            bail!("IndexExpression not yet supported")
        }
        expression::Value::ObjectConsExpression(_) => {
            bail!("ObjectConsExpression not yet supported")
        }
        expression::Value::TupleConsExpression(_) => {
            bail!("TupleConsExpression not yet supported")
        }
        expression::Value::FunctionCallExpression(function_call) => {
            let args = function_call
                .args
                .iter()
                .map(|a| &a.value)
                .map(convert_expression)
                .collect::<Result<Vec<_>>>()
                .context("Failed to convert function call arguments")?
                .join(", ");
            Ok(
                convert_stdlib_function_call(&function_call.name, args, function_call.args.len())
                    .context("Failed to convert function call")?,
            )
        }
        expression::Value::RelativeTraversalExpression(_) => {
            bail!("RelativeTraversalExpression not yet supported")
        }
        expression::Value::ScopeTraversalExpression(scope_traversal) => {
            let mut converted = scope_traversal.root_name.clone();
            for traverser in &scope_traversal.traversal.each {
                match &traverser.value {
                    traverser::Value::TraverseAttr(attr) => {
                        converted = format!("{}.{}", converted, attr.name);
                    }
                    traverser::Value::TraverseIndex(index) => match &index.value {
                        traverse_index::Value::IntIndex(i) => {
                            converted = format!("{}[{}]", converted, i)
                        }
                        traverse_index::Value::StringIndex(s) => {
                            converted = format!("{}[\"{}\"]", converted, s)
                        }
                    },
                    traverser::Value::TraverseRoot(_) => {
                        // root_name already contains this information
                    }
                    traverser::Value::TraverseSplat(_) => {
                        bail!("TraverseSplat not yet supported")
                    }
                }
            }
            Ok(converted)
        }
        expression::Value::AnonymousFunctionExpression(_) => {
            bail!("AnonymousFunctionExpression not yet supported")
        }
        expression::Value::ConditionalExpression(_) => {
            bail!("ConditionalExpression not yet supported")
        }
        expression::Value::BinaryOpExpression(_) => {
            bail!("BinaryOpExpression not yet supported")
        }
        expression::Value::UnaryOpExpression(_) => {
            bail!("UnaryOpExpression not yet supported")
        }
    }
}

fn convert_stdlib_function_call(name: &str, args: String, arg_count: usize) -> Result<String> {
    match name {
        "fromBase64" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!("pulumi_gestalt_rust::stdlib::from_base64({}).expect(\"Fail to convert from base64\")", args))
        }
        "toBase64" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!("pulumi_gestalt_rust::stdlib::to_base64({})", args))
        }
        _ => bail!("Unsupported stdlib function: {}", name),
    }
}

fn ensure_arity(name: &str, got: usize, expected: usize) -> Result<()> {
    if got == expected {
        return Ok(());
    }
    bail!(
        "Invalid argument count for function {}: expected {}, got {}",
        name,
        expected,
        got
    )
}
