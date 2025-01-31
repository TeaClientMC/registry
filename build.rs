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
use std::{env, path::PathBuf}
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
