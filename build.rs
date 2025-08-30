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
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=PATH");
    let file = std::fs::File::open("./openapi.json").unwrap();
    let spec = serde_json::from_reader(file).expect("Failed to deserialize openapi.json");
    let mut generator = progenitor::Generator::default();
    let tokens = generator.generate_tokens(&spec).unwrap();

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
            &syn::parse2::<syn::File>(tokens).expect("Failed to parse generated code"),
        )
    );

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set");

    std::fs::write(PathBuf::from(out_dir).join("schemas.rs"), contents)
        .expect("Failed to write schemas.rs");
}
