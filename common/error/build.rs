use std::io::Result;

fn main() -> Result<()> {
    let mut builder = tonic_prost_build::Config::new();
    builder.compile_protos(&["proto/error_details.proto"], &["proto/"])?;
    Ok(())
}
