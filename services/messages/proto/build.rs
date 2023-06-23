fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    let build = tonic_build::configure();
    cfg_if::cfg_if! {
        if #[cfg(feature = "client")] {
            build
                .build_transport(false)
                .build_server(false);
        } else if #[cfg(not(feature = "server"))] {
            panic!("Specify 'server' or 'client' feature in dependencies")
        }
    }
    build.compile(&["./src/v1/messages.proto"], &["./src/v1"])?;

    Ok(())
}
