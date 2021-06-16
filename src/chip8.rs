#[allow(dead_code)]
pub struct Chip8 {
    opcode: u16,
    v: Vec<u8>,
    i: u16,
    pc: u16,
    stack: Vec<u16>,
    sp: u16,
    ram: Vec<u8>,
    vram: Vec<u8>,
    redraw: bool,
    keys: Vec<u8>,
    delay_t: u8,
    sound_t: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            opcode: 0,
            v: vec![0; 16],
            i: 0,
            pc: 0x200, //rom program starts at 0x200 memory address (512 in decimal)
            stack: vec![0; 16],
            sp: 0,
            ram: vec![0; 4096],
            vram: vec![0; 32 * 64],
            redraw: false,
            keys: vec![0; 16],
            delay_t: 0,
            sound_t: 0,
        }
    }

    pub fn clock(&mut self) {
        for i in 0..self.vram.len() {
            if i % 3 == 0 {
                self.vram[i] = 1;
            }
        }
        self.redraw = true;
    }

    pub fn dump_vram(&self) -> &[u8] {
        self.vram.as_slice()
    }

    pub fn is_awaiting_redraw(&self) -> bool {
        self.redraw
    }

    pub fn fulfill_redraw(&mut self) {
        self.redraw = false;
    }
}
