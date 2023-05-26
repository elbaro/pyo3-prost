# pyo3-prost

pyo3-prost exposes Rust protobuf structs to Python.

[Docs](https://docs.rs/pyo3-prost/0.1.0/pyo3_prost/)
[]


```
# A simple decoding benchmark
Python:  14.008996680990094
Rust  :  0.5708326659951126

# A simple encoding benchmark
Python:  8.68384731200058
Rust  :  0.3029898940003477

===========
PythonTweet
===========
b'\n\x11Hi this is a text\x10\xfb\x99K"\x1b\n\x03Who\x12\x14https://example.com/*\x06@trump*\x06@obama'
===========
  RustTweet
===========
b'\n\x11Hi this is a text\x10\xfb\x99K"\x1b\n\x03Who\x12\x14https://example.com/*\x06@trump*\x06@obama'
```

All you need is adding a single line
```
.type_attribute(".", "#[::pyo3_prost::pyclass_for_prost_struct]")
```
to your prost or tonic build.


The intended use-case is when your Python project is locked down to Protobuf and you want to migrate some part to native modules in Rust.
Python protobuf objects cannot be easily handled in the Rust side.
For conversion, the Python protobuf object should be serialized and deserialized to a Rust protobuf object.
With `pyo3-prost` you can decode the bytes to Rust structs in Python from the beginning and pass it to the native module.

## How it works

The derive macro `pyclass_for_prost_struct` will add
1. `#[pyclass]` to prost-generated structs
2. `#[pyo3(get, set)]` for each field (except oneof fields).
3. `#[pymethods]` with `encode() -> &PyBytes`, `decode(&PyBytes) -> Self`, `decode_merge(&mut self, &PyBytes)`.
4. `#[pyproto]` with `__str__` and `__repr__` for easy inspection.

- Getter/setter works with nested structs.
- `encode_slow` is name this way because it's currently implemented by serializing to Rust buffer and copy the bytes to Python memory.

## Example

1. Go to `examples/proto-gen` and `cargo run`. This will create `examples/rupy_proto/src/app.rs`.
2. Go to `examples/rupy_proto` and `cargo build --release`.
3. Move `examples/rupy_proto/target/release/librupy_proto.so` to `examples/rupy_proto/rupy_proto.so`.
4. Run the bench `cd examples/rupy_proto && PYTHONPATH=. python bench.py`.

## Limitations

Accessing non-primitive fields can copy.
Proxy types are being implemented to avoid copy.

- [ ] List<primitive>
- [ ] List<object>
- [ ] Map<primitive,primitive>
- [ ] Map<primitive, object>
- [ ] Map<object, primitive>
- [ ] Map<object, object>
- [x] object, optional<object>
- [ ] oneof (enum)

## Map<primitive, primitive>

## List<primitive>
