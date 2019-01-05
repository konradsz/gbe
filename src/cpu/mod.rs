mod registers;
use self::registers::Registers;

pub struct Cpu {
    registers: Registers,
    pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            pc: 0x0,
        }
    }
}
