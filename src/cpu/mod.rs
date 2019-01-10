mod instruction;
use self::instruction::*;

mod registers;
use self::registers::Registers;

pub struct Cpu {
    registers: Registers,
    pc: u16,
    sp: u16,
    memory_bus: MemoryBus,
}

struct MemoryBus {
    memory: [u8; MemoryBus::TOTAL_MEMORY_SIZE],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x0,
            memory_bus: MemoryBus::new(),
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
                    AddSource::BC => self.add16(self.registers.get_bc()),
                    AddSource::DE => self.add16(self.registers.get_de()),
                    AddSource::HL => self.add16(self.registers.get_de()),
                    AddSource::SP => self.add16(self.sp),
                };
            }
        }
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
        cpu.add16(cpu.registers.get_de());
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_eq!(cpu.registers.get_f(), 0b10010000);

        cpu.registers.set_hl(0x8888);
        let overflowed_value = 0x8888u16.wrapping_add(0x8888);
        // check that carry flag is set
        cpu.add16(cpu.registers.get_hl());
        assert_eq!(cpu.registers.get_hl(), overflowed_value);
        assert_eq!(cpu.registers.get_f(), 0b10110000);
    }
}
