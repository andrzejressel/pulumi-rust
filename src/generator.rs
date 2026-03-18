use std::fmt::format;
use crate::pcl_model::node::Value;
use crate::pcl_model::{Node, OutputVariable, PclProtobufProgram, expression, literal_value_expression, ConfigVariable};
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
        Value::LocalVariable(_) => {
            bail!("LocalVariable not yet supported")
        }
        Value::ConfigVariable(config_variable) => {
            Ok(convert_config_variable(config_variable)
                .context("Failed to convert config variable")?)
            // bail!("ConfigVariable not yet supported")
        }
        Value::OutputVariable(output) => {
            Ok(convert_output_variable(output).context("Failed to convert output variable")
                .context("Failed to convert output variable")?)
        }
    }
}

fn convert_config_variable(config_variable: &ConfigVariable) -> Result<String> {
    Ok(format!(
        "let {} = context.get_config(\"{}\");",
        config_variable.name, config_variable.name
    ))
}

fn convert_output_variable(output_variable: &OutputVariable) -> Result<String> {
    let static_string = match &output_variable.value.value {
        expression::Value::LiteralValueExpression(literal_value) => match &literal_value.value {
            literal_value_expression::Value::UnknownValue(_) => {
                bail!("UnknownValue not yet supported")
            }
            literal_value_expression::Value::StringValue(s) => format!("\"{}\"", s),
            literal_value_expression::Value::NumberValue(n) => n.to_string(),
            literal_value_expression::Value::BoolValue(b) => b.to_string(),
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
        expression::Value::FunctionCallExpression(_) => {
            bail!("FunctionCallExpression not yet supported")
        }
        expression::Value::RelativeTraversalExpression(_) => {
            bail!("RelativeTraversalExpression not yet supported")
        }
        expression::Value::ScopeTraversalExpression(_) => {
            bail!("ScopeTraversalExpression not yet supported")
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
    };

    Ok(format!(
        "pulumi_gestalt_rust::add_export(\"{}\", &context.new_output(&{}));",
        output_variable.name, static_string
    ))
}
