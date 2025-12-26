use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=PATH");
    if env::var("CARGO_FEATURE_LUNARSERVERMAPPINGS").is_ok() {
        let meta_ts = typify::import_types!(schema = "./LunarServerMappings/metadata.schema.json");
        let inactive_ts =
            typify::import_types!(schema = "./LunarServerMappings/inactive.schema.json");

        let meta_src = prettyplease::unparse(&syn::parse2(meta_ts).unwrap());
        let inactive_src = prettyplease::unparse(&syn::parse2(inactive_ts).unwrap());

        let contents = format!(
            "{meta}\n\n{inactive}",
            meta = meta_src,
            inactive = inactive_src
        );

        let out_dir = env::var_os("OUT_DIR").unwrap();
        std::fs::write(
            PathBuf::from(out_dir).join("lunarclient_schemas.rs"),
            contents,
        )
        .unwrap();
    }
}
