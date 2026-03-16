use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &["pulumi-ast/package.proto", "pulumi-ast/pcl.proto"],
        &["pulumi-ast"],
    )?;
    Ok(())
}
