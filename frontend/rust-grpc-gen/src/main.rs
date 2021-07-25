fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("../wasm/src/blawgd_client")
        .format(true)
        .build_client(true)
        .build_server(false)
        .compile(
            &["../../proto/samachar/samachar.proto"],
            &["../../proto/samachar"],
        )
        .unwrap();
    Ok(())
}
