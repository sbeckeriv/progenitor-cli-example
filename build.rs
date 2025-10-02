use progenitor::{GenerationSettings, Generator, InterfaceStyle, TagStyle, TypeImpl, TypePatch};
use syn::token;

fn main() {
    return;
    let src = "cli-gen.json.txt";
    let file = std::fs::File::open(src).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    /*
        let mut generator = Generator::new(
            GenerationSettings::default()
                .with_interface(InterfaceStyle::Builder)
                .with_cli_bounds("std::clone::Clone")
                .with_tag(TagStyle::Separate),
        );
    */
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Merged)
            .with_derive("schemars::JsonSchema")
            .with_patch("Name", TypePatch::default().with_derive("Hash"))
            .with_conversion(
                schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Integer.into()),
                    format: Some("int32".to_string()),
                    ..Default::default()
                },
                "usize",
                [TypeImpl::Display].into_iter(),
            ),
    );
    let tokens = generator.generate_tokens(&spec).unwrap();

    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    let mut out_file = std::path::Path::new("src").to_path_buf();
    out_file.push("example_builder.rs");
    std::fs::write(out_file, content).unwrap();

    // CLI generation.
    let tokens = generator
        .cli(&spec, &format!("crate::example_builder"))
        .unwrap();

    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    let mut out_file = std::path::Path::new("src").to_path_buf();
    out_file.push("cli.rs");

    std::fs::write(out_file, content).unwrap();

    // httpmock generation.
    let tokens = generator
        .httpmock(&spec, &format!("crate::example_builder"))
        .unwrap();

    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    let mut out_file = std::path::Path::new("src").to_path_buf();
    out_file.push("example_httpmock.rs");

    std::fs::write(out_file, content).unwrap();
}
