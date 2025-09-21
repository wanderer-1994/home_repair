use proto_builder::ProtoBuilder;

fn main() -> Result<(), std::io::Error> {
    let mut builder = tonic_prost_build::configure();

    builder = builder
        .db_enum("AccountRole", "text")
        .graphql_enum("AccountRole")
        .serde("AccountRole");

    builder.compile_protos(
        &["data_type/entity_type/proto/account.proto"],
        &[env!("ROOT_DIR")],
    )
}
