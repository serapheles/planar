use crux_core::typegen::TypeGen;
use shared::Planar;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../shared");

    let mut gen = TypeGen::new();

    gen.register_app::<Planar>()?;

    let output_root = PathBuf::from("./generated");

    gen.java("com.example.planar.shared_types", output_root.join("java"))?;

    Ok(())
}
