use crate::disassembler::Disassembler;

struct State {
    memory: [u8; 1 << 16],
    registers: [u8; 8],
    instruction_pointer: u16,

    /// Stores the end of the instruction segment
    instruction_segment: Option<u16>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            memory: [0; 1 << 16],
            registers: [0; 8],
            instruction_pointer: 0,
            instruction_segment: None,
        }
    }
}

impl Iterator for State {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.instruction_pointer > self.instruction_segment? {
            return None;
        }

        let byte = self.memory.get(self.instruction_pointer as usize);
        self.instruction_pointer += 1;
        byte.copied()
    }
}

pub struct Processor {
    state: State,
}

impl Processor {
    pub fn load(&mut self, program: impl Iterator<Item = u8>) {
        let mut segment_end = 0;
        self.state
            .memory
            .iter_mut()
            .zip(program)
            .enumerate()
            .for_each(|(idx, (mem, prog))| {
                *mem = prog;
                segment_end = idx
            });
        self.state.instruction_segment = Some(segment_end as u16);
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
