pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: FlagRegister,
    pc: u16,
    sp: u16,
}

#[derive(Clone, Copy)]
struct FlagRegister {
    z: bool, // zero flag
    n: bool, // substract flag
    h: bool, // half carry flag
    c: bool, // carry flag
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            f: FlagRegister::from(0x0),
            pc: 0x0,
            sp: 0x0,
        }
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn get_f(&self) -> u8 {
        u8::from(self.f)
    }

    pub fn set_f(&mut self, value: u8) {
        self.f = FlagRegister::from(value);
    }

    pub fn get_bc(&self) -> u16 {
        u16::from(self.b) << 8 | u16::from(self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        u16::from(self.d) << 8 | u16::from(self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        u16::from(self.h) << 8 | u16::from(self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn set_z_flag(&mut self, state: bool) {
        self.f.z = state;
    }

    pub fn set_n_flag(&mut self, state: bool) {
        self.f.n = state;
    }

    pub fn set_h_flag(&mut self, state: bool) {
        self.f.h = state;
    }

    pub fn set_c_flag(&mut self, state: bool) {
        self.f.c = state;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }
}

impl FlagRegister {
    const Z_FLAG_SHIFT: u8 = 7;
    const N_FLAG_SHIFT: u8 = 6;
    const H_FLAG_SHIFT: u8 = 5;
    const C_FLAG_SHIFT: u8 = 4;
}

impl std::convert::From<FlagRegister> for u8 {
    fn from(flag: FlagRegister) -> u8 {
        u8::from(flag.z) << FlagRegister::Z_FLAG_SHIFT
            | u8::from(flag.n) << FlagRegister::N_FLAG_SHIFT
            | u8::from(flag.h) << FlagRegister::H_FLAG_SHIFT
            | u8::from(flag.c) << FlagRegister::C_FLAG_SHIFT
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(value: u8) -> FlagRegister {
        FlagRegister {
            z: value & 0b1 << FlagRegister::Z_FLAG_SHIFT != 0,
            n: value & 0b1 << FlagRegister::N_FLAG_SHIFT != 0,
            h: value & 0b1 << FlagRegister::H_FLAG_SHIFT != 0,
            c: value & 0b1 << FlagRegister::C_FLAG_SHIFT != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const BYTE: u8 = 0xDE;
    const TWO_BYTES: u16 = 0xDEAD;

    #[test]
    fn register_a() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_a(), 0x0);
        registers.set_a(BYTE);
        assert_eq!(registers.get_a(), BYTE);
    }

    #[test]
    fn register_b() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_b(), 0x0);
        registers.set_b(BYTE);
        assert_eq!(registers.get_b(), BYTE);
    }

    #[test]
    fn register_c() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_c(), 0x0);
        registers.set_c(BYTE);
        assert_eq!(registers.get_c(), BYTE);
    }

    #[test]
    fn register_d() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_d(), 0x0);
        registers.set_d(BYTE);
        assert_eq!(registers.get_d(), BYTE);
    }

    #[test]
    fn register_e() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_e(), 0x0);
        registers.set_e(BYTE);
        assert_eq!(registers.get_e(), BYTE);
    }

    #[test]
    fn register_h() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_h(), 0x0);
        registers.set_h(BYTE);
        assert_eq!(registers.get_h(), BYTE);
    }

    #[test]
    fn register_l() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_l(), 0x0);
        registers.set_l(BYTE);
        assert_eq!(registers.get_l(), BYTE);
    }

    #[test]
    fn register_bc() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_bc(), 0x0);
        registers.set_bc(TWO_BYTES);
        assert_eq!(registers.get_bc(), TWO_BYTES);

        registers.set_b(0xBE);
        assert_eq!(registers.get_bc(), 0xBEAD);

        registers.set_c(0xEF);
        assert_eq!(registers.get_bc(), 0xBEEF);
    }

    #[test]
    fn register_de() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_de(), 0x0);
        registers.set_de(TWO_BYTES);
        assert_eq!(registers.get_de(), TWO_BYTES);

        registers.set_d(0xBE);
        assert_eq!(registers.get_de(), 0xBEAD);

        registers.set_e(0xEF);
        assert_eq!(registers.get_de(), 0xBEEF);
    }

    #[test]
    fn register_hl() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_hl(), 0x0);
        registers.set_hl(TWO_BYTES);
        assert_eq!(registers.get_hl(), TWO_BYTES);

        registers.set_h(0xBE);
        assert_eq!(registers.get_hl(), 0xBEAD);

        registers.set_l(0xEF);
        assert_eq!(registers.get_hl(), 0xBEEF);
    }

    #[test]
    fn register_f() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_f(), 0b0000_0000);

        registers.set_f(0b1000_0000);
        assert_eq!(registers.get_f(), 0b1000_0000);

        registers.set_f(0b0100_0000);
        assert_eq!(registers.get_f(), 0b0100_0000);

        registers.set_f(0b0010_0000);
        assert_eq!(registers.get_f(), 0b0010_0000);

        registers.set_f(0b0001_0000);
        assert_eq!(registers.get_f(), 0b0001_0000);

        registers.set_f(0b1100_0000);
        assert_eq!(registers.get_f(), 0b1100_0000);

        registers.set_f(0b1110_0000);
        assert_eq!(registers.get_f(), 0b1110_0000);

        registers.set_f(0b1111_0000);
        assert_eq!(registers.get_f(), 0b1111_0000);

        // lower 4 bits are ignored
        registers.set_f(0b1111_1111);
        assert_eq!(registers.get_f(), 0b1111_0000);

        registers.set_z_flag(false);
        assert_eq!(registers.get_f(), 0b0111_0000);
        registers.set_n_flag(false);
        assert_eq!(registers.get_f(), 0b0011_0000);
        registers.set_h_flag(false);
        assert_eq!(registers.get_f(), 0b0001_0000);
        registers.set_c_flag(false);
        assert_eq!(registers.get_f(), 0b0000_0000);
        registers.set_z_flag(true);
        assert_eq!(registers.get_f(), 0b1000_0000);
        registers.set_n_flag(true);
        assert_eq!(registers.get_f(), 0b1100_0000);
        registers.set_h_flag(true);
        assert_eq!(registers.get_f(), 0b1110_0000);
        registers.set_c_flag(true);
        assert_eq!(registers.get_f(), 0b1111_0000);
    }

    #[test]
    fn register_pc() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_pc(), 0x0);
        registers.increment_pc();
        assert_eq!(registers.get_pc(), 0x1);
    }

    #[test]
    fn register_sp() {
        let registers = Registers::new();
        assert_eq!(registers.get_sp(), 0x0);
    }
}
