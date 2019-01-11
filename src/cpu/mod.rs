mod instruction;
use self::instruction::*;

mod registers;
use self::registers::Registers;

pub struct Cpu {
    registers: Registers,
    memory_bus: MemoryBus,
}

struct MemoryBus {
    memory: [u8; MemoryBus::TOTAL_MEMORY_SIZE],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory_bus: MemoryBus::new(),
        }
    }

    fn step(&mut self) {
        let opcode = self.memory_bus.read_byte(self.registers.get_pc());
        let instruction = Instruction::decode_opcode(opcode);
        self.execute_instruction(instruction);
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Nop => (),
            Instruction::Inc(target) => {
                match target {
                    IncTarget::A => {
                        let value = self.increment(self.registers.get_a());
                        self.registers.set_a(value);
                    },
                    IncTarget::B => {
                        let value = self.increment(self.registers.get_b());
                        self.registers.set_b(value);
                    },
                    IncTarget::C => {
                        let value = self.increment(self.registers.get_c());
                        self.registers.set_c(value);
                    }
                    IncTarget::D => {
                        let value = self.increment(self.registers.get_d());
                        self.registers.set_d(value);
                    },
                    IncTarget::E => {
                        let value = self.increment(self.registers.get_e());
                        self.registers.set_e(value);
                    },
                    IncTarget::H => {
                        let value = self.increment(self.registers.get_h());
                        self.registers.set_h(value);
                    },
                    IncTarget::L => {
                        let value = self.increment(self.registers.get_l());
                        self.registers.set_l(value);
                    }
                    IncTarget::HL => {
                        let address = self.registers.get_hl();
                        let value = self.increment(self.memory_bus.read_byte(address));
                        self.memory_bus.write_byte(address, value);
                    }
                }
            },
            Instruction::Add16(source) => {
                match source {
                    AddSource::BC => self.add16(self.registers.get_bc()),
                    AddSource::DE => self.add16(self.registers.get_de()),
                    AddSource::HL => self.add16(self.registers.get_hl()),
                    AddSource::SP => self.add16(self.registers.get_sp()),
                };
            }
        }
    }

    fn increment(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registers.set_z_flag(result == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(value & 0xF == 0xF);

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

impl MemoryBus {
    const TOTAL_MEMORY_SIZE: usize = 0xFFFF;

    fn new() -> MemoryBus {
        MemoryBus {
            memory: [0; MemoryBus::TOTAL_MEMORY_SIZE],
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_add16_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0xFFF);
        cpu.registers.set_de(0x1);
        cpu.registers.set_f(0b11000000);
        // check that substract flag is reset and half carry flag is set
        cpu.execute_instruction(Instruction::Add16(AddSource::DE));
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_eq!(cpu.registers.get_f(), 0b10010000);

        cpu.registers.set_hl(0x8888);
        let overflowed_value = 0x8888u16.wrapping_add(0x8888);
        // check that carry flag is set
        cpu.execute_instruction(Instruction::Add16(AddSource::HL));
        assert_eq!(cpu.registers.get_hl(), overflowed_value);
        assert_eq!(cpu.registers.get_f(), 0b10110000);
    }

    #[test]
    fn cpu_increment_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_b(0xF);
        cpu.registers.set_f(0b11010000);
        cpu.execute_instruction(Instruction::Inc(IncTarget::B));
        assert_eq!(cpu.registers.get_b(), 0x10);
        assert_eq!(cpu.registers.get_f(), 0b00110000);

        cpu.registers.set_c(u8::max_value());
        // check that zero flag is set
        cpu.execute_instruction(Instruction::Inc(IncTarget::C));
        assert_eq!(cpu.registers.get_c(), 0x0);
        assert_eq!(cpu.registers.get_f(), 0b10110000);

        const ADDRESS: u16 = 0xABCD;
        const VALUE: u8 = 0x1F;
        cpu.memory_bus.write_byte(ADDRESS, VALUE);
        cpu.registers.set_hl(ADDRESS);
        cpu.execute_instruction(Instruction::Inc(IncTarget::HL));
        assert_eq!(cpu.memory_bus.read_byte(ADDRESS), VALUE + 1);
    }
}
