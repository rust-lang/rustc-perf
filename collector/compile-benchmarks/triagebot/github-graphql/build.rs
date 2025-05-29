pub fn main() {
    cynic_codegen::register_schema("github")
        .from_sdl_file("src/github.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
