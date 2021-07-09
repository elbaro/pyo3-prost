use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("../proto/src/")
        .type_attribute(".", "#[::pyo3_prost::pyclass_for_prost_struct]")
        .compile_protos(&["../proto/src/tweet.proto"], &["../proto/src/"])
}
