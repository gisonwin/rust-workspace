pub fn build() {
    prost_build::Config::new().out_dir("src/pb")
        // .include_file("protobuf/SZDFH-public")
        .compile_protos(&["protobuf/hello.proto"], &["."])
        .unwrap()
}