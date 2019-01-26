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
            instruction = Instruction::decode_opcode(opcode);
        }
        self.execute_instruction(instruction);
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match &instruction {
            Instruction::Add8(target) => match target {
                RegistersByteTarget::A => self.add8(self.registers.get_a(), false),
                RegistersByteTarget::B => self.add8(self.registers.get_b(), false),
                RegistersByteTarget::C => self.add8(self.registers.get_c(), false),
                RegistersByteTarget::D => self.add8(self.registers.get_d(), false),
                RegistersByteTarget::E => self.add8(self.registers.get_e(), false),
                RegistersByteTarget::H => self.add8(self.registers.get_h(), false),
                RegistersByteTarget::L => self.add8(self.registers.get_l(), false),
                RegistersByteTarget::HL => self.add8(self.mmu.read_byte(self.registers.get_hl()), false),
                RegistersByteTarget::Byte => self.add8(self.mmu.read_byte(self.registers.get_pc()), false),
            },
            Instruction::Adc(target) => match target {
                RegistersByteTarget::A => self.add8(self.registers.get_a(), true),
                RegistersByteTarget::B => self.add8(self.registers.get_b(), true),
                RegistersByteTarget::C => self.add8(self.registers.get_c(), true),
                RegistersByteTarget::D => self.add8(self.registers.get_d(), true),
                RegistersByteTarget::E => self.add8(self.registers.get_e(), true),
                RegistersByteTarget::H => self.add8(self.registers.get_h(), true),
                RegistersByteTarget::L => self.add8(self.registers.get_l(), true),
                RegistersByteTarget::HL => self.add8(self.mmu.read_byte(self.registers.get_hl()), true),
                RegistersByteTarget::Byte => self.add8(self.mmu.read_byte(self.registers.get_pc()), true),
            },
            Instruction::And(target) => match target {
                RegistersByteTarget::A => self.and(self.registers.get_a()),
                RegistersByteTarget::B => self.and(self.registers.get_b()),
                RegistersByteTarget::C => self.and(self.registers.get_c()),
                RegistersByteTarget::D => self.and(self.registers.get_d()),
                RegistersByteTarget::E => self.and(self.registers.get_e()),
                RegistersByteTarget::H => self.and(self.registers.get_h()),
                RegistersByteTarget::L => self.and(self.registers.get_l()),
                RegistersByteTarget::HL => self.and(self.mmu.read_byte(self.registers.get_hl())),
                RegistersByteTarget::Byte => self.and(self.mmu.read_byte(self.registers.get_pc())),
            },
            Instruction::Or(target) => match target {
                RegistersByteTarget::A => self.or(self.registers.get_a()),
                RegistersByteTarget::B => self.or(self.registers.get_b()),
                RegistersByteTarget::C => self.or(self.registers.get_c()),
                RegistersByteTarget::D => self.or(self.registers.get_d()),
                RegistersByteTarget::E => self.or(self.registers.get_e()),
                RegistersByteTarget::H => self.or(self.registers.get_h()),
                RegistersByteTarget::L => self.or(self.registers.get_l()),
                RegistersByteTarget::HL => self.or(self.mmu.read_byte(self.registers.get_hl())),
                RegistersByteTarget::Byte => self.or(self.mmu.read_byte(self.registers.get_pc())),
            },
            Instruction::Xor(target) => match target {
                RegistersByteTarget::A => self.xor(self.registers.get_a()),
                RegistersByteTarget::B => self.xor(self.registers.get_b()),
                RegistersByteTarget::C => self.xor(self.registers.get_c()),
                RegistersByteTarget::D => self.xor(self.registers.get_d()),
                RegistersByteTarget::E => self.xor(self.registers.get_e()),
                RegistersByteTarget::H => self.xor(self.registers.get_h()),
                RegistersByteTarget::L => self.xor(self.registers.get_l()),
                RegistersByteTarget::HL => self.xor(self.mmu.read_byte(self.registers.get_hl())),
                RegistersByteTarget::Byte => self.xor(self.mmu.read_byte(self.registers.get_pc())),
            },
            Instruction::Cp(target) => match target {
                RegistersByteTarget::A => self.compare(self.registers.get_a()),
                RegistersByteTarget::B => self.compare(self.registers.get_b()),
                RegistersByteTarget::C => self.compare(self.registers.get_c()),
                RegistersByteTarget::D => self.compare(self.registers.get_d()),
                RegistersByteTarget::E => self.compare(self.registers.get_e()),
                RegistersByteTarget::H => self.compare(self.registers.get_h()),
                RegistersByteTarget::L => self.compare(self.registers.get_l()),
                RegistersByteTarget::HL => self.compare(self.mmu.read_byte(self.registers.get_hl())),
                RegistersByteTarget::Byte => self.compare(self.mmu.read_byte(self.registers.get_pc())),
            },
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
            Instruction::Add16(source) => {
                match source {
                    AddSource::BC => self.add16(self.registers.get_bc()),
                    AddSource::DE => self.add16(self.registers.get_de()),
                    AddSource::HL => self.add16(self.registers.get_hl()),
                    AddSource::SP => self.add16(self.registers.get_sp()),
                };
            }
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
        self.registers.set_n_flag(false);
        println!("current: {}, new: {}", current_value, new_value);
        self.registers.set_h_flag((((current_value & 0xf) + (new_value & 0xf)) & 0x10) == 0x10);
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
        cpu.registers.set_f(0b0101_0000);
        cpu.execute_instruction(Instruction::Add8(RegistersByteTarget::A));
        assert_eq!(cpu.registers.get_a(), 0xA);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.registers.set_b(0xC);
        cpu.execute_instruction(Instruction::Add8(RegistersByteTarget::B));

        assert_eq!(cpu.registers.get_a(), 0x16);
        assert_eq!(cpu.registers.get_f(), 0b0010_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0xFF);
        cpu.registers.set_hl(ADDRESS);
        cpu.execute_instruction(Instruction::Add8(RegistersByteTarget::HL));
        assert_eq!(cpu.registers.get_a(), 0x15);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);

        // add with carry flag
        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xEB);
        cpu.execute_instruction(Instruction::Adc(RegistersByteTarget::Byte));
        assert_eq!(cpu.registers.get_a(), 0x1);
        assert_eq!(cpu.registers.get_f(), 0b0001_0000);
    }

    #[test]
    fn cpu_and_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0xAA);
        cpu.execute_instruction(Instruction::And(RegistersByteTarget::A));
        assert_eq!(cpu.registers.get_a(), 0xAA);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.registers.set_b(0x0);
        cpu.execute_instruction(Instruction::And(RegistersByteTarget::B));
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.registers.set_a(0xFF);
        cpu.mmu.write_byte(ADDRESS, 0xDE);
        cpu.registers.set_hl(ADDRESS);
        cpu.execute_instruction(Instruction::And(RegistersByteTarget::HL));
        assert_eq!(cpu.registers.get_a(), 0xDE);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xBC);
        cpu.execute_instruction(Instruction::And(RegistersByteTarget::Byte));
        assert_eq!(cpu.registers.get_a(), 0x9C);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_or_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0x0);
        cpu.execute_instruction(Instruction::Or(RegistersByteTarget::A));
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        cpu.registers.set_b(0x11);
        cpu.execute_instruction(Instruction::Or(RegistersByteTarget::B));
        assert_eq!(cpu.registers.get_a(), 0x11);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0xAB);
        cpu.registers.set_hl(ADDRESS);
        cpu.execute_instruction(Instruction::Or(RegistersByteTarget::HL));
        assert_eq!(cpu.registers.get_a(), 0xBB);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0xBC);
        cpu.execute_instruction(Instruction::Or(RegistersByteTarget::Byte));
        assert_eq!(cpu.registers.get_a(), 0xBF);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);
    }

    #[test]
    fn cpu_xor_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_a(0xDE);
        cpu.execute_instruction(Instruction::Xor(RegistersByteTarget::A));
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        cpu.registers.set_b(0xDF);
        cpu.execute_instruction(Instruction::Xor(RegistersByteTarget::B));
        assert_eq!(cpu.registers.get_a(), 0xDF);
        assert_eq!(cpu.registers.get_f(), 0b0000_0000);

        const ADDRESS: u16 = 0xABCD;
        cpu.mmu.write_byte(ADDRESS, 0xDF);
        cpu.registers.set_hl(ADDRESS);
        cpu.execute_instruction(Instruction::Xor(RegistersByteTarget::HL));
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), 0x0);
        cpu.execute_instruction(Instruction::Xor(RegistersByteTarget::Byte));
        assert_eq!(cpu.registers.get_a(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b1000_0000);
    }

    #[test]
    fn cpu_compare_test() {
        let mut cpu = Cpu::new();

        // compare with itself
        cpu.registers.set_a(0xDE);
        cpu.execute_instruction(Instruction::Cp(RegistersByteTarget::A));
        assert_eq!(cpu.registers.get_f(), 0b1100_0000);

        // compare to smaller value
        cpu.registers.set_b(0x10);
        cpu.execute_instruction(Instruction::Cp(RegistersByteTarget::B));
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);

        // compare to bigger value
        cpu.registers.set_c(0xFE);
        cpu.execute_instruction(Instruction::Cp(RegistersByteTarget::C));
        assert_eq!(cpu.registers.get_f(), 0b0101_0000);

        // check that half carry flag is set
        cpu.registers.set_a(0b1100_0000);
        cpu.registers.set_d(0b1000_1000);
        cpu.execute_instruction(Instruction::Cp(RegistersByteTarget::D));
        assert_eq!(cpu.registers.get_f(), 0b0110_0000);

        const ADDRESS: u16 = 0xABCD;
        const VALUE: u8 = 0x10;
        cpu.mmu.write_byte(ADDRESS, VALUE);
        cpu.registers.set_hl(ADDRESS);
        cpu.registers.set_a(0xAB);
        cpu.execute_instruction(Instruction::Cp(RegistersByteTarget::HL));
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);

        cpu.mmu.write_byte(cpu.registers.get_pc(), VALUE);
        cpu.execute_instruction(Instruction::Cp(RegistersByteTarget::Byte));
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
        cpu.execute_instruction(Instruction::Add16(AddSource::DE));
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_eq!(cpu.registers.get_f(), 0b1001_0000);

        // check that carry flag is set
        cpu.registers.set_hl(0x8888);
        let overflowed_value = 0x8888u16.wrapping_add(0x8888);
        cpu.execute_instruction(Instruction::Add16(AddSource::HL));
        assert_eq!(cpu.registers.get_hl(), overflowed_value);
        assert_eq!(cpu.registers.get_f(), 0b1011_0000);
    }
}
