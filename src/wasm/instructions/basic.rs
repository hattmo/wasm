use crate::wasm::DataTypes;
use crate::wasm::Executable;
use crate::wasm::State;

pub struct NoOp {}

impl Executable for NoOp {
    fn exec(&self, state: &mut State) {
        state.ip = state.ip + 1;
    }
}

pub struct Drop {}

impl Executable for Drop {
    fn exec(&self, state: &mut State) {
        state.stack.pop();
        state.ip = state.ip + 1;
    }
}

pub struct I32Const {
    pub val: i32,
}

impl Executable for I32Const {
    fn exec(&self, state: &mut State) {
        state.stack.push(DataTypes::I32(self.val));
        state.ip = state.ip + 1;
    }
}
pub struct I64Const {
    pub val: i64,
}

impl Executable for I64Const {
    fn exec(&self, state: &mut State) {
        state.stack.push(DataTypes::I64(self.val));
        state.ip = state.ip + 1;
    }
}

pub struct F32Const {
    pub val: f32,
}

impl Executable for F32Const {
    fn exec(&self, state: &mut State) {
        state.stack.push(DataTypes::F32(self.val));
        state.ip = state.ip + 1;
    }
}
pub struct F64Const {
    pub val: f64,
}

impl Executable for F64Const {
    fn exec(&self, state: &mut State) {
        state.stack.push(DataTypes::F64(self.val));
        state.ip = state.ip + 1;
    }
}

pub struct LocalGet {
    pub id: usize,
}

impl Executable for LocalGet {
    fn exec(&self, state: &mut State) {
        state.stack.push(state.local[self.id]);
        state.ip = state.ip + 1;
    }
}

pub struct LocalSet {
    pub id: usize,
}

impl Executable for LocalSet {
    fn exec(&self, state: &mut State) {
        match state.stack.pop() {
            Some(v) => state.local[self.id] = v,
            None => panic!(),
        };
        state.ip = state.ip + 1;
    }
}
