fn main() {
    tonic_build::compile_protos("proto/docker/docker.proto").unwrap();
}
