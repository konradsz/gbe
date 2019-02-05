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
        let opcode = self.mmu.read_byte(self.registers.increment_pc());

        let instruction;
        if opcode == 0xCB {
            panic!("Prefixed instructions not implemented");
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
            0x01 => Instruction::Load16(LoadRegister16::BC, self.read_word()),
            0x11 => Instruction::Load16(LoadRegister16::DE, self.read_word()),
            0x21 => Instruction::Load16(LoadRegister16::HL, self.read_word()),
            0x31 => Instruction::Load16(LoadRegister16::SP, self.read_word()),
            0xF9 => Instruction::Load16(LoadRegister16::SP, self.registers.get_hl()),
            0xF8 => Instruction::Load16(LoadRegister16::HL,
                self.add_signed_byte_to_word(self.mmu.read_byte(self.registers.get_pc()) as i8, self.registers.get_sp())),
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
            0x3C => Instruction::Inc(IncDecTarget::A),
            0x04 => Instruction::Inc(IncDecTarget::B),
            0x0C => Instruction::Inc(IncDecTarget::C),
            0x14 => Instruction::Inc(IncDecTarget::D),
            0x1C => Instruction::Inc(IncDecTarget::E),
            0x24 => Instruction::Inc(IncDecTarget::H),
            0x2C => Instruction::Inc(IncDecTarget::L),
            0x34 => Instruction::Inc(IncDecTarget::HL),
            0x3D => Instruction::Dec(IncDecTarget::A),
            0x05 => Instruction::Dec(IncDecTarget::B),
            0x0D => Instruction::Dec(IncDecTarget::C),
            0x15 => Instruction::Dec(IncDecTarget::D),
            0x1D => Instruction::Dec(IncDecTarget::E),
            0x25 => Instruction::Dec(IncDecTarget::H),
            0x2D => Instruction::Dec(IncDecTarget::L),
            0x35 => Instruction::Dec(IncDecTarget::HL),
            0x09 => Instruction::AddHL(self.registers.get_bc()),
            0x19 => Instruction::AddHL(self.registers.get_de()),
            0x29 => Instruction::AddHL(self.registers.get_hl()),
            0x39 => Instruction::AddHL(self.registers.get_sp()),
            _ => Instruction::Nop
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
                    LoadRegister16::BC => self.registers.set_bc(*value),
                    LoadRegister16::DE => self.registers.set_de(*value),
                    LoadRegister16::HL => self.registers.set_hl(*value),
                    LoadRegister16::SP => self.registers.set_sp(*value),
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
            Instruction::Inc(target) | Instruction::Dec(target) => {
                let perform_operation = |cpu: &mut Cpu, instruction: &Instruction, value| -> u8 {
                    match instruction {
                        Instruction::Inc(_) => cpu.increment(value),
                        Instruction::Dec(_) => cpu.decrement(value),
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
            Instruction::AddHL(register_value) => self.add16(*register_value),
            Instruction::Nop => (),
        }
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
    fn cpu_increment_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_b(0xF);
        cpu.registers.set_f(0b1101_0000);
        cpu.execute_instruction(Instruction::Inc(IncDecTarget::B));
        assert_eq!(cpu.registers.get_b(), 0x10);
        assert_eq!(cpu.registers.get_f(), 0b0011_0000);

        // check that zero flag is set
        cpu.registers.set_c(u8::max_value());
        cpu.execute_instruction(Instruction::Inc(IncDecTarget::C));
        assert_eq!(cpu.registers.get_c(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1011_0000);

        const ADDRESS: u16 = 0xABCD;
        const VALUE: u8 = 0x1F;
        cpu.mmu.write_byte(ADDRESS, VALUE);
        cpu.registers.set_hl(ADDRESS);
        cpu.execute_instruction(Instruction::Inc(IncDecTarget::HL));
        assert_eq!(cpu.mmu.read_byte(ADDRESS), VALUE + 1);
    }

    #[test]
    fn cpu_decrement_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0xF);
        cpu.registers.set_f(0b1011_0000);
        cpu.execute_instruction(Instruction::Dec(IncDecTarget::A));
        assert_eq!(cpu.registers.get_a(), 0xE);
        assert_eq!(cpu.registers.get_f(), 0b0101_0000);

        // check that zero flag is set
        cpu.registers.set_b(0x1);
        cpu.execute_instruction(Instruction::Dec(IncDecTarget::B));
        assert_eq!(cpu.registers.get_b(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1101_0000);

        // check that half carry flag is set
        cpu.registers.set_c(0b10000);
        cpu.execute_instruction(Instruction::Dec(IncDecTarget::C));
        assert_eq!(cpu.registers.get_c(), 0b1111);
        assert_eq!(cpu.registers.get_f(), 0b0111_0000);

        const ADDRESS: u16 = 0xABCD;
        const VALUE: u8 = 0x1F;
        cpu.mmu.write_byte(ADDRESS, VALUE);
        cpu.registers.set_hl(ADDRESS);
        cpu.execute_instruction(Instruction::Dec(IncDecTarget::HL));
        assert_eq!(cpu.mmu.read_byte(ADDRESS), VALUE - 1);
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
}
