use crate::wasm::DataTypes;
use crate::wasm::Executable;
use crate::wasm::State;

pub struct I32Add {}

impl Executable for I32Add {
    fn exec(&self, state: &mut State) {
        let aval = match state.stack.pop() {
            Some(a) => a,
            None => panic!(),
        };
        let bval = match state.stack.pop() {
            Some(b) => b,
            None => panic!(),
        };
        let result: DataTypes = match aval {
            DataTypes::I32(a) => match bval {
                DataTypes::I32(b) => DataTypes::I32(a + b),
                _ => panic!(),
            },
            _ => panic!(),
        };
        state.stack.push(result);
        state.ip = state.ip + 1;
    }
}

pub struct I64Add {}

impl Executable for I64Add {
    fn exec(&self, state: &mut State) {
        let aval = match state.stack.pop() {
            Some(a) => a,
            None => panic!(),
        };
        let bval = match state.stack.pop() {
            Some(b) => b,
            None => panic!(),
        };
        let result: DataTypes = match aval {
            DataTypes::I64(a) => match bval {
                DataTypes::I64(b) => DataTypes::I64(a + b),
                _ => panic!(),
            },
            _ => panic!(),
        };
        state.stack.push(result);
        state.ip = state.ip + 1;
    }
}

pub struct I32Sub {}

impl Executable for I32Sub {
    fn exec(&self, state: &mut State) {
        let aval = match state.stack.pop() {
            Some(a) => a,
            None => panic!(),
        };
        let bval = match state.stack.pop() {
            Some(b) => b,
            None => panic!(),
        };
        let result: DataTypes = match aval {
            DataTypes::I32(a) => match bval {
                DataTypes::I32(b) => DataTypes::I32(a - b),
                _ => panic!(),
            },
            _ => panic!(),
        };
        state.stack.push(result);
        state.ip = state.ip + 1;
    }
}

pub struct I64Sub {}

impl Executable for I64Sub {
    fn exec(&self, state: &mut State) {
        let aval = match state.stack.pop() {
            Some(a) => a,
            None => panic!(),
        };
        let bval = match state.stack.pop() {
            Some(b) => b,
            None => panic!(),
        };
        let result: DataTypes = match aval {
            DataTypes::I64(a) => match bval {
                DataTypes::I64(b) => DataTypes::I64(a - b),
                _ => panic!(),
            },
            _ => panic!(),
        };
        state.stack.push(result);
        state.ip = state.ip + 1;
    }
}

pub struct I32Mul {}

impl Executable for I32Mul {
    fn exec(&self, state: &mut State) {
        let aval = match state.stack.pop() {
            Some(a) => a,
            None => panic!(),
        };
        let bval = match state.stack.pop() {
            Some(b) => b,
            None => panic!(),
        };
        let result: DataTypes = match aval {
            DataTypes::I32(a) => match bval {
                DataTypes::I32(b) => DataTypes::I32(a * b),
                _ => panic!(),
            },
            _ => panic!(),
        };
        state.stack.push(result);
        state.ip = state.ip + 1;
    }
}

pub struct I64Mul {}

impl Executable for I64Mul {
    fn exec(&self, state: &mut State) {
        let aval = match state.stack.pop() {
            Some(a) => a,
            None => panic!(),
        };
        let bval = match state.stack.pop() {
            Some(b) => b,
            None => panic!(),
        };
        let result: DataTypes = match aval {
            DataTypes::I64(a) => match bval {
                DataTypes::I64(b) => DataTypes::I64(a * b),
                _ => panic!(),
            },
            _ => panic!(),
        };
        state.stack.push(result);
        state.ip = state.ip + 1;
    }
}

pub struct I32DivS {}

impl Executable for I32DivS {
    fn exec(&self, state: &mut State) {
        let aval = match state.stack.pop() {
            Some(a) => a,
            None => panic!(),
        };
        let bval = match state.stack.pop() {
            Some(b) => b,
            None => panic!(),
        };
        let result: DataTypes = match aval {
            DataTypes::I32(a) => match bval {
                DataTypes::I32(b) => DataTypes::I32(a / b),
                _ => panic!(),
            },
            _ => panic!(),
        };
        state.stack.push(result);
        state.ip = state.ip + 1;
    }
}

pub struct I64DivS {}

impl Executable for I64DivS {
    fn exec(&self, state: &mut State) {
        let aval = match state.stack.pop() {
            Some(a) => a,
            None => panic!(),
        };
        let bval = match state.stack.pop() {
            Some(b) => b,
            None => panic!(),
        };
        let result: DataTypes = match aval {
            DataTypes::I64(a) => match bval {
                DataTypes::I64(b) => DataTypes::I64(a / b),
                _ => panic!(),
            },
            _ => panic!(),
        };
        state.stack.push(result);
        state.ip = state.ip + 1;
    }
}

