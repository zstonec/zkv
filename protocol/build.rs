fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."])
        .type_attribute(".", "#[derive(PartialOrd)]")
        .out_dir("src/pb")
        .compile_protos(&["proto/commands.proto"], &["."])
        .unwrap();
}