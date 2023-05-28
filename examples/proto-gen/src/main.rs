use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("../rupy_proto/src/")
        .type_attribute(".", "#[::fastproto_macro::pyclass_for_prost_struct]")
        .compile_protos(&["../rupy_proto/src/tweet.proto"], &["../rupy_proto/src"])
}
