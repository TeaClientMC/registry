use std::{env, path::PathBuf};

use reqwest::blocking::ClientBuilder;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let api_url = "https://api.teaclient.net/registry";

    let client = ClientBuilder::new()
        .build()
        .expect("Failed to create client");
    let registry_json = client
        .get(api_url.to_owned() + &"/items-schema".to_owned().to_string())
        .send()
        .expect("Failed")
        .json::<schemars::schema::RootSchema>()
        .expect("Failed");

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(registry_json).unwrap();

    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
    );

    std::fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("schemas.rs"),
        contents,
    )
    .unwrap();
}
