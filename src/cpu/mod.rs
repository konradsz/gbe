mod instruction;
mod mmu;
mod registers;

use self::{instruction::*, mmu::Mmu, registers::Registers};

pub struct Cpu {
    registers: Registers,
    mmu: Mmu,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            mmu: Mmu::new(),
        }
    }

    fn step(&mut self) {
        let opcode = self.mmu.read_byte(self.registers.get_and_increment_pc());

        let instruction;
        if opcode == 0xCB {
            let prefixed_opcode = self.mmu.read_byte(self.registers.get_and_increment_pc());
            instruction = self.decode_prefixed_opcode(prefixed_opcode);
        } else {
            instruction = self.decode_opcode(opcode);
        }
        self.execute_instruction(instruction);
    }

    fn read_word(&self) -> u16 {
        let lsb = self.mmu.read_byte(self.registers.get_pc());
        let msb = self.mmu.read_byte(self.registers.get_pc() + 1);
        u16::from(msb) << 8 | u16::from(lsb)
    }

    // move back to instruction.rs with cpu as argument (problem with decrement/increment hl)
    fn decode_opcode(&mut self, opcode: u8) -> Instruction {
        // constant for 0xFF00 ?
        match opcode {
            0x06 => Instruction::Load(LoadRegister::B, self.mmu.read_byte(self.registers.get_pc())),
            0x0E => Instruction::Load(LoadRegister::C, self.mmu.read_byte(self.registers.get_pc())),
            0x16 => Instruction::Load(LoadRegister::D, self.mmu.read_byte(self.registers.get_pc())),
            0x1E => Instruction::Load(LoadRegister::E, self.mmu.read_byte(self.registers.get_pc())),
            0x26 => Instruction::Load(LoadRegister::H, self.mmu.read_byte(self.registers.get_pc())),
            0x2E => Instruction::Load(LoadRegister::L, self.mmu.read_byte(self.registers.get_pc())),
            0x7F => Instruction::Load(LoadRegister::A, self.registers.get_a()),
            0x78 => Instruction::Load(LoadRegister::A, self.registers.get_b()),
            0x79 => Instruction::Load(LoadRegister::A, self.registers.get_c()),
            0x7A => Instruction::Load(LoadRegister::A, self.registers.get_d()),
            0x7B => Instruction::Load(LoadRegister::A, self.registers.get_e()),
            0x7C => Instruction::Load(LoadRegister::A, self.registers.get_h()),
            0x7D => Instruction::Load(LoadRegister::A, self.registers.get_l()),
            0x7E => Instruction::Load(LoadRegister::A, self.mmu.read_byte(self.registers.get_hl())),
            0x0A => Instruction::Load(LoadRegister::A, self.mmu.read_byte(self.registers.get_bc())),
            0x1A => Instruction::Load(LoadRegister::A, self.mmu.read_byte(self.registers.get_de())),
            0xFA => Instruction::Load(LoadRegister::A, self.mmu.read_byte(self.read_word())),
            0x3E => Instruction::Load(LoadRegister::A, self.mmu.read_byte(self.registers.get_pc())),
            0x40 => Instruction::Load(LoadRegister::B, self.registers.get_b()),
            0x41 => Instruction::Load(LoadRegister::B, self.registers.get_c()),
            0x42 => Instruction::Load(LoadRegister::B, self.registers.get_d()),
            0x43 => Instruction::Load(LoadRegister::B, self.registers.get_e()),
            0x44 => Instruction::Load(LoadRegister::B, self.registers.get_h()),
            0x45 => Instruction::Load(LoadRegister::B, self.registers.get_l()),
            0x46 => Instruction::Load(LoadRegister::B, self.mmu.read_byte(self.registers.get_hl())),
            0x48 => Instruction::Load(LoadRegister::C, self.registers.get_b()),
            0x49 => Instruction::Load(LoadRegister::C, self.registers.get_c()),
            0x4A => Instruction::Load(LoadRegister::C, self.registers.get_d()),
            0x4B => Instruction::Load(LoadRegister::C, self.registers.get_e()),
            0x4C => Instruction::Load(LoadRegister::C, self.registers.get_h()),
            0x4D => Instruction::Load(LoadRegister::C, self.registers.get_l()),
            0x4E => Instruction::Load(LoadRegister::C, self.mmu.read_byte(self.registers.get_hl())),
            0x50 => Instruction::Load(LoadRegister::D, self.registers.get_b()),
            0x51 => Instruction::Load(LoadRegister::D, self.registers.get_c()),
            0x52 => Instruction::Load(LoadRegister::D, self.registers.get_d()),
            0x53 => Instruction::Load(LoadRegister::D, self.registers.get_e()),
            0x54 => Instruction::Load(LoadRegister::D, self.registers.get_h()),
            0x55 => Instruction::Load(LoadRegister::D, self.registers.get_l()),
            0x56 => Instruction::Load(LoadRegister::D, self.mmu.read_byte(self.registers.get_hl())),
            0x58 => Instruction::Load(LoadRegister::E, self.registers.get_b()),
            0x59 => Instruction::Load(LoadRegister::E, self.registers.get_c()),
            0x5A => Instruction::Load(LoadRegister::E, self.registers.get_d()),
            0x5B => Instruction::Load(LoadRegister::E, self.registers.get_e()),
            0x5C => Instruction::Load(LoadRegister::E, self.registers.get_h()),
            0x5D => Instruction::Load(LoadRegister::E, self.registers.get_l()),
            0x5E => Instruction::Load(LoadRegister::E, self.mmu.read_byte(self.registers.get_hl())),
            0x60 => Instruction::Load(LoadRegister::H, self.registers.get_b()),
            0x61 => Instruction::Load(LoadRegister::H, self.registers.get_c()),
            0x62 => Instruction::Load(LoadRegister::H, self.registers.get_d()),
            0x63 => Instruction::Load(LoadRegister::H, self.registers.get_e()),
            0x64 => Instruction::Load(LoadRegister::H, self.registers.get_h()),
            0x65 => Instruction::Load(LoadRegister::H, self.registers.get_l()),
            0x66 => Instruction::Load(LoadRegister::H, self.mmu.read_byte(self.registers.get_hl())),
            0x68 => Instruction::Load(LoadRegister::L, self.registers.get_b()),
            0x69 => Instruction::Load(LoadRegister::L, self.registers.get_c()),
            0x6A => Instruction::Load(LoadRegister::L, self.registers.get_d()),
            0x6B => Instruction::Load(LoadRegister::L, self.registers.get_e()),
            0x6C => Instruction::Load(LoadRegister::L, self.registers.get_h()),
            0x6D => Instruction::Load(LoadRegister::L, self.registers.get_l()),
            0x6E => Instruction::Load(LoadRegister::L, self.mmu.read_byte(self.registers.get_hl())),
            0x70 => Instruction::LoadToMemory(self.registers.get_hl(), LoadRegister::B),
            0x71 => Instruction::LoadToMemory(self.registers.get_hl(), LoadRegister::C),
            0x72 => Instruction::LoadToMemory(self.registers.get_hl(), LoadRegister::D),
            0x73 => Instruction::LoadToMemory(self.registers.get_hl(), LoadRegister::E),
            0x74 => Instruction::LoadToMemory(self.registers.get_hl(), LoadRegister::H),
            0x75 => Instruction::LoadToMemory(self.registers.get_hl(), LoadRegister::L),
            0x36 => Instruction::LoadToMemoryFromMemory(self.registers.get_hl(), self.mmu.read_byte(self.registers.get_pc())),
            0x47 => Instruction::Load(LoadRegister::B, self.registers.get_a()),
            0x4F => Instruction::Load(LoadRegister::C, self.registers.get_a()),
            0x57 => Instruction::Load(LoadRegister::D, self.registers.get_a()),
            0x5F => Instruction::Load(LoadRegister::E, self.registers.get_a()),
            0x67 => Instruction::Load(LoadRegister::H, self.registers.get_a()),
            0x6F => Instruction::Load(LoadRegister::L, self.registers.get_a()),
            0x02 => Instruction::LoadToMemory(self.registers.get_bc(), LoadRegister::A),
            0x12 => Instruction::LoadToMemory(self.registers.get_de(), LoadRegister::A),
            0x77 => Instruction::LoadToMemory(self.registers.get_hl(), LoadRegister::A),
            0xEA => Instruction::LoadToMemory(self.read_word(), LoadRegister::A),
            0xF2 => Instruction::Load(LoadRegister::A, self.mmu.read_byte(0xFF00) + self.registers.get_c()),
            0xE2 => Instruction::LoadToMemory(0xFF00 + u16::from(self.registers.get_c()), LoadRegister::A),

            0x3A => Instruction::Load(LoadRegister::A, self.mmu.read_byte(self.registers.decrement_hl())),
            0x32 => Instruction::LoadToMemory(self.registers.decrement_hl(), LoadRegister::A),
            0x2A => Instruction::Load(LoadRegister::A, self.mmu.read_byte(self.registers.increment_hl())),
            0x22 => Instruction::LoadToMemory(self.registers.increment_hl(), LoadRegister::A),
            0xE0 => Instruction::LoadToMemory(0xFF00 + u16::from(self.mmu.read_byte(self.registers.get_pc())), LoadRegister::A),
            0xF0 => Instruction::Load(LoadRegister::A, self.mmu.read_byte(0xFF00 + u16::from(self.mmu.read_byte(self.registers.get_pc())))),
            0x01 => Instruction::Load16(TargetRegister16::BC, self.read_word()),
            0x11 => Instruction::Load16(TargetRegister16::DE, self.read_word()),
            0x21 => Instruction::Load16(TargetRegister16::HL, self.read_word()),
            0x31 => Instruction::Load16(TargetRegister16::SP, self.read_word()),
            0xF9 => Instruction::Load16(TargetRegister16::SP, self.registers.get_hl()),
            0xF8 => Instruction::Load16(TargetRegister16::HL,
                self.add_signed_byte_to_word(self.mmu.read_byte(self.registers.get_pc()) as i8, self.registers.get_sp())),
            0x08 => Instruction::LoadStackPointerToMemory(self.read_word()),

            0xF5 => Instruction::PushStack(StackOperationRegisters::AF),
            0xC5 => Instruction::PushStack(StackOperationRegisters::BC),
            0xD5 => Instruction::PushStack(StackOperationRegisters::DE),
            0xE5 => Instruction::PushStack(StackOperationRegisters::HL),
            0xF1 => Instruction::PopStack(StackOperationRegisters::AF),
            0xC1 => Instruction::PopStack(StackOperationRegisters::BC),
            0xD1 => Instruction::PopStack(StackOperationRegisters::DE),
            0xE1 => Instruction::PopStack(StackOperationRegisters::HL),

            0x87 => Instruction::Add8(self.registers.get_a()),
            0x80 => Instruction::Add8(self.registers.get_b()),
            0x81 => Instruction::Add8(self.registers.get_c()),
            0x82 => Instruction::Add8(self.registers.get_d()),
            0x83 => Instruction::Add8(self.registers.get_e()),
            0x84 => Instruction::Add8(self.registers.get_h()),
            0x85 => Instruction::Add8(self.registers.get_l()),
            0x86 => Instruction::Add8(self.mmu.read_byte(self.registers.get_hl())),
            0xC6 => Instruction::Add8(self.mmu.read_byte(self.registers.get_pc())),
            0x8F => Instruction::Adc(self.registers.get_a()),
            0x88 => Instruction::Adc(self.registers.get_b()),
            0x89 => Instruction::Adc(self.registers.get_c()),
            0x8A => Instruction::Adc(self.registers.get_d()),
            0x8B => Instruction::Adc(self.registers.get_e()),
            0x8C => Instruction::Adc(self.registers.get_h()),
            0x8D => Instruction::Adc(self.registers.get_l()),
            0x8E => Instruction::Adc(self.mmu.read_byte(self.registers.get_hl())),
            0xCE => Instruction::Adc(self.mmu.read_byte(self.registers.get_pc())),
            0x97 => Instruction::Sub(self.registers.get_a()),
            0x90 => Instruction::Sub(self.registers.get_b()),
            0x91 => Instruction::Sub(self.registers.get_c()),
            0x92 => Instruction::Sub(self.registers.get_d()),
            0x93 => Instruction::Sub(self.registers.get_e()),
            0x94 => Instruction::Sub(self.registers.get_h()),
            0x95 => Instruction::Sub(self.registers.get_l()),
            0x96 => Instruction::Sub(self.mmu.read_byte(self.registers.get_hl())),
            0xD6 => Instruction::Sub(self.mmu.read_byte(self.registers.get_pc())),
            0x9F => Instruction::Sbc(self.registers.get_a()),
            0x98 => Instruction::Sbc(self.registers.get_b()),
            0x99 => Instruction::Sbc(self.registers.get_c()),
            0x9A => Instruction::Sbc(self.registers.get_d()),
            0x9B => Instruction::Sbc(self.registers.get_e()),
            0x9C => Instruction::Sbc(self.registers.get_h()),
            0x9D => Instruction::Sbc(self.registers.get_l()),
            0x9E => Instruction::Sbc(self.mmu.read_byte(self.registers.get_hl())),
            0xA7 => Instruction::And(self.registers.get_a()),
            0xA0 => Instruction::And(self.registers.get_b()),
            0xA1 => Instruction::And(self.registers.get_c()),
            0xA2 => Instruction::And(self.registers.get_d()),
            0xA3 => Instruction::And(self.registers.get_e()),
            0xA4 => Instruction::And(self.registers.get_h()),
            0xA5 => Instruction::And(self.registers.get_l()),
            0xA6 => Instruction::And(self.mmu.read_byte(self.registers.get_hl())),
            0xE6 => Instruction::And(self.mmu.read_byte(self.registers.get_pc())),
            0xB7 => Instruction::Or(self.registers.get_a()),
            0xB0 => Instruction::Or(self.registers.get_b()),
            0xB1 => Instruction::Or(self.registers.get_c()),
            0xB2 => Instruction::Or(self.registers.get_d()),
            0xB3 => Instruction::Or(self.registers.get_e()),
            0xB4 => Instruction::Or(self.registers.get_h()),
            0xB5 => Instruction::Or(self.registers.get_l()),
            0xB6 => Instruction::Or(self.mmu.read_byte(self.registers.get_hl())),
            0xF6 => Instruction::Or(self.mmu.read_byte(self.registers.get_pc())),
            0xAF => Instruction::Xor(self.registers.get_a()),
            0xA8 => Instruction::Xor(self.registers.get_b()),
            0xA9 => Instruction::Xor(self.registers.get_c()),
            0xAA => Instruction::Xor(self.registers.get_d()),
            0xAB => Instruction::Xor(self.registers.get_e()),
            0xAC => Instruction::Xor(self.registers.get_h()),
            0xAD => Instruction::Xor(self.registers.get_l()),
            0xAE => Instruction::Xor(self.mmu.read_byte(self.registers.get_hl())),
            0xEE => Instruction::Xor(self.mmu.read_byte(self.registers.get_pc())),
            0xBF => Instruction::Cp(self.registers.get_a()),
            0xB8 => Instruction::Cp(self.registers.get_b()),
            0xB9 => Instruction::Cp(self.registers.get_c()),
            0xBA => Instruction::Cp(self.registers.get_d()),
            0xBB => Instruction::Cp(self.registers.get_e()),
            0xBC => Instruction::Cp(self.registers.get_h()),
            0xBD => Instruction::Cp(self.registers.get_l()),
            0xBE => Instruction::Cp(self.mmu.read_byte(self.registers.get_hl())),
            0xFE => Instruction::Cp(self.mmu.read_byte(self.registers.get_pc())),
            0x3C => Instruction::Inc8(IncDecTarget::A),
            0x04 => Instruction::Inc8(IncDecTarget::B),
            0x0C => Instruction::Inc8(IncDecTarget::C),
            0x14 => Instruction::Inc8(IncDecTarget::D),
            0x1C => Instruction::Inc8(IncDecTarget::E),
            0x24 => Instruction::Inc8(IncDecTarget::H),
            0x2C => Instruction::Inc8(IncDecTarget::L),
            0x34 => Instruction::Inc8(IncDecTarget::HL),
            0x3D => Instruction::Dec8(IncDecTarget::A),
            0x05 => Instruction::Dec8(IncDecTarget::B),
            0x0D => Instruction::Dec8(IncDecTarget::C),
            0x15 => Instruction::Dec8(IncDecTarget::D),
            0x1D => Instruction::Dec8(IncDecTarget::E),
            0x25 => Instruction::Dec8(IncDecTarget::H),
            0x2D => Instruction::Dec8(IncDecTarget::L),
            0x35 => Instruction::Dec8(IncDecTarget::HL),
            0x13 => Instruction::Inc16(TargetRegister16::DE),
            0x03 => Instruction::Inc16(TargetRegister16::BC),
            0x23 => Instruction::Inc16(TargetRegister16::HL),
            0x33 => Instruction::Inc16(TargetRegister16::SP),
            0x0B => Instruction::Dec16(TargetRegister16::BC),
            0x1B => Instruction::Dec16(TargetRegister16::DE),
            0x2B => Instruction::Dec16(TargetRegister16::HL),
            0x3B => Instruction::Dec16(TargetRegister16::SP),
            0x09 => Instruction::AddHL(self.registers.get_bc()),
            0x19 => Instruction::AddHL(self.registers.get_de()),
            0x29 => Instruction::AddHL(self.registers.get_hl()),
            0x39 => Instruction::AddHL(self.registers.get_sp()),
            0xE8 => Instruction::Load16(TargetRegister16::SP,
                self.add_signed_byte_to_word(self.mmu.read_byte(self.registers.get_pc()) as i8, self.registers.get_sp())),
            0x27 => panic!("DAA instruction not implemented"),
            0x2F => Instruction::ComplementA,
            0x3F => Instruction::ComplementCarryFlag,
            0x37 => Instruction::SetCarryFlag,
            0x00 => Instruction::Nop,
            0x76 => panic!("HALT instruction not implemented"),
            0x10 => panic!("STOP instruction not implemented"),
            0xF3 => panic!("DI instruction not implemented"),
            0xFB => panic!("EI instruction not implemented"),
            0x07 => Instruction::Rlc(IncDecTarget::A),
            0x17 => Instruction::Rl(IncDecTarget::A),
            0x0F => Instruction::Rrc(IncDecTarget::A),
            0x1F => Instruction::Rr(IncDecTarget::A),
            _ => panic!("unknown opcode {}", opcode)
        }
    }

    fn decode_prefixed_opcode(&mut self, opcode: u8) -> Instruction {
        match opcode {
            0x37 => Instruction::Swap(IncDecTarget::A),
            0x30 => Instruction::Swap(IncDecTarget::B),
            0x31 => Instruction::Swap(IncDecTarget::C),
            0x32 => Instruction::Swap(IncDecTarget::D),
            0x33 => Instruction::Swap(IncDecTarget::E),
            0x34 => Instruction::Swap(IncDecTarget::H),
            0x35 => Instruction::Swap(IncDecTarget::L),
            0x36 => Instruction::Swap(IncDecTarget::HL),
            0x07 => Instruction::Rlc(IncDecTarget::A),
            0x00 => Instruction::Rlc(IncDecTarget::B),
            0x01 => Instruction::Rlc(IncDecTarget::C),
            0x02 => Instruction::Rlc(IncDecTarget::D),
            0x03 => Instruction::Rlc(IncDecTarget::E),
            0x04 => Instruction::Rlc(IncDecTarget::H),
            0x05 => Instruction::Rlc(IncDecTarget::L),
            0x06 => Instruction::Rlc(IncDecTarget::HL),
            0x17 => Instruction::Rl(IncDecTarget::A),
            0x10 => Instruction::Rl(IncDecTarget::B),
            0x11 => Instruction::Rl(IncDecTarget::C),
            0x12 => Instruction::Rl(IncDecTarget::D),
            0x13 => Instruction::Rl(IncDecTarget::E),
            0x14 => Instruction::Rl(IncDecTarget::H),
            0x15 => Instruction::Rl(IncDecTarget::L),
            0x16 => Instruction::Rl(IncDecTarget::HL),
            0x0F => Instruction::Rrc(IncDecTarget::A),
            0x08 => Instruction::Rrc(IncDecTarget::B),
            0x09 => Instruction::Rrc(IncDecTarget::C),
            0x0A => Instruction::Rrc(IncDecTarget::D),
            0x0B => Instruction::Rrc(IncDecTarget::E),
            0x0C => Instruction::Rrc(IncDecTarget::H),
            0x0D => Instruction::Rrc(IncDecTarget::L),
            0x0E => Instruction::Rrc(IncDecTarget::HL),
            0x1F => Instruction::Rr(IncDecTarget::A),
            0x18 => Instruction::Rr(IncDecTarget::B),
            0x19 => Instruction::Rr(IncDecTarget::C),
            0x1A => Instruction::Rr(IncDecTarget::D),
            0x1B => Instruction::Rr(IncDecTarget::E),
            0x1C => Instruction::Rr(IncDecTarget::H),
            0x1D => Instruction::Rr(IncDecTarget::L),
            0x1E => Instruction::Rr(IncDecTarget::HL),
            _ => panic!("unknown opcode {}", opcode)
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match &instruction {
            Instruction::Load(target, value) => {
                match target {
                    LoadRegister::A => self.registers.set_a(*value),
                    LoadRegister::B => self.registers.set_b(*value),
                    LoadRegister::C => self.registers.set_c(*value),
                    LoadRegister::D => self.registers.set_d(*value),
                    LoadRegister::E => self.registers.set_e(*value),
                    LoadRegister::H => self.registers.set_h(*value),
                    LoadRegister::L => self.registers.set_l(*value),
                }
            }
            Instruction::LoadToMemory(address, source) => {
                match source {
                    LoadRegister::A => self.mmu.write_byte(*address, self.registers.get_a()),
                    LoadRegister::B => self.mmu.write_byte(*address, self.registers.get_b()),
                    LoadRegister::C => self.mmu.write_byte(*address, self.registers.get_c()),
                    LoadRegister::D => self.mmu.write_byte(*address, self.registers.get_d()),
                    LoadRegister::E => self.mmu.write_byte(*address, self.registers.get_e()),
                    LoadRegister::H => self.mmu.write_byte(*address, self.registers.get_h()),
                    LoadRegister::L => self.mmu.write_byte(*address, self.registers.get_l()),
                }
            }
            Instruction::LoadToMemoryFromMemory(address, value) => self.mmu.write_byte(*address, *value),
            Instruction::Load16(target, value) => {
                match target {
                    TargetRegister16::BC => self.registers.set_bc(*value),
                    TargetRegister16::DE => self.registers.set_de(*value),
                    TargetRegister16::HL => self.registers.set_hl(*value),
                    TargetRegister16::SP => self.registers.set_sp(*value),
                }
            }
            Instruction::LoadStackPointerToMemory(address) => {
                let sp_lsb = (self.registers.get_sp() & 0x00FF) as u8;
                let sp_msb = (self.registers.get_sp() >> 8) as u8;
                self.mmu.write_byte(*address, sp_lsb);
                self.mmu.write_byte(*address + 1, sp_msb);
            }
            Instruction::PushStack(register) => {
                match register {
                    StackOperationRegisters::AF => self.push(self.registers.get_af()),
                    StackOperationRegisters::BC => self.push(self.registers.get_bc()),
                    StackOperationRegisters::DE => self.push(self.registers.get_de()),
                    StackOperationRegisters::HL => self.push(self.registers.get_hl()),
                }
            }
            Instruction::PopStack(register) => {
                let value = self.pop();
                match register {
                    StackOperationRegisters::AF => self.registers.set_af(value),
                    StackOperationRegisters::BC => self.registers.set_bc(value),
                    StackOperationRegisters::DE => self.registers.set_de(value),
                    StackOperationRegisters::HL => self.registers.set_hl(value),
                }
            }
            Instruction::Add8(register_value) => self.add8(*register_value, false),
            Instruction::Adc(register_value) => self.add8(*register_value, true),
            Instruction::Sub(register_value) => self.sub(*register_value, false),
            Instruction::Sbc(register_value) => self.sub(*register_value, true),
            Instruction::And(register_value) => self.and(*register_value),
            Instruction::Or(register_value) => self.or(*register_value),
            Instruction::Xor(register_value) => self.xor(*register_value),
            Instruction::Cp(register_value) => self.compare(*register_value),
            Instruction::Inc8(target) | Instruction::Dec8(target) => {
                let perform_operation = |cpu: &mut Cpu, instruction: &Instruction, value| -> u8 {
                    match instruction {
                        Instruction::Inc8(_) => cpu.increment(value),
                        Instruction::Dec8(_) => cpu.decrement(value),
                        _ => 0,
                    }
                };

                match target {
                    IncDecTarget::A => {
                        let value = perform_operation(self, &instruction, self.registers.get_a());
                        self.registers.set_a(value);
                    }
                    IncDecTarget::B => {
                        let value = perform_operation(self, &instruction, self.registers.get_b());
                        self.registers.set_b(value);
                    }
                    IncDecTarget::C => {
                        let value = perform_operation(self, &instruction, self.registers.get_c());
                        self.registers.set_c(value);
                    }
                    IncDecTarget::D => {
                        let value = perform_operation(self, &instruction, self.registers.get_d());
                        self.registers.set_d(value);
                    }
                    IncDecTarget::E => {
                        let value = perform_operation(self, &instruction, self.registers.get_e());
                        self.registers.set_e(value);
                    }
                    IncDecTarget::H => {
                        let value = perform_operation(self, &instruction, self.registers.get_h());
                        self.registers.set_h(value);
                    }
                    IncDecTarget::L => {
                        let value = perform_operation(self, &instruction, self.registers.get_l());
                        self.registers.set_l(value);
                    }
                    IncDecTarget::HL => {
                        let address = self.registers.get_hl();
                        let value =
                            perform_operation(self, &instruction, self.mmu.read_byte(address));
                        self.mmu.write_byte(address, value);
                    }
                }
            }
            Instruction::Inc16(target) | Instruction::Dec16(target) => {
                let perform_operation = |instruction: &Instruction, value: u16| -> u16 {
                    match instruction {
                        Instruction::Inc16(_) => value + 1,
                        Instruction::Dec16(_) => value - 1,
                        _ => 0,
                    }
                };

                match target {
                    TargetRegister16::BC => {
                        let value = perform_operation(&instruction, self.registers.get_bc());
                        self.registers.set_bc(value);
                    }
                    TargetRegister16::DE => {
                        let value = perform_operation(&instruction, self.registers.get_de());
                        self.registers.set_de(value);
                    }
                    TargetRegister16::HL => {
                        let value = perform_operation(&instruction, self.registers.get_hl());
                        self.registers.set_hl(value);
                    }
                    TargetRegister16::SP => {
                        let value = perform_operation(&instruction, self.registers.get_sp());
                        self.registers.set_sp(value);
                    }
                }
            },
            Instruction::AddHL(register_value) => self.add16(*register_value),
            Instruction::Swap(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.swap(self.registers.get_a());
                        self.registers.set_a(value);
                    }
                    IncDecTarget::B => {
                        let value = self.swap(self.registers.get_b());
                        self.registers.set_b(value);
                    }
                    IncDecTarget::C => {
                        let value = self.swap(self.registers.get_c());
                        self.registers.set_c(value);
                    }
                    IncDecTarget::D => {
                        let value = self.swap(self.registers.get_d());
                        self.registers.set_d(value);
                    }
                    IncDecTarget::E => {
                        let value = self.swap(self.registers.get_e());
                        self.registers.set_e(value);
                    }
                    IncDecTarget::H => {
                        let value = self.swap(self.registers.get_h());
                        self.registers.set_h(value);
                    }
                    IncDecTarget::L => {
                        let value = self.swap(self.registers.get_l());
                        self.registers.set_l(value);
                    }
                    IncDecTarget::HL => {
                        let value = self.swap(self.mmu.read_byte(self.registers.get_hl()));
                        self.mmu.write_byte(self.registers.get_hl(), value);
                    }
                }
            },
            Instruction::ComplementA => self.complement_a(),
            Instruction::ComplementCarryFlag => self.complement_carry_flag(),
            Instruction::SetCarryFlag => {
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(false);
                self.registers.set_c_flag(true);
            }
            Instruction::Nop => (),
            Instruction::Rlc(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.rlc(self.registers.get_a());
                        self.registers.set_a(value);
                    }
                    IncDecTarget::B => {
                        let value = self.rlc(self.registers.get_b());
                        self.registers.set_b(value);
                    }
                    IncDecTarget::C => {
                        let value = self.rlc(self.registers.get_c());
                        self.registers.set_c(value);
                    }
                    IncDecTarget::D => {
                        let value = self.rlc(self.registers.get_d());
                        self.registers.set_d(value);
                    }
                    IncDecTarget::E => {
                        let value = self.rlc(self.registers.get_e());
                        self.registers.set_e(value);
                    }
                    IncDecTarget::H => {
                        let value = self.rlc(self.registers.get_h());
                        self.registers.set_h(value);
                    }
                    IncDecTarget::L => {
                        let value = self.rlc(self.registers.get_l());
                        self.registers.set_l(value);
                    }
                    IncDecTarget::HL => {
                        let value = self.rlc(self.mmu.read_byte(self.registers.get_hl()));
                        self.mmu.write_byte(self.registers.get_hl(), value);
                    }
                }
            }
            Instruction::Rl(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.rl(self.registers.get_a());
                        self.registers.set_a(value);
                    }
                    IncDecTarget::B => {
                        let value = self.rl(self.registers.get_b());
                        self.registers.set_b(value);
                    }
                    IncDecTarget::C => {
                        let value = self.rl(self.registers.get_c());
                        self.registers.set_c(value);
                    }
                    IncDecTarget::D => {
                        let value = self.rl(self.registers.get_d());
                        self.registers.set_d(value);
                    }
                    IncDecTarget::E => {
                        let value = self.rl(self.registers.get_e());
                        self.registers.set_e(value);
                    }
                    IncDecTarget::H => {
                        let value = self.rl(self.registers.get_h());
                        self.registers.set_h(value);
                    }
                    IncDecTarget::L => {
                        let value = self.rl(self.registers.get_l());
                        self.registers.set_l(value);
                    }
                    IncDecTarget::HL => {
                        let value = self.rl(self.mmu.read_byte(self.registers.get_hl()));
                        self.mmu.write_byte(self.registers.get_hl(), value);
                    }
                }
            }
            Instruction::Rrc(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.rrc(self.registers.get_a());
                        self.registers.set_a(value);
                    }
                    IncDecTarget::B => {
                        let value = self.rrc(self.registers.get_b());
                        self.registers.set_b(value);
                    }
                    IncDecTarget::C => {
                        let value = self.rrc(self.registers.get_c());
                        self.registers.set_c(value);
                    }
                    IncDecTarget::D => {
                        let value = self.rrc(self.registers.get_d());
                        self.registers.set_d(value);
                    }
                    IncDecTarget::E => {
                        let value = self.rrc(self.registers.get_e());
                        self.registers.set_e(value);
                    }
                    IncDecTarget::H => {
                        let value = self.rrc(self.registers.get_h());
                        self.registers.set_h(value);
                    }
                    IncDecTarget::L => {
                        let value = self.rrc(self.registers.get_l());
                        self.registers.set_l(value);
                    }
                    IncDecTarget::HL => {
                        let value = self.rrc(self.mmu.read_byte(self.registers.get_hl()));
                        self.mmu.write_byte(self.registers.get_hl(), value);
                    }
                }
            }
            Instruction::Rr(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.rr(self.registers.get_a());
                        self.registers.set_a(value);
                    }
                    IncDecTarget::B => {
                        let value = self.rr(self.registers.get_b());
                        self.registers.set_b(value);
                    }
                    IncDecTarget::C => {
                        let value = self.rr(self.registers.get_c());
                        self.registers.set_c(value);
                    }
                    IncDecTarget::D => {
                        let value = self.rr(self.registers.get_d());
                        self.registers.set_d(value);
                    }
                    IncDecTarget::E => {
                        let value = self.rr(self.registers.get_e());
                        self.registers.set_e(value);
                    }
                    IncDecTarget::H => {
                        let value = self.rr(self.registers.get_h());
                        self.registers.set_h(value);
                    }
                    IncDecTarget::L => {
                        let value = self.rr(self.registers.get_l());
                        self.registers.set_l(value);
                    }
                    IncDecTarget::HL => {
                        let value = self.rr(self.mmu.read_byte(self.registers.get_hl()));
                        self.mmu.write_byte(self.registers.get_hl(), value);
                    }
                }
            }
        }
    }

    fn push(&mut self, value: u16) {
        let lsb = (value & 0x00FF) as u8;
        let msb = (value >> 8) as u8;

        self.registers.decrement_sp();
        self.mmu.write_byte(self.registers.get_sp(), msb);
        self.registers.decrement_sp();
        self.mmu.write_byte(self.registers.get_sp(), lsb);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.mmu.read_byte(self.registers.get_sp());
        self.registers.increment_sp();
        let msb = self.mmu.read_byte(self.registers.get_sp());
        self.registers.increment_sp();
        u16::from(msb) << 8 | u16::from(lsb)
    }

    fn add8(&mut self, mut value: u8, with_carry: bool) {
        let current_value = self.registers.get_a();
        if with_carry && self.registers.get_c_flag() {
            value += 1;
        }
        let (new_value, overflowed) = current_value.overflowing_add(value);
        self.registers.set_a(new_value);
        self.registers.set_z_flag(new_value == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag((((current_value & 0xf) + (new_value & 0xf)) & 0x10) == 0x10);
        self.registers.set_c_flag(overflowed);
    }

    fn add_signed_byte_to_word(&mut self, byte: i8, word: u16) -> u16 {
        self.registers.set_z_flag(false);
        self.registers.set_n_flag(false);

        let abs_value = byte.abs() as u16;
        if byte >= 0 {
            self.registers.set_h_flag((word & 0xF) + (abs_value & 0xF) > 0xF);
            self.registers.set_c_flag((word & 0xFF) + abs_value > 0xFF);
            return word + abs_value;
        } else {
            let (result, overflowed) = word.overflowing_sub(abs_value);
            self.registers.set_h_flag(abs_value & 0xF > word & 0xF);
            self.registers.set_c_flag(overflowed);
            return result;
        }
    }

    fn sub(&mut self, mut value: u8, with_carry: bool) {
        let current_value = self.registers.get_a();
        if with_carry && self.registers.get_c_flag() {
            value += 1;
        }
        let (new_value, overflowed) = current_value.overflowing_sub(value);
        self.registers.set_a(new_value);
        self.registers.set_z_flag(new_value == 0);
        self.registers.set_n_flag(true);
        self.registers.set_h_flag(value & 0xF > current_value & 0xF);
        self.registers.set_c_flag(overflowed);
    }

    fn and(&mut self, value: u8) {
        let result = self.registers.get_a() & value;
        self.registers.set_a(result);
        self.registers.set_z_flag(result == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);
    }

    fn or(&mut self, value: u8) {
        let result = self.registers.get_a() | value;
        self.registers.set_a(result);
        self.registers.set_z_flag(result == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);
    }

    fn xor(&mut self, value: u8) {
        let result = self.registers.get_a() ^ value;
        self.registers.set_a(result);
        self.registers.set_z_flag(result == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);
    }

    fn compare(&mut self, value: u8) {
        let register_a = self.registers.get_a();
        self.registers.set_z_flag(register_a == value);
        self.registers.set_n_flag(true);
        self.registers.set_h_flag(register_a & 0xF < value & 0xF);
        self.registers.set_c_flag(register_a < value);
    }

    fn increment(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registers.set_z_flag(result == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(value & 0xF == 0xF);

        result
    }

    fn decrement(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.registers.set_z_flag(result == 0);
        self.registers.set_n_flag(true);
        self.registers.set_h_flag(value & 0xF == 0);

        result
    }

    fn add16(&mut self, value: u16) {
        let current_value = self.registers.get_hl();
        let (new_value, overflowed) = current_value.overflowing_add(value);
        self.registers.set_hl(new_value);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(overflowed);
        self.registers
            .set_c_flag((current_value & 0xFFF) + (value & 0xFFF) > 0xFFF);
    }

    fn swap(&mut self, value: u8) -> u8 {
        let lower_nibble = value & 0x0F;
        let upper_nibble = value >> 4;
        let result = lower_nibble << 4 | upper_nibble;
        self.registers.set_z_flag(result == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);
        result
    }

    fn complement_a(&mut self) {
        let mut value = self.registers.get_a();
        self.registers.set_a(!value);
        self.registers.set_n_flag(true);
        self.registers.set_h_flag(true);
    }

    fn complement_carry_flag(&mut self) {
        let value = self.registers.get_c_flag();
        self.registers.set_c_flag(!value);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
    }

    fn rlc(&mut self, value: u8) -> u8 {
        let c = value >> 7;
        let result = value << 1 | c;
        self.registers.set_z_flag(result == 0); // some says it should be false always (for A ?)
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(c != 0);
        result
    }

    fn rl(&mut self, value: u8) -> u8 {
        let c = u8::from(self.registers.get_c_flag());
        let result = value << 1 | c;
        self.registers.set_a(result);
        self.registers.set_z_flag(result == 0); // some says it should be false always (for A ?)
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value >> 7) != 0);
        result
    }

    fn rrc(&mut self, value: u8) -> u8 {
        let c: u8 = value & 1;
        let result = c << 7 | value >> 1;

        self.registers.set_a(result);
        self.registers.set_z_flag(result == 0); // some says it should be false always (for A ?)
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(c != 0);
        result
    }

    fn rr(&mut self, value: u8) -> u8 {
        let c = u8::from(self.registers.get_c_flag());
        let result = c << 7 | value >> 1;
        self.registers.set_a(result);
        self.registers.set_z_flag(result == 0); // some says it should be false always (for A ?)
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value & 1) != 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_load8_immediate_test() {
        let mut cpu = Cpu::new();
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xE0);
        let load_b = cpu.decode_opcode(0x06);
        cpu.execute_instruction(load_b);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xE1);
        let load_c = cpu.decode_opcode(0x0E);
        cpu.execute_instruction(load_c);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xE2);
        let load_d = cpu.decode_opcode(0x16);
        cpu.execute_instruction(load_d);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xE3);
        let load_e = cpu.decode_opcode(0x1E);
        cpu.execute_instruction(load_e);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xE4);
        let load_h = cpu.decode_opcode(0x26);
        cpu.execute_instruction(load_h);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xE5);
        let load_l = cpu.decode_opcode(0x2E);
        cpu.execute_instruction(load_l);

        assert_eq!(cpu.registers.get_b(), 0xE0);
        assert_eq!(cpu.registers.get_c(), 0xE1);
        assert_eq!(cpu.registers.get_d(), 0xE2);
        assert_eq!(cpu.registers.get_e(), 0xE3);
        assert_eq!(cpu.registers.get_h(), 0xE4);
        assert_eq!(cpu.registers.get_l(), 0xE5);

        cpu.mmu.write_byte(0xFF00 + u16::from(cpu.mmu.read_byte(cpu.registers.get_pc())), 0xE6);
        let load_a = cpu.decode_opcode(0xF0);
        cpu.execute_instruction(load_a);
        assert_eq!(cpu.registers.get_a(), 0xE6);
    }

    #[test]
    fn cpu_load8_registers_test() {
        let mut cpu = Cpu::new();
        const ADDRESS: u16 = 0xABCD;
        cpu.registers.set_hl(ADDRESS);
        cpu.registers.set_bc(ADDRESS);
        cpu.registers.set_de(ADDRESS);

        // Load(A, A)
        cpu.registers.set_a(0xE0);
        let load_a_to_a = cpu.decode_opcode(0x7F);
        cpu.execute_instruction(load_a_to_a);
        assert_eq!(cpu.registers.get_a(), 0xE0);
        // Load(A, HL) & LDD(A, HL) & LDI(A, HL)
        cpu.mmu.write_byte(ADDRESS, 0xFA);
        let load_memory_hl_to_a = cpu.decode_opcode(0x7E);
        cpu.execute_instruction(load_memory_hl_to_a);
        assert_eq!(cpu.registers.get_a(), 0xFA);
        cpu.mmu.write_byte(ADDRESS, 0xF0);
        let load_memory_hl_to_a_decrement_hl = cpu.decode_opcode(0x3A);
        cpu.execute_instruction(load_memory_hl_to_a_decrement_hl);
        assert_eq!(cpu.registers.get_a(), 0xF0);
        assert_eq!(cpu.registers.get_hl(), ADDRESS - 1);
        cpu.mmu.write_byte(ADDRESS - 1, 0xF1);
        let load_memory_hl_to_a_increment_hl = cpu.decode_opcode(0x2A);
        cpu.execute_instruction(load_memory_hl_to_a_increment_hl);
        assert_eq!(cpu.registers.get_a(), 0xF1);
        assert_eq!(cpu.registers.get_hl(), ADDRESS);
        // Load(A, BC)
        cpu.mmu.write_byte(ADDRESS, 0xFB);
        let load_memory_bc_to_a = cpu.decode_opcode(0x0A);
        cpu.execute_instruction(load_memory_bc_to_a);
        assert_eq!(cpu.registers.get_a(), 0xFB);
        // Load(A, DE)
        cpu.mmu.write_byte(ADDRESS, 0xFC);
        let load_memory_de_to_a = cpu.decode_opcode(0x1A);
        cpu.execute_instruction(load_memory_de_to_a);
        assert_eq!(cpu.registers.get_a(), 0xFC);
        // Load(A, nn)
        cpu.mmu.write_byte(ADDRESS, 0xFD);
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xCD);
        cpu.mmu.write_byte(cpu.registers.get_pc() + 1, 0xAB);
        let load_bytes_to_a = cpu.decode_opcode(0xFA);
        cpu.execute_instruction(load_bytes_to_a);
        assert_eq!(cpu.registers.get_a(), 0xFD);
        // Load(A, #)
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xEA);
        let load_memory_pc_to_a = cpu.decode_opcode(0x3E);
        cpu.execute_instruction(load_memory_pc_to_a);
        assert_eq!(cpu.registers.get_a(), 0xEA);
        // Load(B, B)
        cpu.registers.set_b(0xE1);
        let load_b_to_b = cpu.decode_opcode(0x40);
        cpu.execute_instruction(load_b_to_b);
        assert_eq!(cpu.registers.get_b(), 0xE1);
        // Load(B, HL)
        cpu.mmu.write_byte(ADDRESS, 0xFE);
        let load_memory_to_b = cpu.decode_opcode(0x46);
        cpu.execute_instruction(load_memory_to_b);
        assert_eq!(cpu.registers.get_b(), 0xFE);
        // Load(C, C)
        cpu.registers.set_c(0xE2);
        let load_c_to_c = cpu.decode_opcode(0x49);
        cpu.execute_instruction(load_c_to_c);
        assert_eq!(cpu.registers.get_c(), 0xE2);
        // Load(C, HL)
        cpu.mmu.write_byte(ADDRESS, 0xFF);
        let load_memory_to_c = cpu.decode_opcode(0x4E);
        cpu.execute_instruction(load_memory_to_c);
        assert_eq!(cpu.registers.get_c(), 0xFF);
        // Load(D, D)
        cpu.registers.set_d(0xE3);
        let load_d_to_d = cpu.decode_opcode(0x52);
        cpu.execute_instruction(load_d_to_d);
        assert_eq!(cpu.registers.get_d(), 0xE3);
        // Load(D, HL)
        let load_memory_to_d = cpu.decode_opcode(0x56);
        cpu.execute_instruction(load_memory_to_d);
        assert_eq!(cpu.registers.get_d(), 0xFF);
        // Load(E, E)
        cpu.registers.set_e(0xE4);
        let load_e_to_e = cpu.decode_opcode(0x5B);
        cpu.execute_instruction(load_e_to_e);
        assert_eq!(cpu.registers.get_e(), 0xE4);
        // Load(E, HL)
        let load_memory_to_e = cpu.decode_opcode(0x5E);
        cpu.execute_instruction(load_memory_to_e);
        assert_eq!(cpu.registers.get_e(), 0xFF);
        // Load(H, H)
        cpu.registers.set_h(0xE5);
        let load_h_to_h = cpu.decode_opcode(0x64);
        cpu.execute_instruction(load_h_to_h);
        assert_eq!(cpu.registers.get_h(), 0xE5);
        // Load(H, HL)
        cpu.mmu.write_byte(cpu.registers.get_hl(), 0xFF); // h register changed, thus hl as well
        let load_memory_to_h = cpu.decode_opcode(0x66);
        cpu.execute_instruction(load_memory_to_h);
        assert_eq!(cpu.registers.get_h(), 0xFF);
        // Load(L, L)
        cpu.registers.set_l(0xE6);
        let load_l_to_l = cpu.decode_opcode(0x6D);
        cpu.execute_instruction(load_l_to_l);
        assert_eq!(cpu.registers.get_l(), 0xE6);
        // Load(L, HL)
        cpu.mmu.write_byte(cpu.registers.get_hl(), 0xFF); // l register changed, thus hl as well
        let load_memory_to_l = cpu.decode_opcode(0x6E);
        cpu.execute_instruction(load_memory_to_l);
        assert_eq!(cpu.registers.get_l(), 0xFF);

        cpu.registers.set_hl(ADDRESS);
        // Load(HL, n)
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xE8);
        let load_to_memory_from_pc = cpu.decode_opcode(0x36);
        cpu.execute_instruction(load_to_memory_from_pc);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), 0xE8);

        // Load(A, (C))
        cpu.mmu.write_byte(0xFF00, 0x1);
        cpu.registers.set_c(0x2);
        let load_to_a_0xff00_plus_c = cpu.decode_opcode(0xF2);
        cpu.execute_instruction(load_to_a_0xff00_plus_c);
        assert_eq!(cpu.registers.get_a(), 0x1 + 0x2);
    }

    #[test]
    fn cpu_load8_to_memory_test() {
        let mut cpu = Cpu::new();
        const ADDRESS: u16 = 0xABCD;
        cpu.registers.set_bc(ADDRESS);
        cpu.registers.set_de(ADDRESS);
        cpu.registers.set_hl(ADDRESS);

        // Load(BC, A)
        cpu.registers.set_a(0xE5);
        let load_to_memory_bc_from_a = cpu.decode_opcode(0x02);
        cpu.execute_instruction(load_to_memory_bc_from_a);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), 0xE5);
        // Load(DE, A)
        cpu.registers.set_a(0xE6);
        let load_to_memory_de_from_a = cpu.decode_opcode(0x12);
        cpu.execute_instruction(load_to_memory_de_from_a);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), 0xE6);
        // Load(HL, A) & LDD(HL, A) & LDI(HL, A)
        cpu.registers.set_a(0xE7);
        let load_to_memory_hl_from_a = cpu.decode_opcode(0x77);
        cpu.execute_instruction(load_to_memory_hl_from_a);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), 0xE7);
        cpu.registers.set_a(0xE3);
        let load_to_memory_hl_from_a_decrement_hl = cpu.decode_opcode(0x32);
        cpu.execute_instruction(load_to_memory_hl_from_a_decrement_hl);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), 0xE3);
        assert_eq!(cpu.registers.get_hl(), ADDRESS - 1);
        cpu.registers.set_a(0xE4);
        let load_to_memory_hl_from_a_increment_hl = cpu.decode_opcode(0x22);
        cpu.execute_instruction(load_to_memory_hl_from_a_increment_hl);
        assert_eq!(cpu.mmu.read_byte(ADDRESS - 1), 0xE4);
        assert_eq!(cpu.registers.get_hl(), ADDRESS);
        // Load(nn, A)
        cpu.registers.set_a(0xE8);
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xCD);
        cpu.mmu.write_byte(cpu.registers.get_pc() + 1, 0xAB);
        let load_to_memory_nn_from_a = cpu.decode_opcode(0xEA);
        cpu.execute_instruction(load_to_memory_nn_from_a);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), 0xE8);
        // Load(HL, B)
        cpu.registers.set_b(0xE9);
        let load_to_memory_hl_from_b = cpu.decode_opcode(0x70);
        cpu.execute_instruction(load_to_memory_hl_from_b);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), 0xE9);
        // Load((C), A)
        cpu.registers.set_a(0xEA);
        cpu.registers.set_c(0x1);
        let load_to_memory_0xff00_plus_c_from_a = cpu.decode_opcode(0xE2);
        cpu.execute_instruction(load_to_memory_0xff00_plus_c_from_a);
        assert_eq!(cpu.mmu.read_byte(0xFF00 + 0x1), 0xEA);
        // Load((n), A)
        cpu.registers.set_a(0xEF);
        let load_to_memory_0xff00_plus_n_from_a = cpu.decode_opcode(0xE0);
        cpu.execute_instruction(load_to_memory_0xff00_plus_n_from_a);
        assert_eq!(cpu.mmu.read_byte(0xFF00 + u16::from(cpu.mmu.read_byte(cpu.registers.get_pc()))), 0xEF);
    }

    #[test]
    fn cpu_load16_immediate_test() {
        let mut cpu = Cpu::new();
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xCD);
        cpu.mmu.write_byte(cpu.registers.get_pc() + 1, 0xAB);

        // Load(BC, nn)
        let load_to_bc = cpu.decode_opcode(0x01);
        cpu.execute_instruction(load_to_bc);
        assert_eq!(cpu.registers.get_bc(), 0xABCD);
        // Load(DE, nn)
        let load_to_de = cpu.decode_opcode(0x11);
        cpu.execute_instruction(load_to_de);
        assert_eq!(cpu.registers.get_de(), 0xABCD);
        // Load(HL, nn)
        let load_to_hl = cpu.decode_opcode(0x21);
        cpu.execute_instruction(load_to_hl);
        assert_eq!(cpu.registers.get_hl(), 0xABCD);
        // Load(SP, nn)
        let load_to_sp = cpu.decode_opcode(0x31);
        cpu.execute_instruction(load_to_sp);
        assert_eq!(cpu.registers.get_sp(), 0xABCD);
        // Load(SP, HL)
        cpu.registers.set_hl(0xDEAD);
        let load_to_sp_from_hl = cpu.decode_opcode(0xF9);
        cpu.execute_instruction(load_to_sp_from_hl);
        assert_eq!(cpu.registers.get_sp(), 0xDEAD);
    }

    #[test]
    fn cpu_load_stack_pointer_to_memory_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_sp(0xABCD);

        let load_sp_to_memory = cpu.decode_opcode(0x08);
        cpu.execute_instruction(load_sp_to_memory);

        assert_eq!(cpu.mmu.read_byte(cpu.registers.get_pc()), 0xCD);
        assert_eq!(cpu.mmu.read_byte(cpu.registers.get_pc()), 0xCD);
    }

    #[test]
    fn cpu_stack_push_pop_test() {
        const INITIAL_SP: u16 = 0xFFFE;
        let mut cpu = Cpu::new();
        cpu.registers.set_sp(INITIAL_SP);
        cpu.registers.set_af(0x8890);
        cpu.registers.set_bc(0xAABB);
        cpu.registers.set_de(0xCCDD);
        cpu.registers.set_hl(0xEEFF);

        let push_af_on_stack = cpu.decode_opcode(0xF5);
        let pop_stack_to_af = cpu.decode_opcode(0xF1);
        cpu.execute_instruction(push_af_on_stack);
        assert_eq!(cpu.registers.get_sp(), INITIAL_SP - 2);
        cpu.registers.set_af(0x0);
        cpu.execute_instruction(pop_stack_to_af);
        assert_eq!(cpu.registers.get_sp(), INITIAL_SP);
        assert_eq!(cpu.registers.get_af(), 0x8890);

        let push_bc_on_stack = cpu.decode_opcode(0xC5);
        let pop_stack_to_bc = cpu.decode_opcode(0xC1);
        cpu.execute_instruction(push_bc_on_stack);
        assert_eq!(cpu.registers.get_sp(), INITIAL_SP - 2);
        cpu.registers.set_bc(0x0);
        cpu.execute_instruction(pop_stack_to_bc);
        assert_eq!(cpu.registers.get_sp(), INITIAL_SP);
        assert_eq!(cpu.registers.get_bc(), 0xAABB);

        let push_de_on_stack = cpu.decode_opcode(0xD5);
        let pop_stack_to_de = cpu.decode_opcode(0xD1);
        cpu.execute_instruction(push_de_on_stack);
        assert_eq!(cpu.registers.get_sp(), INITIAL_SP - 2);
        cpu.registers.set_de(0x0);
        cpu.execute_instruction(pop_stack_to_de);
        assert_eq!(cpu.registers.get_sp(), INITIAL_SP);
        assert_eq!(cpu.registers.get_de(), 0xCCDD);

        let push_hl_on_stack = cpu.decode_opcode(0xE5);
        let pop_stack_to_hl = cpu.decode_opcode(0xE1);
        cpu.execute_instruction(push_hl_on_stack);
        assert_eq!(cpu.registers.get_sp(), INITIAL_SP - 2);
        cpu.registers.set_hl(0x0);
        cpu.execute_instruction(pop_stack_to_hl);
        assert_eq!(cpu.registers.get_sp(), INITIAL_SP);
        assert_eq!(cpu.registers.get_hl(), 0xEEFF);
    }

    #[test]
    fn cpu_add8_test() {
        // add without carry flag
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0x5);
        cpu.registers.set_f(0b1101_0000);
        let add_a = cpu.decode_opcode(0x87);
        cpu.execute_instruction(add_a);
        assert_eq!(cpu.registers.get_a(), 0xA);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.registers.set_b(0xC);
        let add_b = cpu.decode_opcode(0x80);
        cpu.execute_instruction(add_b);
        assert_eq!(cpu.registers.get_a(), 0x16);
        assert_eq!(cpu.registers.get_f(), 0b0010_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0xFF);
        cpu.registers.set_hl(ADDRESS);
        let add_from_memory_hl = cpu.decode_opcode(0x86);
        cpu.execute_instruction(add_from_memory_hl);
        assert_eq!(cpu.registers.get_a(), 0x15);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);

        // add with carry flag
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xEA);
        let add_from_memory_pc_with_carry = cpu.decode_opcode(0xCE);
        cpu.execute_instruction(add_from_memory_pc_with_carry);
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1001_0000);
    }

    #[test]
    fn cpu_add_signed_byte_to_word_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_f(0b1100_0000);

        // positive byte
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0b0100_1000);
        // load with half carry
        cpu.registers.set_sp(0b0000_1111);
        let ld_hl_sp_n = cpu.decode_opcode(0xF8);
        cpu.execute_instruction(ld_hl_sp_n);
        assert_eq!(cpu.registers.get_f(), 0b0010_0000);
        assert_eq!(cpu.registers.get_hl(), 0b0100_1000 + 0b0000_1111);
        // load with carry
        cpu.registers.set_sp(0b1111_0000);
        let ld_hl_sp_n = cpu.decode_opcode(0xF8);
        cpu.execute_instruction(ld_hl_sp_n);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);
        assert_eq!(cpu.registers.get_hl(), 0b0100_1000 + 0b1111_0000);

        // negative byte
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0b1100_1000);
        // load with half carry
        cpu.registers.set_sp(0b1000_0111);
        let ld_hl_sp_n = cpu.decode_opcode(0xF8);
        cpu.execute_instruction(ld_hl_sp_n);
        assert_eq!(cpu.registers.get_f(), 0b0010_0000);
        assert_eq!(cpu.registers.get_hl(), 0b1000_0111 - 56);
        // load with carry
        cpu.registers.set_sp(0b0000_1111);
        let ld_hl_sp_n = cpu.decode_opcode(0xF8);
        cpu.execute_instruction(ld_hl_sp_n);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0b0001_1000);
        let add_n_to_sp = cpu.decode_opcode(0xE8);
        let sp = cpu.registers.get_sp();
        let n = u16::from(cpu.mmu.read_byte(cpu.registers.get_pc()));
        cpu.execute_instruction(add_n_to_sp);
        assert_eq!(cpu.registers.get_sp(), sp + n);
    }

    #[test]
    fn cpu_sub_test() {
        // sub without carry flag
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0xFA);
        cpu.registers.set_b(0x15);
        cpu.registers.set_f(0b1000_0000);
        let sub_b = cpu.decode_opcode(0x90);
        cpu.execute_instruction(sub_b);
        assert_eq!(cpu.registers.get_a(), 0xE5);
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);

        let sub_a = cpu.decode_opcode(0x97);
        cpu.execute_instruction(sub_a);
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1100_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0xF0);
        cpu.registers.set_hl(ADDRESS);
        let sub_from_memory_hl = cpu.decode_opcode(0x96);
        cpu.execute_instruction(sub_from_memory_hl);
        assert_eq!(cpu.registers.get_a(), 0x10);
        assert_eq!(cpu.registers.get_f(), 0b0101_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xB);
        let sub_from_memory_pc = cpu.decode_opcode(0xD6);
        cpu.execute_instruction(sub_from_memory_pc);
        assert_eq!(cpu.registers.get_a(), 0x5);
        assert_eq!(cpu.registers.get_f(), 0b0110_0000);

        // sub with carry flag
        cpu.registers.set_f(0b0001_0000);
        cpu.registers.set_c(0x4);
        let sub_c_with_carry = cpu.decode_opcode(0x99);
        cpu.execute_instruction(sub_c_with_carry);
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1100_0000);
    }

    #[test]
    fn cpu_and_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0xAA);
        let and_a = cpu.decode_opcode(0xA7);
        cpu.execute_instruction(and_a);
        assert_eq!(cpu.registers.get_a(), 0xAA);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.registers.set_b(0x0);
        let and_b = cpu.decode_opcode(0xA0);
        cpu.execute_instruction(and_b);
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.registers.set_a(0xFF);
        cpu.mmu.write_byte(ADDRESS, 0xDE);
        cpu.registers.set_hl(ADDRESS);
        let and_from_memory_hl = cpu.decode_opcode(0xA6);
        cpu.execute_instruction(and_from_memory_hl);
        assert_eq!(cpu.registers.get_a(), 0xDE);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xBC);
        let and_from_memory_pc = cpu.decode_opcode(0xE6);
        cpu.execute_instruction(and_from_memory_pc);
        assert_eq!(cpu.registers.get_a(), 0x9C);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_or_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0x0);
        let or_a = cpu.decode_opcode(0xB7);
        cpu.execute_instruction(or_a);
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        cpu.registers.set_b(0x11);
        let or_b = cpu.decode_opcode(0xB0);
        cpu.execute_instruction(or_b);
        assert_eq!(cpu.registers.get_a(), 0x11);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0xAB);
        cpu.registers.set_hl(ADDRESS);
        let or_from_memory_hl = cpu.decode_opcode(0xB6);
        cpu.execute_instruction(or_from_memory_hl);
        assert_eq!(cpu.registers.get_a(), 0xBB);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xBC);
        let or_from_memory_pc = cpu.decode_opcode(0xF6);
        cpu.execute_instruction(or_from_memory_pc);
        assert_eq!(cpu.registers.get_a(), 0xBF);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_xor_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0xDE);
        let xor_a = cpu.decode_opcode(0xAF);
        cpu.execute_instruction(xor_a);
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        cpu.registers.set_b(0xDF);
        let xor_b = cpu.decode_opcode(0xA8);
        cpu.execute_instruction(xor_b);
        assert_eq!(cpu.registers.get_a(), 0xDF);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0xDF);
        cpu.registers.set_hl(ADDRESS);
        let xor_from_memory_hl = cpu.decode_opcode(0xAE);
        cpu.execute_instruction(xor_from_memory_hl);
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0x0);
        let xor_from_memory_pc = cpu.decode_opcode(0xEE);
        cpu.execute_instruction(xor_from_memory_pc);
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);
    }

    #[test]
    fn cpu_compare_test() {
        let mut cpu = Cpu::new();

        // compare with itself
        cpu.registers.set_a(0xDE);
        let compare_a = cpu.decode_opcode(0xBF);
        cpu.execute_instruction(compare_a);
        assert_eq!(cpu.registers.get_f(), 0b1100_0000);

        // compare to smaller value
        cpu.registers.set_b(0x10);
        let compare_b = cpu.decode_opcode(0xB8);
        cpu.execute_instruction(compare_b);
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);

        // compare to bigger value
        cpu.registers.set_c(0xFE);
        let compare_c = cpu.decode_opcode(0xB9);
        cpu.execute_instruction(compare_c);
        assert_eq!(cpu.registers.get_f(), 0b0101_0000);

        // check that half carry flag is set
        cpu.registers.set_a(0b1100_0000);
        cpu.registers.set_d(0b1000_1000);
        let compare_d = cpu.decode_opcode(0xBA);
        cpu.execute_instruction(compare_d);
        assert_eq!(cpu.registers.get_f(), 0b0110_0000);

        const ADDRESS: u16 = 0xABCD;
        const VALUE: u8 = 0x10;
        cpu.mmu.write_byte(ADDRESS, VALUE);
        cpu.registers.set_hl(ADDRESS);
        cpu.registers.set_a(0xAB);
        let compare_from_memory_hl = cpu.decode_opcode(0xBE);
        cpu.execute_instruction(compare_from_memory_hl);
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), VALUE);
        let compare_from_memory_pc = cpu.decode_opcode(0xFE);
        cpu.execute_instruction(compare_from_memory_pc);
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);
    }

    #[test]
    fn cpu_increment8_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_b(0xF);
        cpu.registers.set_f(0b1101_0000);
        let inc_b = cpu.decode_opcode(0x04);
        cpu.execute_instruction(inc_b);
        assert_eq!(cpu.registers.get_b(), 0x10);
        assert_eq!(cpu.registers.get_f(), 0b0011_0000);

        // check that zero flag is set
        cpu.registers.set_c(u8::max_value());
        let inc_c = cpu.decode_opcode(0x0C);
        cpu.execute_instruction(inc_c);
        assert_eq!(cpu.registers.get_c(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1011_0000);

        const ADDRESS: u16 = 0xABCD;
        const VALUE: u8 = 0x1F;
        cpu.mmu.write_byte(ADDRESS, VALUE);
        cpu.registers.set_hl(ADDRESS);
        let inc_hl = cpu.decode_opcode(0x34);
        cpu.execute_instruction(inc_hl);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), VALUE + 1);
    }

    #[test]
    fn cpu_decrement8_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0xF);
        cpu.registers.set_f(0b1011_0000);
        cpu.execute_instruction(Instruction::Dec8(IncDecTarget::A));
        assert_eq!(cpu.registers.get_a(), 0xE);
        assert_eq!(cpu.registers.get_f(), 0b0101_0000);

        // check that zero flag is set
        cpu.registers.set_b(0x1);
        let dec_b = cpu.decode_opcode(0x05);
        cpu.execute_instruction(dec_b);
        assert_eq!(cpu.registers.get_b(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1101_0000);

        // check that half carry flag is set
        cpu.registers.set_c(0b10000);
        let dec_c = cpu.decode_opcode(0x0D);
        cpu.execute_instruction(dec_c);
        assert_eq!(cpu.registers.get_c(), 0b1111);
        assert_eq!(cpu.registers.get_f(), 0b0111_0000);

        const ADDRESS: u16 = 0xABCD;
        const VALUE: u8 = 0x1F;
        cpu.mmu.write_byte(ADDRESS, VALUE);
        cpu.registers.set_hl(ADDRESS);
        let dec_hl = cpu.decode_opcode(0x35);
        cpu.execute_instruction(dec_hl);
        assert_eq!(cpu.mmu.read_byte(ADDRESS), VALUE - 1);
    }

    #[test]
    fn cpu_increment16_decrement16_test() {
        const INITIAL_REGISTER_VALUE: u16 = 0xDEAD;
        let mut cpu = Cpu::new();
        cpu.registers.set_bc(INITIAL_REGISTER_VALUE);
        cpu.registers.set_de(INITIAL_REGISTER_VALUE);
        cpu.registers.set_hl(INITIAL_REGISTER_VALUE);
        cpu.registers.set_sp(INITIAL_REGISTER_VALUE);

        let increment_bc = cpu.decode_opcode(0x03);
        cpu.execute_instruction(increment_bc);
        assert_eq!(cpu.registers.get_bc(), INITIAL_REGISTER_VALUE + 1);
        let decrement_bc = cpu.decode_opcode(0x0B);
        cpu.execute_instruction(decrement_bc);
        assert_eq!(cpu.registers.get_bc(), INITIAL_REGISTER_VALUE);

        let increment_de = cpu.decode_opcode(0x13);
        cpu.execute_instruction(increment_de);
        assert_eq!(cpu.registers.get_de(), INITIAL_REGISTER_VALUE + 1);
        let decrement_de = cpu.decode_opcode(0x1B);
        cpu.execute_instruction(decrement_de);
        assert_eq!(cpu.registers.get_de(), INITIAL_REGISTER_VALUE);

        let increment_hl = cpu.decode_opcode(0x23);
        cpu.execute_instruction(increment_hl);
        assert_eq!(cpu.registers.get_hl(), INITIAL_REGISTER_VALUE + 1);
        let decrement_hl = cpu.decode_opcode(0x2B);
        cpu.execute_instruction(decrement_hl);
        assert_eq!(cpu.registers.get_hl(), INITIAL_REGISTER_VALUE);

        let increment_sp = cpu.decode_opcode(0x33);
        cpu.execute_instruction(increment_sp);
        assert_eq!(cpu.registers.get_sp(), INITIAL_REGISTER_VALUE + 1);
        let decrement_sp = cpu.decode_opcode(0x3B);
        cpu.execute_instruction(decrement_sp);
        assert_eq!(cpu.registers.get_hl(), INITIAL_REGISTER_VALUE);
    }

    #[test]
    fn cpu_add16_test() {
        let mut cpu = Cpu::new();

        // check that substract flag is reset and half carry flag is set
        cpu.registers.set_hl(0xFFF);
        cpu.registers.set_de(0x1);
        cpu.registers.set_f(0b1100_0000);
        let add_de = cpu.decode_opcode(0x19);
        cpu.execute_instruction(add_de);
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_eq!(cpu.registers.get_f(), 0b1001_0000);

        // check that carry flag is set
        cpu.registers.set_hl(0x8888);
        let overflowed_value = 0x8888u16.wrapping_add(0x8888);
        let add_hl = cpu.decode_opcode(0x29);
        cpu.execute_instruction(add_hl);
        assert_eq!(cpu.registers.get_hl(), overflowed_value);
        assert_eq!(cpu.registers.get_f(), 0b1011_0000);
    }

    #[test]
    fn cpu_swap_test() {
        const ADDRESS: u16 = 0xABCD;
        let mut cpu = Cpu::new();

        cpu.registers.set_a(0b0);
        cpu.registers.set_b(0b1111_0000);
        cpu.registers.set_c(0b1001_0110);
        cpu.registers.set_f(0b1111_0000);
        cpu.mmu.write_byte(ADDRESS, 0b1100_0011);
        cpu.registers.set_hl(ADDRESS);

        let swap_a = cpu.decode_prefixed_opcode(0x37);
        cpu.execute_instruction(swap_a);
        assert_eq!(cpu.registers.get_a(), 0b0000_0000);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        let swap_b = cpu.decode_prefixed_opcode(0x30);
        cpu.execute_instruction(swap_b);
        assert_eq!(cpu.registers.get_b(), 0b0000_1111);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        let swap_c = cpu.decode_prefixed_opcode(0x31);
        cpu.execute_instruction(swap_c);
        assert_eq!(cpu.registers.get_c(), 0b0110_1001);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        let swap_hl  = cpu.decode_prefixed_opcode(0x36);
        cpu.execute_instruction(swap_hl);
        assert_eq!(cpu.mmu.read_byte(cpu.registers.get_hl()), 0b0011_1100);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_complement_a_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0b1001_0110);
        cpu.registers.set_f(0b1001_0000);

        let complement_a = cpu.decode_opcode(0x2F);
        cpu.execute_instruction(complement_a);

        assert_eq!(cpu.registers.get_a(), 0b0110_1001);
        assert_eq!(cpu.registers.get_f(), 0b1111_0000);
    }

    #[test]
    fn cpu_complement_carry_flag_test() {
        let mut cpu = Cpu::new();

        cpu.registers.set_f(0b1000_0000);
        let complement_carry_flag = cpu.decode_opcode(0x3F);
        cpu.execute_instruction(complement_carry_flag);
        assert_eq!(cpu.registers.get_f(), 0b1001_0000);

        cpu.registers.set_f(0b0111_0000);
        let complement_carry_flag = cpu.decode_opcode(0x3F);
        cpu.execute_instruction(complement_carry_flag);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_set_carry_flag_test() {
        let mut cpu = Cpu::new();

        cpu.registers.set_f(0b1110_0000);
        let set_carry_flag = cpu.decode_opcode(0x37);
        cpu.execute_instruction(set_carry_flag);
        assert_eq!(cpu.registers.get_f(), 0b1001_0000);

        cpu.registers.set_f(0b0001_0000);
        let set_carry_flag = cpu.decode_opcode(0x37);
        cpu.execute_instruction(set_carry_flag);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);
    }

    #[test]
    fn cpu_rlc_test() {
        let mut cpu = Cpu::new();

        cpu.registers.set_a(0b1100_0011);
        cpu.registers.set_f(0b1110_0000);
        let rlca = cpu.decode_opcode(0x07);
        cpu.execute_instruction(rlca);
        assert_eq!(cpu.registers.get_a(), 0b1000_0111);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);

        cpu.registers.set_a(0b0111_0000);
        cpu.registers.set_f(0b0001_0000);
        let rlca = cpu.decode_prefixed_opcode(0x07);
        cpu.execute_instruction(rlca);
        assert_eq!(cpu.registers.get_a(), 0b1110_0000);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.registers.set_a(0b0000_0000);
        cpu.registers.set_f(0b1111_0000);
        let rlca = cpu.decode_opcode(0x07);
        cpu.execute_instruction(rlca);
        assert_eq!(cpu.registers.get_a(), 0b0000_0000);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0b0011_1100);
        cpu.registers.set_hl(ADDRESS);
        cpu.registers.set_f(0b0001_0000);
        let rlchl = cpu.decode_prefixed_opcode(0x06);
        cpu.execute_instruction(rlchl);
        assert_eq!(cpu.mmu.read_byte(cpu.registers.get_hl()), 0b0111_1000);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_rl_test() {
        let mut cpu = Cpu::new();

        cpu.registers.set_a(0b1100_0011);
        cpu.registers.set_f(0b1110_0000);
        let rla = cpu.decode_opcode(0x17);
        cpu.execute_instruction(rla);
        assert_eq!(cpu.registers.get_a(), 0b1000_0110);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);

        cpu.registers.set_a(0b0100_0011);
        cpu.registers.set_f(0b0001_0000);
        let rla = cpu.decode_prefixed_opcode(0x17);
        cpu.execute_instruction(rla);
        assert_eq!(cpu.registers.get_a(), 0b1000_0111);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.registers.set_a(0b0000_0000);
        cpu.registers.set_f(0b1110_0000);
        let rla = cpu.decode_opcode(0x17);
        cpu.execute_instruction(rla);
        assert_eq!(cpu.registers.get_a(), 0b0000_0000);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0b0011_1100);
        cpu.registers.set_hl(ADDRESS);
        cpu.registers.set_f(0b0001_0000);
        let rlhl = cpu.decode_prefixed_opcode(0x16);
        cpu.execute_instruction(rlhl);
        assert_eq!(cpu.mmu.read_byte(cpu.registers.get_hl()), 0b011_11001);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_rrc_test() {
        let mut cpu = Cpu::new();

        cpu.registers.set_a(0b1100_0011);
        cpu.registers.set_f(0b1110_0000);
        let rrca = cpu.decode_opcode(0x0F);
        cpu.execute_instruction(rrca);
        assert_eq!(cpu.registers.get_a(), 0b1110_0001);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);

        cpu.registers.set_a(0b1100_0010);
        cpu.registers.set_f(0b0001_0000);
        let rrca = cpu.decode_prefixed_opcode(0x0F);
        cpu.execute_instruction(rrca);
        assert_eq!(cpu.registers.get_a(), 0b0110_0001);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.registers.set_a(0b0000_0000);
        cpu.registers.set_f(0b1110_0000);
        let rrca = cpu.decode_opcode(0x0F);
        cpu.execute_instruction(rrca);
        assert_eq!(cpu.registers.get_a(), 0b0000_0000);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0b0011_1100);
        cpu.registers.set_hl(ADDRESS);
        cpu.registers.set_f(0b0001_0000);
        let rrchl = cpu.decode_prefixed_opcode(0x0E);
        cpu.execute_instruction(rrchl);
        assert_eq!(cpu.mmu.read_byte(cpu.registers.get_hl()), 0b0001_1110);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_rr_test() {
        let mut cpu = Cpu::new();

        cpu.registers.set_a(0b1100_0011);
        cpu.registers.set_f(0b1110_0000);
        let rra = cpu.decode_opcode(0x1F);
        cpu.execute_instruction(rra);
        assert_eq!(cpu.registers.get_a(), 0b0110_0001);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);

        cpu.registers.set_a(0b1100_0010);
        cpu.registers.set_f(0b0001_0000);
        let rra = cpu.decode_prefixed_opcode(0x1F);
        cpu.execute_instruction(rra);
        assert_eq!(cpu.registers.get_a(), 0b1110_0001);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.registers.set_a(0b0000_0000);
        cpu.registers.set_f(0b1110_0000);
        let rra = cpu.decode_opcode(0x1F);
        cpu.execute_instruction(rra);
        assert_eq!(cpu.registers.get_a(), 0b0000_0000);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0b0011_1100);
        cpu.registers.set_hl(ADDRESS);
        cpu.registers.set_f(0b0001_0000);
        let rrchl = cpu.decode_prefixed_opcode(0x0E);
        cpu.execute_instruction(rrchl);
        assert_eq!(cpu.mmu.read_byte(cpu.registers.get_hl()), 0b0001_1110);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }
}
