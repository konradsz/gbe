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
        let opcode = self.mmu.read_byte(self.registers.get_pc());
        self.registers.increment_pc();

        let instruction;
        if opcode == 0xCB {
            panic!("Prefixed instructions not implemented");
        } else {
            instruction = self.decode_opcode(opcode);
        }
        self.execute_instruction(instruction);
    }

    fn decode_opcode(&self, opcode: u8) -> Instruction {
        match opcode {
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
            0x09 => Instruction::Add16(self.registers.get_bc()),
            0x19 => Instruction::Add16(self.registers.get_de()),
            0x29 => Instruction::Add16(self.registers.get_hl()),
            0x39 => Instruction::Add16(self.registers.get_sp()),
            _ => Instruction::Nop
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match &instruction {
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
            Instruction::Add16(register_value) => self.add16(*register_value),
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
