pub type Data = u8;

pub type DatLo = Data;
pub type DatHi = Data;

pub type AddLo = DatLo;
pub type AddHi = DatHi;

pub enum Register {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::B,
            1 => Self::C,
            2 => Self::D,
            3 => Self::E,
            4 => Self::H,
            5 => Self::L,
            6 => Self::HL,
            7 => Self::A,
            _ => panic!("{value} is not a valid for a Register"),
        }
    }
}

pub enum RegisterPair {
    BC,
    DE,
    FH,
    SP, // Stack pointer
}

impl From<u8> for RegisterPair {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::BC,
            1 => Self::DE,
            2 => Self::FH,
            3 => Self::SP,
            _ => panic!("{value} is not a valid for a register pair"),
        }
    }
}

pub enum Instruction {
    NOP,
    LXI(RegisterPair, DatLo, DatHi),
    STAX(RegisterPair),
    INX(RegisterPair),
    INR(Register),
    DCR(Register),
    MVI(Register, Data),
    DAD(RegisterPair),
    LDAX(RegisterPair),
    DCX(RegisterPair),
    RLC,
    RRC,
    RAL,
    RAR,
    SHLD(AddLo, AddHi),
    DAA,
    LHLD(AddLo, AddHi),
    CMA,
    STA(AddLo, AddHi),
}

pub struct Disassembler<I>
where
    I: Iterator<Item = u8>,
{
    bytes: I,
}

impl<B> Disassembler<B>
where
    B: Iterator<Item = u8>,
{
    fn new(bytes: B) -> Self {
        Self { bytes }
    }
}

impl<B> Iterator for Disassembler<B>
where
    B: Iterator<Item = u8>,
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let opcode = self.bytes.next()?;

        let rp_mask = !0x30;
        let rp = (opcode & !rp_mask >> 4).into();

        let ddd_mask = !0x38;
        let ddd = (opcode & !ddd_mask >> 5).into();

        return if opcode ^ 0x00 == 0 {
            Some(Instruction::NOP)
        } else if opcode & rp_mask ^ 0x01 == 0 {
            Some(Instruction::LXI(rp, self.bytes.next()?, self.bytes.next()?))
        } else if opcode & rp_mask ^ 0x02 == 0 {
            Some(Instruction::STAX(rp))
        } else if opcode & rp_mask ^ 0x03 == 0 {
            Some(Instruction::INX(rp))
        } else if opcode & ddd_mask ^ 0x04 == 0 {
            Some(Instruction::INR(ddd))
        } else if opcode & ddd_mask ^ 0x05 == 0 {
            Some(Instruction::DCR(ddd))
        } else if opcode & ddd_mask ^ 0x06 == 0 {
            Some(Instruction::MVI(ddd, self.bytes.next()?))
        } else if opcode & rp_mask ^ 0x09 == 0 {
            Some(Instruction::DAD(rp))
        } else if opcode & rp_mask ^ 0x0A == 0 {
            Some(Instruction::LDAX(rp))
        } else if opcode & rp_mask ^ 0x0B == 0 {
            Some(Instruction::DCX(rp))
        } else if opcode ^ 0x07 == 0 {
            Some(Instruction::RLC)
        } else if opcode ^ 0x0F == 0 {
            Some(Instruction::RRC)
        } else if opcode ^ 0x17 == 0 {
            Some(Instruction::RAL)
        } else if opcode ^ 0x1F == 0 {
            Some(Instruction::RAR)
        } else if opcode ^ 0x22 == 0 {
            Some(Instruction::SHLD(self.bytes.next()?, self.bytes.next()?))
        } else if opcode ^ 0x27 == 0 {
            Some(Instruction::DAA)
        } else if opcode ^ 0x2A == 0 {
            Some(Instruction::LHLD(self.bytes.next()?, self.bytes.next()?))
        } else if opcode ^ 0x2A == 0 {
            Some(Instruction::CMA)
        } else if opcode ^ 0x32 == 0 {
            Some(Instruction::STA(self.bytes.next()?, self.bytes.next()?))
        } else if opcode ^ 0x32 == 0 {
            Some(Instruction::STA(self.bytes.next()?, self.bytes.next()?))
        } else {
            None
        };
    }
}
