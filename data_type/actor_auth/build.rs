fn main() -> Result<(), std::io::Error> {
    let builder = tonic_prost_build::configure();

    builder.compile_protos(
        &["data_type/actor_auth/actor_auth.proto"],
        &[env!("ROOT_DIR")],
    )
}
