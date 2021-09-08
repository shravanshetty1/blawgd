fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("../../../frontends/rust/client/src/blawgd_client")
        .format(true)
        .build_client(true)
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["../../../backends/cosmos/api/grpc/blawgd.proto"],
            &["../../../backends/cosmos/api/grpc"],
        )
        .unwrap();
    Ok(())
}
