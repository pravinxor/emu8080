pub type Data = u8;

pub type DatLo = Data;
pub type DatHi = Data;

pub type AddLo = DatLo;
pub type AddHi = DatHi;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Lxi(RegisterPair, DatLo, DatHi),
    Stax(RegisterPair),
    Inx(RegisterPair),
    Inr(Register),
    Dcr(Register),
    Mvi(Register, Data),
    Dad(RegisterPair),
    Ldax(RegisterPair),
    Dcx(RegisterPair),
    Rlc,
    Rrc,
    Ral,
    Rar,
    Shld(AddLo, AddHi),
    Daa,
    Lhld(AddLo, AddHi),
    Cma,
    Sta(AddLo, AddHi),
    Stc,
    Idfk, // keep this for debugging
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
    pub fn new(bytes: B) -> Self {
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

        if opcode == 0 {
            Some(Instruction::Nop)
        } else if opcode & rp_mask ^ 0x01 == 0 {
            Some(Instruction::Lxi(rp, self.bytes.next()?, self.bytes.next()?))
        } else if opcode & rp_mask ^ 0x02 == 0 {
            Some(Instruction::Stax(rp))
        } else if opcode & rp_mask ^ 0x03 == 0 {
            Some(Instruction::Inx(rp))
        } else if opcode & ddd_mask ^ 0x04 == 0 {
            Some(Instruction::Inr(ddd))
        } else if opcode & ddd_mask ^ 0x05 == 0 {
            Some(Instruction::Dcr(ddd))
        } else if opcode & ddd_mask ^ 0x06 == 0 {
            Some(Instruction::Mvi(ddd, self.bytes.next()?))
        } else if opcode & rp_mask ^ 0x09 == 0 {
            Some(Instruction::Dad(rp))
        } else if opcode & rp_mask ^ 0x0A == 0 {
            Some(Instruction::Ldax(rp))
        } else if opcode & rp_mask ^ 0x0B == 0 {
            Some(Instruction::Dcx(rp))
        } else if opcode ^ 0x07 == 0 {
            Some(Instruction::Rlc)
        } else if opcode ^ 0x0F == 0 {
            Some(Instruction::Rrc)
        } else if opcode ^ 0x17 == 0 {
            Some(Instruction::Ral)
        } else if opcode ^ 0x1F == 0 {
            Some(Instruction::Rar)
        } else if opcode ^ 0x22 == 0 {
            Some(Instruction::Shld(self.bytes.next()?, self.bytes.next()?))
        } else if opcode ^ 0x27 == 0 {
            Some(Instruction::Daa)
        } else if opcode ^ 0x2A == 0 {
            Some(Instruction::Lhld(self.bytes.next()?, self.bytes.next()?))
        } else if opcode ^ 0x2F == 0 {
            Some(Instruction::Cma)
        } else if opcode ^ 0x32 == 0 {
            Some(Instruction::Sta(self.bytes.next()?, self.bytes.next()?))
        } else if opcode ^ 0x37 == 0 {
            Some(Instruction::Stc)
        } else {
            Some(Instruction::Idfk)
        }
    }
}
