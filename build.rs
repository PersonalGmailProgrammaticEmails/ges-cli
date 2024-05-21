fn main() {
    tonic_build::configure()
        .compile(&["GesProtobuf/ges.proto"], &["./GesProtobuf/"])
        .unwrap();
}
