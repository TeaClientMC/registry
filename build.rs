/**
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See thread GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 **/
use reqwest::blocking::{Client, ClientBuilder};
use schemars::schema::RootSchema;
use std::{env, path::PathBuf};
use typify::{TypeSpace, TypeSpaceSettings};

fn add_schema(client: &Client, url: &str) -> Result<RootSchema, reqwest::Error> {
    client.get(url).send()?.json::<RootSchema>()
}

fn main() {
    println!("cargo:rerun-if-env-changed=PATH");

    let api_url = std::env::var("API_URL").unwrap_or("https://api.teaclient.net".to_string());
    let client = ClientBuilder::new()
        .build()
        .expect("Failed to build HTTP client");

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));

    let schema_paths = ["/items-schema", "/servers/schema", "/partners/schema"];

    for path in &schema_paths {
        let url = format!("{}/registry{}", api_url, path);
        let schema =
            add_schema(&client, &url).expect(&format!("Failed to fetch schema from {}", url));
        type_space
            .add_root_schema(schema)
            .expect("Failed to add schema to TypeSpace");
    }

    let contents = format!(
        r#"/** 
     *  This program is free software: you can redistribute it and/or modify
     *  it under the terms of the GNU General Public License as published by
     *  the Free Software Foundation, either version 3 of the License, or
     *  (at your option) any later version.
     *
     *  This program is distributed in the hope that it will be useful,
     *  but WITHOUT ANY WARRANTY; without even the implied warranty of
     *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
     *  GNU General Public License for more details.
     *
     *  You should have received a copy of the GNU General Public License
     *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
     */
    {}
    "#,
        prettyplease::unparse(
            &syn::parse2::<syn::File>(type_space.to_stream())
                .expect("Failed to parse generated code"),
        )
    );

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set");

    std::fs::write(PathBuf::from(out_dir).join("schemas.rs"), contents)
        .expect("Failed to write schemas.rs");
}
