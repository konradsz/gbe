pub struct Registers {
    a: u8,
    bc: u16,
    de: u16,
    hl: u16,
    f: FlagRegister,
}

#[derive(Default)]
struct FlagRegister {
    z: bool, // zero flag
    n: bool, // substract flag
    h: bool, // half carry flag
    c: bool, // carry flag
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            bc: 0,
            de: 0,
            hl: 0,
            f: Default::default(),
        }
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn get_b(&self) -> u8 {
        (self.bc >> 8) as u8
    }

    pub fn set_b(&mut self, value: u8) {
        let c = self.bc & 0x00FF;
        self.bc = u16::from(value) << 8 | c;
    }

    pub fn get_c(&self) -> u8 {
        (self.bc & 0x00FF) as u8
    }

    pub fn set_c(&mut self, value: u8) {
        let b = self.bc >> 8;
        self.bc = b << 8 | u16::from(value);
    }

    pub fn get_d(&self) -> u8 {
        (self.de >> 8) as u8
    }

    pub fn set_d(&mut self, value: u8) {
        let e = self.de & 0x00FF;
        self.de = u16::from(value) << 8 | e;
    }

    pub fn get_e(&self) -> u8 {
        (self.de & 0x00FF) as u8
    }

    pub fn set_e(&mut self, value: u8) {
        let d = self.de >> 8;
        self.de = d << 8 | u16::from(value);
    }

    pub fn get_h(&self) -> u8 {
        (self.hl >> 8) as u8
    }

    pub fn set_h(&mut self, value: u8) {
        let l = self.hl & 0x00FF;
        self.hl = u16::from(value) << 8 | l;
    }

    pub fn get_l(&self) -> u8 {
        (self.hl & 0x00FF) as u8
    }

    pub fn set_l(&mut self, value: u8) {
        let h = self.hl >> 8;
        self.hl = h << 8 | u16::from(value);
    }

    pub fn get_f(&self) -> u8 {
        0
    }

    pub fn set_f(&mut self, value: u8) {}

    pub fn get_bc(&self) -> u16 {
        self.bc
    }

    pub fn set_bc(&mut self, value: u16) {
        self.bc = value;
    }

    pub fn get_de(&self) -> u16 {
        self.de
    }

    pub fn set_de(&mut self, value: u16) {
        self.de = value;
    }

    pub fn get_hl(&self) -> u16 {
        self.hl
    }

    pub fn set_hl(&mut self, value: u16) {
        self.hl = value;
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
    fn register_f() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_f(), 0x0);
        registers.set_f(BYTE);
        assert_eq!(registers.get_f(), BYTE);
    }

    #[test]
    fn register_bc() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_bc(), 0x0);
        registers.set_bc(TWO_BYTES);
        assert_eq!(registers.get_bc(), TWO_BYTES);

        registers.set_b(0xFF);
        assert_eq!(registers.get_bc(), 0xFFAD);

        registers.set_c(0xEE);
        assert_eq!(registers.get_bc(), 0xFFEE);
    }

    #[test]
    fn register_de() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_de(), 0x0);
        registers.set_de(TWO_BYTES);
        assert_eq!(registers.get_de(), TWO_BYTES);

        registers.set_d(0xFF);
        assert_eq!(registers.get_de(), 0xFFAD);

        registers.set_e(0xEE);
        assert_eq!(registers.get_de(), 0xFFEE);
    }

    #[test]
    fn register_hl() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_hl(), 0x0);
        registers.set_hl(TWO_BYTES);
        assert_eq!(registers.get_hl(), TWO_BYTES);

        registers.set_h(0xFF);
        assert_eq!(registers.get_hl(), 0xFFAD);

        registers.set_l(0xEE);
        assert_eq!(registers.get_hl(), 0xFFEE);
    }
}
