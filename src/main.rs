mod wasm;

fn main() {
    wasm::WasmFunc::new(4)
        .i64const(20)
        .local_get(0)
        .i64add()
        .exec(&[wasm::DataTypes::I64(30)]);
}
