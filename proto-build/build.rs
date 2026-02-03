fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &[
                "../proto/sf/firehose/v2/firehose.proto",
                "../proto/sf/ethereum/type/v2/type.proto",
            ],
            &["../proto"], // root folder for proto files
        )?;
    Ok(())
}
