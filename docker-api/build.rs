fn main() {
    //tower_grpc_build::compile_protos("proto/docker/docker.proto").unwrap();


    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(
            &["proto/docker/docker.proto"],
            &["proto/docker"],
        )
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
    println!("cargo:rerun-if-changed=proto/docker/docker.proto");
}
