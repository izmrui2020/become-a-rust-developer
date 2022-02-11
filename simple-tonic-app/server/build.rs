//tonic build
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/demo.proto")?;
    Ok(())
}