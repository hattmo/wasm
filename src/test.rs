#[test]
fn simpleTest() {
    println!("Running the Test!!!!");
    wasm::WasmFunc::new(4)
        .i64const(20)
        .local_get(0)
        .i64add()
        .exec(&[wasm::DataTypes::I64(30)]);
    assert_eq!(1, 1);
    assert_eq!(1,2);
}
