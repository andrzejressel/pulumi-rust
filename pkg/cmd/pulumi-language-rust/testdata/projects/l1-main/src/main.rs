use anyhow::Result;
use pulumi_gestalt_rust::add_export;
fn main() {
    let t = pulumi_gestalt_rust::run(pulumi_main);
    if let Err(e) = t {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    add_export("test", &context.new_output(&"Hello, Pulumi!".to_string()));
    Ok(())
}
