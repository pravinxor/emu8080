use crate::disassembler::Disassembler;

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
}

impl Default for Processor {
    fn default() -> Self {
        Self {
            state: State::default(),
        }
    }
}

impl Iterator for Processor {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = Disassembler::new(&mut self.state).next()?;
        dbg!(instruction);
        Some(())
    }
}
