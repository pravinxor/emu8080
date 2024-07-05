use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use num_enum::{FromPrimitive, IntoPrimitive};

pub type Data = u8;
pub type Port = u8;

pub type XData = u16;
pub type Addr = u16;

#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum AluMode {
    #[num_enum(default)]
    Add,
    Adc,
    Sub,
    Sbb,
    Ana,
    Xra,
    Ora,
    Cmp,
}

#[derive(Debug, Clone, Copy, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Register {
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    #[num_enum(default)]
    A,
}

#[derive(Debug, Clone, Copy, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum RegisterPair {
    BC,
    DE,
    FH,
    SP, // Stack pointer
    #[num_enum(default)]
    Invalid,
}

#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum CarryCode {
    #[num_enum(default)]
    Nz,
    Z,
    Nc,
    C,
    Po,
    Pe,
    P,
    N,
}

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Lxi(RegisterPair, XData),
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
    Shld(Addr),
    Daa,
    Lhld(Addr),
    Cma,
    Sta(Addr),
    Stc,
    Lda(Addr),
    Cmc,
    Mov(Register, Register),
    Hlt,
    AluR(AluMode, Register),
    Rcc(CarryCode),
    Pop(RegisterPair),
    Jcc(CarryCode, Addr),
    Jmp(Addr),
    Ccc(CarryCode, Addr),
    Push(RegisterPair),
    AluI(AluMode, Data),
    Rst(u8),
    Ret,
    Call(Addr),
    Out(Port),
    In(Port),
    Xthl,
    Pchl,
    Di,
    Sphl,
    Ei,
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
        let rp = ((opcode & !rp_mask) >> 4).into();

        let cc_mask = !0x30;
        let cc = ((opcode & !cc_mask) >> 4).into();

        let alu_mask = !0x38;
        let alu = ((opcode & !alu_mask) >> 3).into();

        let ddd_mask = !0x38;
        let ddd = ((opcode & !ddd_mask) >> 3).into();

        let n_mask = !0x38;
        let n = (opcode & !n_mask) >> 3;

        let sss_mask = !0x07;
        let sss = (opcode & !sss_mask).into();

        if opcode == 0 {
            Some(Instruction::Nop)
        } else if opcode & rp_mask ^ 0x01 == 0 {
            Some(Instruction::Lxi(
                rp,
                u16::from_le_bytes([self.bytes.next()?, self.bytes.next()?]),
            ))
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
            Some(Instruction::Shld(u16::from_le_bytes([
                self.bytes.next()?,
                self.bytes.next()?,
            ])))
        } else if opcode ^ 0x27 == 0 {
            Some(Instruction::Daa)
        } else if opcode ^ 0x2A == 0 {
            Some(Instruction::Lhld(u16::from_le_bytes([
                self.bytes.next()?,
                self.bytes.next()?,
            ])))
        } else if opcode ^ 0x2F == 0 {
            Some(Instruction::Cma)
        } else if opcode ^ 0x32 == 0 {
            Some(Instruction::Sta(u16::from_le_bytes([
                self.bytes.next()?,
                self.bytes.next()?,
            ])))
        } else if opcode ^ 0x37 == 0 {
            Some(Instruction::Stc)
        } else if opcode ^ 0x3A == 0 {
            Some(Instruction::Lda(u16::from_le_bytes([
                self.bytes.next()?,
                self.bytes.next()?,
            ])))
        } else if opcode ^ 0x3F == 0 {
            Some(Instruction::Cmc)
        } else if opcode & ddd_mask & sss_mask ^ 0x40 == 0 {
            Some(Instruction::Mov(ddd, sss))
        } else if opcode ^ 0x76 == 0 {
            Some(Instruction::Hlt)
        } else if opcode & alu_mask & sss_mask ^ 0x80 == 0 {
            Some(Instruction::AluR(alu, sss))
        } else if opcode & cc_mask ^ 0xC0 == 0 {
            Some(Instruction::Rcc(cc))
        } else if opcode & rp_mask ^ 0xC1 == 0 {
            Some(Instruction::Pop(rp))
        } else if opcode & cc_mask ^ 0xC2 == 0 {
            Some(Instruction::Jcc(
                cc,
                u16::from_le_bytes([self.bytes.next()?, self.bytes.next()?]),
            ))
        } else if opcode ^ 0xC3 == 0 {
            Some(Instruction::Jmp(u16::from_le_bytes([
                self.bytes.next()?,
                self.bytes.next()?,
            ])))
        } else if opcode & cc_mask ^ 0xC4 == 0 {
            Some(Instruction::Ccc(
                cc,
                u16::from_le_bytes([self.bytes.next()?, self.bytes.next()?]),
            ))
        } else if opcode & rp_mask ^ 0xC5 == 0 {
            Some(Instruction::Push(rp))
        } else if opcode & alu_mask ^ 0xC6 == 0 {
            Some(Instruction::AluI(alu, self.bytes.next()?))
        } else if opcode & n_mask ^ 0xC7 == 0 {
            Some(Instruction::Rst(n))
        } else if opcode ^ 0xC9 == 0 {
            Some(Instruction::Ret)
        } else if opcode ^ 0xCD == 0 {
            Some(Instruction::Call(u16::from_le_bytes([
                self.bytes.next()?,
                self.bytes.next()?,
            ])))
        } else if opcode ^ 0xD3 == 0 {
            Some(Instruction::Out(self.bytes.next()?))
        } else if opcode ^ 0xDB == 0 {
            Some(Instruction::In(self.bytes.next()?))
        } else if opcode ^ 0xE3 == 0 {
            Some(Instruction::Xthl)
        } else if opcode ^ 0xE9 == 0 {
            Some(Instruction::Pchl)
        } else if opcode ^ 0xF3 == 0 {
            Some(Instruction::Di)
        } else if opcode ^ 0xF9 == 0 {
            Some(Instruction::Sphl)
        } else if opcode ^ 0xFB == 0 {
            Some(Instruction::Ei)
        } else {
            Some(Instruction::Nop)
        }
    }
}
