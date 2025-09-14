//! Build script for `error` crate

use std::io::Result;

/// Build proto spec
fn main() -> Result<()> {
    let mut builder = prost_build::Config::new();
    builder.compile_protos(&["proto/error_details.proto"], &["proto/"])?;
    Ok(())
}
