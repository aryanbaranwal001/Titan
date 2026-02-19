fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(false)
        .type_attribute(
            ".sf.ethereum.type.v2.BigInt",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".sf.ethereum.type.v2.Uint64NestedArray",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".sf.ethereum.type.v2.Uint64Array",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile_protos(
            &[
                "../proto/sf/firehose/v2/firehose.proto",
                "../proto/sf/ethereum/type/v2/type.proto",
            ],
            &["../proto"],
        )?;
    Ok(())
}
