pub fn generate_main() -> String {
    let file = include_str!("main.rs.template").replace("{{CONTENT}}", "");

    let syntax_tree = syn::parse_file(file.as_str()).unwrap();
    

    prettyplease::unparse(&syntax_tree)
}
