//! Binary for generating `schema.graphql` for FE
//! TODO(huy): add CI check to verify that `schema.graphql` is up-to-date

use core_service_server::{CreateSchemaOption, create_schema};
use std::{fs::File, io::Write, path::Path};

fn main() {
    let schema = create_schema(CreateSchemaOption::NoContext);
    let mut file = File::create(Path::new("schema.graphql")).unwrap();
    file.write_all(schema.sdl().as_bytes()).unwrap();
}
