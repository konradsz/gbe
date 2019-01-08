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

enum Instruction {
    Add(AddSource, AddDestination)
}

enum AddSource {
    BC
}

enum AddDestination {
    HL
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
        // create Instruction struct that can be matched to enum (?)
        let instruction = Self::decode_opcode(opcode);
        self.execute_instruction(instruction);
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(source, destination) => {
                let value = match source {
                    AddSource::BC => self.registers.get_bc()
                };
                match destination {
                    AddDestination::HL => {
                        let current_value = self.registers.get_hl();
                        let (new_value, overflowed) = current_value.overflowing_add(value);
                        self.registers.set_hl(new_value);
                        self.registers.set_n_flag(false);
                        self.registers.set_h_flag(overflowed);
                        self.registers.set_c_flag((current_value & 0xFF + value & 0xFF) > 0xFF);
                    }
                };
            }
        }
    }

    fn decode_opcode(opcode: u8) -> Instruction {
        match opcode {
            0x09 => Instruction::Add(AddSource::BC, AddDestination::HL),
            _ => panic!("{} not implemented!", opcode)
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
    fn cpu_add_test() {
        const OLD_VALUE: u16 = 0xDEAD;
        const SOURCE_VALUE: u16 = 0x1234;
        const SUM: u16 = OLD_VALUE + SOURCE_VALUE;

        let mut cpu = Cpu::new();
        cpu.registers.set_bc(SOURCE_VALUE);
        cpu.registers.set_hl(OLD_VALUE);
        cpu.registers.set_f(0b11000000);

        cpu.execute_instruction(Instruction::Add(AddSource::BC, AddDestination::HL));
        assert_eq!(cpu.registers.get_hl(), SUM);
        assert_eq!(cpu.registers.get_f(), 0b10000000);

        let overflowed_value = SUM.wrapping_add(SOURCE_VALUE);
        cpu.execute_instruction(Instruction::Add(AddSource::BC, AddDestination::HL));
        assert_eq!(cpu.registers.get_hl(), overflowed_value);
    }
}
