fn main() {
    tonic_build::configure()
        .compile(&["GesProtobuf/ges_interface.proto"], &["./GesProtobuf/"])
        .unwrap();
}
