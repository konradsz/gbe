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
            Instruction::Cp(target) => match target {
                CpTarget::A => self.compare(self.registers.get_a()),
                CpTarget::B => self.compare(self.registers.get_b()),
                CpTarget::C => self.compare(self.registers.get_c()),
                CpTarget::D => self.compare(self.registers.get_d()),
                CpTarget::E => self.compare(self.registers.get_e()),
                CpTarget::H => self.compare(self.registers.get_h()),
                CpTarget::L => self.compare(self.registers.get_l()),
                CpTarget::HL => self.compare(self.mmu.read_byte(self.registers.get_hl())),
                CpTarget::Byte => self.compare(self.mmu.read_byte(self.registers.get_pc())),
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

    fn compare(&mut self, value: u8) {
        let a_register = self.registers.get_a();
        self.registers.set_z_flag(a_register == value);
        self.registers.set_n_flag(true);
        self.registers.set_h_flag(a_register & 0xF < value & 0xF);
        self.registers.set_c_flag(a_register < value);
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
    fn cpu_add16_test() {
        let mut cpu = Cpu::new();

        // check that substract flag is reset and half carry flag is set
        cpu.registers.set_hl(0xFFF);
        cpu.registers.set_de(0x1);
        cpu.registers.set_f(0b1100_0000);
        cpu.execute_instruction(Instruction::Add16(AddSource::DE));
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_eq!(cpu.registers.get_f(), 0b1001_0000);

        cpu.registers.set_hl(0x8888);
        let overflowed_value = 0x8888u16.wrapping_add(0x8888);
        // check that carry flag is set
        cpu.execute_instruction(Instruction::Add16(AddSource::HL));
        assert_eq!(cpu.registers.get_hl(), overflowed_value);
        assert_eq!(cpu.registers.get_f(), 0b1011_0000);
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
    fn cpu_compare_test() {
        let mut cpu = Cpu::new();

        // compare with itself
        cpu.registers.set_a(0xDE);
        cpu.execute_instruction(Instruction::Cp(CpTarget::A));
        assert_eq!(cpu.registers.get_f(), 0b1100_0000);

        // compare to smaller value
        cpu.registers.set_b(0x10);
        cpu.execute_instruction(Instruction::Cp(CpTarget::B));
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);

        // compare to bigger value
        cpu.registers.set_c(0xFE);
        cpu.execute_instruction(Instruction::Cp(CpTarget::C));
        assert_eq!(cpu.registers.get_f(), 0b0101_0000);

        // check that half carry flag is set
        cpu.registers.set_a(0b1100_0000);
        cpu.registers.set_d(0b1000_1000);
        cpu.execute_instruction(Instruction::Cp(CpTarget::D));
        assert_eq!(cpu.registers.get_f(), 0b0110_0000);

        const ADDRESS: u16 = 0xABCD;
        const VALUE: u8 = 0x10;
        cpu.mmu.write_byte(ADDRESS, VALUE);
        cpu.registers.set_hl(ADDRESS);
        cpu.registers.set_a(0xAB);
        cpu.execute_instruction(Instruction::Cp(CpTarget::HL));
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);

        cpu.mmu.write_byte(0x1, VALUE);
        cpu.execute_instruction(Instruction::Cp(CpTarget::Byte));
        assert_eq!(cpu.registers.get_f(), 0b0100_0000);
    }
}
