use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["ast/package.proto", "ast/pcl.proto"], &["ast"])?;
    Ok(())
}
