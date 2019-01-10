mod instruction;
use self::instruction::*;

mod registers;
use self::registers::Registers;

pub struct Cpu {
    registers: Registers,
    pc: u16,
    memory_bus: MemoryBus
}

struct MemoryBus {
    memory: [u8; MemoryBus::TOTAL_MEMORY_SIZE]
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            pc: 0x0,
            memory_bus: MemoryBus::new()
        }
    }

    fn step(&mut self) {
        let opcode = self.memory_bus.read_byte(self.pc);
        let instruction = Instruction::decode_opcode(opcode);
        self.execute_instruction(instruction);
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add16(source) => {
                let value = match source {
                    AddSource::BC => self.registers.get_bc()
                };

                let current_value = self.registers.get_hl();
                let (new_value, overflowed) = current_value.overflowing_add(value);
                self.registers.set_hl(new_value);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(overflowed);
                self.registers.set_c_flag((current_value & 0xFFF) + (value & 0xFFF) > 0xFFF);
            }
        }
    }
}

impl MemoryBus {
    const TOTAL_MEMORY_SIZE: usize = 0xFFFF;

    fn new() -> MemoryBus {
        MemoryBus {
            memory: [0; MemoryBus::TOTAL_MEMORY_SIZE]
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_add16_test() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        cpu.registers.set_bc(0x2345);
        cpu.registers.set_f(0b11000000);

        cpu.execute_instruction(Instruction::Add16(AddSource::BC));
        assert_eq!(cpu.registers.get_hl(), 0x1234 + 0x2345);
        assert_eq!(cpu.registers.get_f(), 0b10000000);

        // check for half carry flag
        cpu.registers.set_hl(0xFFF);
        cpu.registers.set_bc(0x1);
        cpu.execute_instruction(Instruction::Add16(AddSource::BC));
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_eq!(cpu.registers.get_f(), 0b10010000);

        // check for carry flag
        cpu.registers.set_bc(0xFFFE);
        let overflowed_value = 0x1000u16.wrapping_add(0xFFFE);
        cpu.execute_instruction(Instruction::Add16(AddSource::BC));
        assert_eq!(cpu.registers.get_hl(), overflowed_value);
        assert_eq!(cpu.registers.get_f(), 0b10100000);
    }
}
