mod cpu;
mod gpu;
mod mmu;
use self::{cpu::Cpu, gpu::Gpu, mmu::Mmu};

struct Gameboy {
    cpu: Cpu,
    gpu: Gpu,
    mmu: Mmu
}

impl Gameboy {
    fn new() -> Gameboy {
        Gameboy {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            mmu: Mmu::new(),
        }
    }

    fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.mmu);
            self.gpu.step(&mut self.mmu);
        }
    }
}

fn main() {
    let mut gameboy = Gameboy::new();
    gameboy.run();
}
