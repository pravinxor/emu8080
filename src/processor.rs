use std::ops::Range;

use crate::disassembler::{Disassembler, Instruction, Register, RegisterPair, XData};

struct State {
    memory: [u8; 1 << 16],
    registers: [u8; 8],
    instruction_pointer: u16,
}

impl Default for State {
    fn default() -> Self {
        Self {
            memory: [0; 1 << 16],
            registers: [0; 8],
            instruction_pointer: 0,
        }
    }
}

impl Iterator for State {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.memory.get(self.instruction_pointer as usize);
        self.instruction_pointer = self.instruction_pointer.checked_add(1)?;
        byte.copied()
    }
}

#[derive(Default)]
pub struct Processor {
    state: State,
}

impl Processor {
    pub fn load(&mut self, program: impl Iterator<Item = u8>) {
        self.state
            .memory
            .iter_mut()
            .zip(program)
            .for_each(|(mem, prog)| *mem = prog);
    }

    fn set_rp(&mut self, rp: RegisterPair, data: XData) {
        let r: Range<usize> = rp.into();
        self.state.registers[r].copy_from_slice(&data.to_le_bytes());
    }

    fn get_rp(&self, rp: RegisterPair) -> XData {
        let mut r: Range<usize> = rp.into();
        let a = r.next().unwrap();
        let b = r.next().unwrap();
        XData::from_le_bytes([self.state.registers[a], self.state.registers[b]])
    }
}

impl Iterator for Processor {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = Disassembler::new(&mut self.state).next()?;
        match instruction {
            Instruction::Nop => {}
            Instruction::Lxi(rp, data) => self.set_rp(rp, data),
            Instruction::Stax(rp) => {
                self.set_rp(rp, self.state.registers[Register::A as usize] as u16)
            }
            Instruction::Inx(rp) => self.set_rp(rp, self.get_rp(rp) + 1),
            Instruction::Inr(r) => self.state.registers[r as usize] += 1,
            Instruction::Dcr(r) => self.state.registers[r as usize] -= 1,
            Instruction::Mvi(r, data) => self.state.registers[r as usize] = data,
            Instruction::Dad(rp) => self.set_rp(
                RegisterPair::Hl,
                self.get_rp(RegisterPair::Hl) + self.get_rp(rp),
            ),
            Instruction::Ldax(rp) => {
                self.state.registers[Register::A as usize] =
                    self.state.memory[self.get_rp(rp) as usize]
            }
            Instruction::Dcx(rp) => self.set_rp(rp, self.get_rp(rp) - 1),
            Instruction::Rlc => todo!(),
            Instruction::Rrc => todo!(),
            Instruction::Ral => todo!(),
            Instruction::Rar => todo!(),
            Instruction::Shld(_) => todo!(),
            Instruction::Daa => todo!(),
            Instruction::Lhld(_) => todo!(),
            Instruction::Cma => todo!(),
            Instruction::Sta(_) => todo!(),
            Instruction::Stc => todo!(),
            Instruction::Lda(_) => todo!(),
            Instruction::Cmc => todo!(),
            Instruction::Mov(_, _) => todo!(),
            Instruction::Hlt => return None,
            Instruction::AluR(_, _) => todo!(),
            Instruction::Rcc(_) => todo!(),
            Instruction::Pop(_) => todo!(),
            Instruction::Jcc(_, _) => todo!(),
            Instruction::Jmp(_) => todo!(),
            Instruction::Ccc(_, _) => todo!(),
            Instruction::Push(_) => todo!(),
            Instruction::AluI(_, _) => todo!(),
            Instruction::Rst(_) => todo!(),
            Instruction::Ret => todo!(),
            Instruction::Call(_) => todo!(),
            Instruction::Out(_) => todo!(),
            Instruction::In(_) => todo!(),
            Instruction::Xthl => todo!(),
            Instruction::Pchl => todo!(),
            Instruction::Di => todo!(),
            Instruction::Sphl => todo!(),
            Instruction::Ei => todo!(),
        }
        Some(())
    }
}
