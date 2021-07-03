use crate::wasm::DataTypes;
use crate::wasm::Executable;
use crate::wasm::State;

pub struct Jump {
    dst: usize,
}

impl Executable for Jump {
    fn exec(&self, state: &mut State) {
        state.ip = self.dst;
    }
}

pub struct CondJump {
    pub dst: usize,
}

impl Executable for CondJump {
    fn exec(&self, state: &mut State) {
        if match state.stack.pop() {
            Some(item) => match item {
                DataTypes::I32(i) => i == 0,
                DataTypes::I64(i) => i == 0,
                _ => panic!(),
            },
            None => {
                panic!();
            }
        } {
            state.ip = state.ip + 1;
        } else {
            state.ip = self.dst;
        }
    }
}
